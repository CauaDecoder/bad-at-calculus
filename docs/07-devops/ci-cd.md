# CI/CD

Implementação na **Fase 0** (`.github/workflows/`). Todos os jobs com `permissions:` mínimas e
Actions fixadas por SHA (com comentário da versão), atualizadas pelo Renovate.

## Workflows

| Workflow | Gatilho | Jobs |
| -------- | ------- | ---- |
| `ci.yml` | `pull_request`, `push: main` | `rust` · `web` · `wasm` · `e2e` |
| `security.yml` | `pull_request`, `push: main`, semanal | `deps` · `secrets` · `sast` · `private-content-guard` · `scorecard` (semanal) |
| `preview.yml` | `pull_request` (mesmo repo) | build estático + deploy de preview no Cloudflare + comentário com a URL |
| `deploy.yml` | `push: main` | build + deploy de produção (ambiente `production` com aprovação) |
| `release.yml` | release-please | tag, CHANGELOG, builds Tauri em matriz Win/macOS/Linux, SBOM, assinatura, attestations |
| `content.yml` (F1) | mudanças em `content/**` | compila `*.ex.md`, valida schema, licença e auto-correção das respostas |

## Detalhe dos jobs

```yaml
# rust
- cargo fmt --all -- --check
- cargo clippy --workspace --exclude math-view-desktop --all-targets -- -D warnings
- cargo test --workspace --exclude math-view-desktop
# web
- pnpm install --frozen-lockfile
- pnpm -C apps/web check && pnpm -C apps/web lint && pnpm -C apps/web test
# wasm
- pnpm wasm:build && pnpm -C apps/web build      # garante que o bundle estático sai
# e2e (Playwright, F1)
- fluxos: digitar x^2 e arrastar o ponto; resolver um exercício; importar um pack de exemplo
```

```yaml
# security
deps:     cargo-deny check · cargo-audit · osv-scanner --recursive . · pnpm audit --prod
secrets:  gitleaks detect --redact
sast:     CodeQL (javascript-typescript, rust) · semgrep ci (regras p/ TS, Rust, Svelte)
private-content-guard: falha se o diff tiver *.pdf ou caminhos em content-private/
scorecard: ossf/scorecard-action (publica no Security tab)
```

## Regras
- PRs de forks rodam `ci.yml` e `security.yml` **sem segredos**; o preview só roda para branches do
  próprio repositório.
- Branch `main` protegida: CI verde, 1 revisão (pode ser o agente `qa-engineer` + humano), sem
  *force push*.
- Cache: `Swatinem/rust-cache` e cache do pnpm store.
- Falhas de segurança de severidade alta bloqueiam o merge; médias abrem issue automática.

## Dev local (pre-commit opcional)
`lefthook` ou `pre-commit` com: `cargo fmt`, `prettier`, `gitleaks protect --staged`.
