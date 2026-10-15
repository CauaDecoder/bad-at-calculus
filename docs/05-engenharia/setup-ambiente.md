# Setup do ambiente

Guia para Windows (funciona de forma análoga em macOS/Linux).

## 1. Ferramentas

| Ferramenta | Versão   | Instalação (Windows)                                          |
| ---------- | -------- | ------------------------------------------------------------- |
| Rust       | ≥ 1.80   | `winget install Rustlang.Rustup` e depois `rustup default stable` |
| Alvo WASM  | —        | `rustup target add wasm32-unknown-unknown`                    |
| wasm-pack  | ≥ 0.13   | `cargo install wasm-pack`                                     |
| Node.js    | ≥ 20 LTS | `winget install OpenJS.NodeJS.LTS`                            |
| pnpm       | ≥ 9      | `npm install -g pnpm@9` (ou `corepack enable` em terminal de administrador) |
| Tauri      | 2.x      | Pré-requisitos: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (“Desktop development with C++”) e WebView2 (já incluso no Windows 11) |

Guia oficial do Tauri: <https://v2.tauri.app/start/prerequisites/>.

## 2. Primeira execução

```bash
pnpm install            # dependências do front e do Tauri CLI
pnpm wasm:build         # compila crates/mv-wasm para apps/web/src/lib/wasm/pkg
pnpm dev                # interface web em http://localhost:5173
```

Desktop:
```bash
pnpm -C apps/desktop icons   # gera ícones (precisa de apps/web/static/logo.png, 1024×1024)
pnpm desktop:dev
```

## 3. Verificações antes de abrir PR

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --exclude math-view-desktop
pnpm check
pnpm lint
pnpm -C apps/web test
```

## 4. Editor recomendado
VS Code com: `rust-analyzer`, `Svelte for VS Code`, `Tauri`, `EditorConfig`, `Even Better TOML`,
`Prettier`, `ESLint`.

## Problemas comuns
- **`'rustup'`/`'pnpm'` não é reconhecido** logo após instalar — o terminal não recarregou o `PATH`.
  Feche e abra o terminal (ou o VS Code).
- **"execução de scripts foi desabilitada"** ao rodar `pnpm` no PowerShell — use `pnpm.cmd`, ou
  libere scripts locais para seu usuário: `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`.
- **`link.exe not found`** — instale o C++ Build Tools (passo 1).
- **Import de `$lib/wasm/pkg` falha** — rode `pnpm wasm:build` antes do `pnpm dev`.
- **Tauri reclama de ícones** — rode `pnpm -C apps/desktop icons`.
