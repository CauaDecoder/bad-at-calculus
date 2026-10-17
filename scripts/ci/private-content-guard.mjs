import { execFileSync } from 'node:child_process';
import { pathToFileURL } from 'node:url';

function git(args) {
  return execFileSync('git', args, { encoding: 'utf8', stdio: 'pipe', maxBuffer: 32 * 1024 * 1024 });
}

export function isPrivatePath(path) {
  return /(^|\/)content-private(\/|$)/i.test(path) || /\.pdf$/i.test(path);
}

export function checkPrivateContent({ base, head }) {
  if (!/^[a-f\d]{40}$/i.test(head ?? '')) throw new Error('HEAD_SHA deve ser um SHA completo.');
  if (base && !/^[a-f\d]{40}$/i.test(base)) throw new Error('BASE_SHA deve ser um SHA completo.');

  // Check the full tree too: a weekly scan must not silently skip existing private files.
  const paths = git(['ls-tree', '-r', '--name-only', '-z', head]).split('\0');
  const range = base && !/^0+$/.test(base) ? `${base}..${head}` : head;
  const commits = git(['rev-list', range]).trim().split('\n').filter(Boolean);
  for (const commit of commits) {
    // No rename detection: a rename must check its destination, even with unusual filenames.
    paths.push(...git([
      'diff-tree', '--root', '--no-commit-id', '--name-only', '--no-renames',
      '--diff-filter=ACMT', '-r', '-m', '-z', commit,
    ]).split('\0'));
  }
  if (paths.some(isPrivatePath)) {
    // Never echo private filenames or file contents into public CI logs.
    throw new Error('Conteúdo privado ou PDF detectado na árvore ou nos commits. Remova-o do histórico antes de publicar.');
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    checkPrivateContent({ base: process.env.BASE_SHA, head: process.env.HEAD_SHA });
    console.log('Guard de conteúdo privado aprovado.');
  } catch (error) {
    // Git errors can contain paths; do not forward their output to public logs.
    console.error(error.status !== undefined ? 'Não foi possível conferir o histórico Git.' : error.message);
    process.exitCode = 1;
  }
}
