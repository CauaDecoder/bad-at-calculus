import svelte from 'eslint-plugin-svelte';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  ...tseslint.configs.strict,
  ...svelte.configs['flat/recommended'],
  {
    files: ['**/*.svelte', '**/*.svelte.ts'],
    languageOptions: { parserOptions: { parser: tseslint.parser } },
  },
  {
    // Enforce the front-end dependency rule (docs/03-arquitetura/frontend.md).
    files: ['src/lib/ui/**', 'src/lib/application/**', 'src/lib/domain/**'],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            { group: ['@tauri-apps/*'], message: 'Only $lib/adapters may import Tauri.' },
            { group: ['$lib/wasm/*'], message: 'Only $lib/adapters may import the WASM engine.' },
            { group: ['$lib/adapters/*'], message: 'Depend on $lib/ports, not on adapters.' },
          ],
        },
      ],
    },
  },
  { ignores: ['.svelte-kit/', 'build/', 'src/lib/wasm/pkg/'] },
);
