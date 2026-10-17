import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { isPrivatePath } from './private-content-guard.mjs';

const guard = fileURLToPath(new URL('./private-content-guard.mjs', import.meta.url));

function repository(t) {
  const cwd = mkdtempSync(join(tmpdir(), 'vertice-guard-'));
  t.after(() => rmSync(cwd, { recursive: true, force: true }));
  const git = (...args) => execFileSync('git', args, { cwd, encoding: 'utf8', stdio: 'pipe' }).trim();
  git('init');
  git('config', 'user.email', 'ci@example.invalid');
  git('config', 'user.name', 'CI fixture');
  const commit = () => {
    git('add', '-A');
    git('commit', '--allow-empty', '-m', 'test fixture');
    return git('rev-parse', 'HEAD');
  };
  const base = commit();
  const write = (path) => {
    mkdirSync(dirname(join(cwd, path)), { recursive: true });
    writeFileSync(join(cwd, path), 'Synthetic fixture. No third-party content.');
  };
  const check = (start = base) => execFileSync(process.execPath, [guard], {
    cwd, encoding: 'utf8', stdio: 'pipe',
    env: { ...process.env, BASE_SHA: start, HEAD_SHA: git('rev-parse', 'HEAD') },
  });
  return { cwd, git, base, commit, write, check };
}

test('path policy handles case, nested directories and unrelated names', () => {
  for (const path of ['a.pdf', 'A.PDF', 'content-private/a.ex.md', 'nested/Content-Private/a']) {
    assert.equal(isPrivatePath(path), true);
  }
  for (const path of ['content/a.ex.md', 'docs/pdf.md', 'content-private-notes.md']) {
    assert.equal(isPrivatePath(path), false);
  }
});

test('allows authored Markdown and an empty range', (t) => {
  const repo = repository(t);
  repo.check();
  repo.write('content/author.ex.md');
  repo.commit();
  repo.check();
});

for (const path of ['notes with spaces.PDF', 'nested/Content-Private/file.ex.md', 'line\nbreak.pdf']) {
  test(`rejects a forbidden path (${JSON.stringify(path)}) without logging it`, (t) => {
    if (process.platform === 'win32' && path.includes('\n')) return t.skip('Windows forbids newlines in filenames.');
    const repo = repository(t);
    repo.write(path);
    repo.commit();
    assert.throws(repo.check, (error) => {
      assert.equal(error.status, 1);
      assert.match(error.stderr, /Conteúdo privado ou PDF detectado/);
      assert.equal(error.stderr.includes(path), false);
      return true;
    });
  });
}

test('rejects add-then-delete in intermediate commits', (t) => {
  const repo = repository(t);
  repo.write('transient.pdf');
  repo.commit();
  repo.git('rm', 'transient.pdf');
  repo.commit();
  assert.throws(repo.check);
});

test('rejects rename to a private destination', (t) => {
  const repo = repository(t);
  repo.write('notes.md');
  const base = repo.commit();
  repo.git('mv', 'notes.md', 'notes.pdf');
  repo.commit();
  assert.throws(() => repo.check(base));
});

test('checks an existing tree even when no commits changed', (t) => {
  const repo = repository(t);
  repo.write('existing.pdf');
  const head = repo.commit();
  assert.throws(() => repo.check(head));
});

test('checks all history when the push has no previous SHA', (t) => {
  const repo = repository(t);
  repo.write('transient.pdf');
  repo.commit();
  repo.git('rm', 'transient.pdf');
  repo.commit();
  assert.throws(() => repo.check('0'.repeat(40)));
});

test('fails closed on an unavailable base commit', (t) => {
  const repo = repository(t);
  assert.throws(() => repo.check('a'.repeat(40)));
});
