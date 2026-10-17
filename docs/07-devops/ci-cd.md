# CI/CD

Configuração local da **F0.6** em `.github/workflows/`. Execução em PR ainda precisa ser
comprovada; veja o [relatório da unidade](../05-engenharia/implementacao/f0-6-ci-seguranca.md).
Actions externas estão fixadas por SHA, com comentário da versão. `renovate.json` prepara
atualizações dessas Actions sem automerge; instalar o GitHub App continua a cargo do mantenedor.

## Workflows

| Workflow | Gatilho | Jobs |
| -------- | ------- | ---- |
| `ci.yml` | `pull_request`, `push: main` | `rust` · `web` · `wasm` · `workflow-checks` |
| `security.yml` | `pull_request`, `push: main`, semanal | `deps` · `secrets` · `sast` · `private-content-guard` · `scorecard` (semanal) |
| `preview.yml` (F0.7, pendente) | `pull_request` (mesmo repo) | build estático + deploy de preview no Cloudflare + comentário com a URL |
| `deploy.yml` (F0.7, pendente) | `push: main` | build + deploy de produção (ambiente `production` com aprovação) |
| `release.yml` (pendente) | release-please | tag, CHANGELOG, builds Tauri em matriz Win/macOS/Linux, SBOM, assinatura, attestations |
| `content.yml` (F1) | mudanças em `content/**` | compila `*.ex.md`, valida schema, licença e auto-correção das respostas |

## Detalhe dos jobs

```yaml
# rust
- cargo fmt --all -- --check
- cargo clippy --workspace --exclude vertice-desktop --all-targets --locked -- -D warnings
- cargo test --workspace --exclude vertice-desktop --locked
# web
- pnpm install --frozen-lockfile
- pnpm wasm:build -- --locked                 # binding real exigido pelo check
- pnpm -C apps/web check
- pnpm -C apps/web lint
- pnpm -C apps/web test
# wasm
- pnpm wasm:build -- --locked
- pnpm -C apps/web build                      # garante que o bundle estático sai
# workflow-checks
- actionlint
- node --test scripts/ci/private-content-guard.test.mjs
# e2e (Playwright, F1)
- fluxos: digitar x^2 e arrastar o ponto; resolver um exercício; importar um pack de exemplo
```

```yaml
# security
deps:     cargo deny --locked check · cargo audit · osv-scanner scan source --recursive . · pnpm audit --prod --audit-level high
secrets:  gitleaks git --redact --log-opts="--all" .
sast:     CodeQL (javascript-typescript, rust; security-extended; build-mode none) · Semgrep CE com .semgrep.yml
private-content-guard: rejeita *.pdf e caminhos content-private/ na árvore e nos commits novos
scorecard: ossf/scorecard-action (publica no Security tab)
```

## Regras
- PRs de forks rodam `ci.yml` e `security.yml` **sem segredos do repositório**, sem ambientes e
  sem deploy. Não há `pull_request_target`. Checkout não persiste credenciais. O token automático
  recebe apenas `contents: read`, exceto o job CodeQL, que declara também `security-events: write`
  para SARIF. GitHub restringe esse token em forks; CodeQL possui o fluxo próprio para esse evento.
- Scorecard roda apenas no agendamento da branch padrão, sem executar scripts do projeto.
  Só ele recebe `id-token: write`; Scorecard e CodeQL recebem `security-events: write`.
- Objetivo de proteção da `main`: checks obrigatórios, uma revisão humana e proibição de
  *force push*. Em 2026-10-15, `protected: false`; criar workflows não configura essa proteção.
- Cache: `Swatinem/rust-cache` (gravação só em push de `main`) e pnpm store pelo `setup-node`.
  Nenhum upload de bundle, conteúdo ou logs de scanners em artefatos públicos nesta unidade.
- Rust 1.99.0, Node 24.16.0 e wasm-pack 0.15.0 reproduzem o ambiente local verificado. pnpm usa
  `packageManager` da raiz. Isso não altera o MSRV 1.85 declarado para o motor.
- `cargo-audit`, `cargo-deny`, OSV, Gitleaks e Semgrep estão fixados por versão. Binários baixados
  diretamente (OSV, Gitleaks e actionlint) exigem SHA-256 literal antes de executar. Essas versões
  e checksums precisam de revisão manual; Renovate está limitado às Actions.
- `deny.toml` não ignora advisories e confere também dependências desktop. Licenças e fontes
  desconhecidas bloqueiam; duplicatas e versões wildcard geram aviso (as crates locais usam
  `path` sem versão de registry). A allowlist não substitui revisão jurídica.
- Cargo-deny e OSV, segredos e regras Semgrep bloqueiam seus checks; npm bloqueia alta/crítica.
  `cargo audit` usa o comportamento padrão: advisories informativos e versões yanked geram
  avisos, não falha; cargo-deny/OSV oferecem a checagem complementar bloqueante.
  CodeQL e Scorecard publicam alertas, mas exigem regras do repositório para bloquear merge por
  severidade. Automatizar issues para achados médios permanece pendente, sem conceder escrita
  de issues a código de PR. Até lá, o mantenedor deve fazer a triagem manual.
- O guard usa SHAs do evento via ambiente, não expressões interpoladas em shell. Trata nomes
  com espaços/quebras de linha, maiúsculas, renomes e adição seguida de remoção. Falha se faltar
  histórico e não imprime nomes nem conteúdo privados. Detectar depois de um push **não desfaz
  um vazamento**: rode a verificação local antes de publicar.
- E2E de exercícios, compilação de conteúdo, previews, produção e release não são chamados na F0.6.
  Não há CLI `vertice content build` disponível nesta fase.

## Configuração externa e custo

O mantenedor deve autorizar a publicação do checkout e abrir um PR; executar e registrar os
checks de PR (incluindo um fork sem segredos), CodeQL e o agendamento Scorecard; ativar reporte
privado e proteção da `main`; instalar Renovate. Não é necessário PAT, token Cloudflare nem
serviço pago para os checks desta unidade.

O repositório foi confirmado público pela API. Segundo a
[documentação oficial de cobrança do GitHub Actions](https://docs.github.com/en/billing/concepts/product-billing/github-actions),
consultada em 2026-10-15, runners padrão em repositórios públicos são gratuitos. Os workflows
usam apenas `ubuntu-24.04`, sem runners maiores ou self-hosted, sem armazenamento de artefatos
do bundle e sem serviços externos pagos. O cache tem franquia separada de 10 GB/repositório;
não aumentar o limite faturável. Orçamento de US$ 0 e uso compartilhado da conta exigem
conferência do mantenedor; nenhum ajuste de cobrança foi realizado.

## Dev local (pre-commit opcional)
`lefthook` ou `pre-commit` com: `cargo fmt`, `prettier`, `gitleaks protect --staged`.
