// Compare the real web WASM binding with native dispatch on the same authored fixtures.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const root = new URL('../', import.meta.url);
const native = spawnSync('cargo', ['run', '--quiet', '-p', 'vx-api', '--example', 'assessment_fixture_reports'], {
  cwd: fileURLToPath(root),
  encoding: 'utf8',
});
if (native.error) throw native.error;
assert.equal(native.status, 0, native.stderr);
const expected = JSON.parse(native.stdout);
const cases = JSON.parse(readFileSync(new URL('crates/vx-api/tests/fixtures/assessment-cases.json', root), 'utf8'));
const binding = await import(new URL('apps/web/src/lib/wasm/pkg/vx_wasm.js', root));
await binding.default({ module_or_path: readFileSync(new URL('apps/web/src/lib/wasm/pkg/vx_wasm_bg.wasm', root)) });
const actual = cases.map(test => ({
  name: test.name,
  result: binding.dispatch('assessment.check', test.request),
}));
assert.deepEqual(actual, expected);

// Exercise the accepted boundary and oversized syntax without relying on process crashes.
for (const [response, status] of [
  ['-'.repeat(127) + '3', 'incorrect'],
  ['('.repeat(63) + '3' + ')'.repeat(63), 'correct'],
  ['sin '.repeat(42) + '3', 'incorrect'],
  ['1^'.repeat(63) + '1', 'incorrect'],
  ['('.repeat(5000), 'invalid_input'],
  ['sin '.repeat(5000), 'invalid_input'],
  ['x^'.repeat(5000), 'invalid_input'],
]) {
  const report = binding.dispatch('assessment.check', {
    exerciseId: 'guard-fixture', answer: { type: 'numeric', value: '3' }, response,
  });
  assert.equal(report.status, status);
}
console.log(`Paridade native/WASM: ${cases.length} fixtures; limites de entrada sem trap.`);
