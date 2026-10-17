# F0.3 — atualização controlada de dependências

## Resultado e escopo — 2026-10-15

Unidade concluída como atualização compatível, sem alteração das regras matemáticas ou do
contrato `dispatch`. Papéis: `backend-developer` para Cargo e Tauri; `frontend-developer`
para pacotes web, tooling e adaptações obrigatórias. As entregas estão separadas abaixo.
F0.2 foi conferida no registro de execução. F0.4 e publicação não foram iniciadas.

O checkout já continha o renome local e outras alterações não commitadas. Elas foram
preservadas; não houve reset, troca de branch, commit, publicação ou leitura de material
privado. As versões abaixo foram consultadas nos registros oficiais nesta execução,
não copiadas de pesquisa anterior.

## Linha de base

Windows x64, Rust `1.99.0`, Node.js `24.16.0`, pnpm `9.12.0` via Corepack e
`wasm-pack 0.15.0`. Chromium headless `156.0.8078.4`, Playwright do cache local,
viewport `1440 × 900`. As versões mínimas declaradas não foram testadas separadamente.

Antes das alterações, fmt, Clippy, 140 testes Rust, `check`, lint, 58 testes web e build
estático passaram. Os seis cenários da F0.1 foram repetidos no preview da linha de base:
HTTP 200, resultados esperados e somente o erro 3D já conhecido em vetores.

O audit npm inicial retornou 13 vulnerabilidades: 2 baixas, 6 moderadas, 2 altas e
3 críticas. O audit final retorna 1 baixa; não há vulnerabilidades npm moderadas,
altas ou críticas reportadas nessa consulta. Isso não é uma auditoria completa do produto.

## Entrega backend

Arquivos: `Cargo.toml`, `Cargo.lock` e `apps/desktop/src-tauri/Cargo.toml`.

1. **Motor:** `thiserror` direto passou de `1.0.69` para `2.0.21`, consolidando a versão
   2 já presente transitivamente; `serde_json` passou de `1.0.151` para `1.0.154`.
   Não foi necessário modificar código Rust. Testes, Clippy e WASM passaram antes do
   próximo grupo.
2. **Plataforma Tauri:** `tauri` passou de `2.12.0` para `2.12.4`; `tauri-build`, de
   `2.7.0` para `2.7.3`, com atualização de seus componentes internos no lockfile.
   API JS `2.12.4` e CLI `2.12.3` acompanham esse grupo. Build debug Windows, `check`
   e 58 testes web passaram antes do próximo grupo. O build release integrado passou
   na verificação final.

O desktop agora declara `rust-version = "1.90"`: Tauri `2.12.0`/`tauri-build 2.7.0`
da linha de base já exigiam esse mínimo, embora herdassem a declaração `1.85` do motor.
O mínimo declarado do motor permanece `1.85`. `nalgebra 0.33.3` foi mantida: as linhas
`0.34` e `0.35` exigem Rust `1.87` e `1.89`, respectivamente. Elevar o mínimo do motor
não foi necessário para esta entrega. `serde`, `wasm-bindgen` e `serde-wasm-bindgen`
já estavam nas versões estáveis consultadas: `1.0.229`, `0.2.129` e `0.6.5`.

## Entrega frontend

Arquivos: manifests npm da raiz/web/desktop, `pnpm-lock.yaml`,
`apps/web/src/lib/ui/copy/navigation.ts`, `apps/web/src/lib/ui/layout/ModuleNav.svelte`,
`apps/web/src/routes/{+layout,+page}.svelte` e `../setup-ambiente.md`.

| Grupo | Versões resolvidas antes e depois | Validação antes do grupo seguinte |
| --- | --- | --- |
| Atualizações compatíveis | Svelte `5.57.1` → `5.57.2`, Prettier `3.9.9` → `3.9.10`, typescript-eslint `8.71.0` → `8.71.2`, transitivas compatíveis | `check`, lint e 58 testes |
| Compilação e testes | Vite `5.4.21` → `8.3.4`, plugin Svelte `4.0.4` → `7.3.1`, Vitest `2.1.9` → `5.0.5` | `check`, lint, 58 testes e build estático |
| Renderização matemática | KaTeX `0.16.47` → `0.19.0` | `check`, lint e 58 testes; renderização conferida no smoke final |
| Análise e formatação | ESLint `9.39.5` → `10.12.0`, eslint-plugin-svelte `2.46.1` → `3.23.1`, prettier-plugin-svelte `3.5.2` → `4.1.1` | `check`, lint e 58 testes, seguidos do build e smoke integrados |

### Adaptações obrigatórias

- O workspace declara Node.js `^22.12.0 || ^24.0.0 || >=26.0.0`, compatível com Vitest 5
  e Vite 8. O setup documenta esses requisitos e instalação com lockfile congelado.
- Vite 8 usa Rolldown/Oxc e Lightning CSS. O projeto não utilizava as opções removidas
  dos guias de migração. Foram mantidos os novos padrões, sem adicionar fallback para
  ferramentas antigas. O target padrão de navegador mudou; navegadores antigos e
  outras engines não foram testados.
- Vitest 5 limpa histórico de mocks por padrão e exige aguardar assertions assíncronas.
  Os 58 testes existentes passam sem desabilitar esses padrões ou alterar expectativas.
- A recomendação nova `svelte/no-navigation-without-resolve` apontou quatro links.
  Eles passaram a usar `$app/paths.resolve`; `Topic.href` agora usa `RouteId`.
  O destaque da sidebar compara também o caminho resolvido. URLs, textos e composição
  visual permanecem iguais; não houve mudança para `/lab` nem antecipação da F0.4.
- As regras arquiteturais do ESLint permanecem intactas; nenhuma regra foi desabilitada.
- KaTeX mudou classes internas e tratamento de avisos `strict`. O único seletor próprio,
  `.katex-display`, permanece válido. O componente usa `renderToString` sem APIs internas
  ou callbacks `strict`; as fórmulas renderizadas coincidiram com a linha de base.

### Majors não incorporadas

SvelteKit continua em `2.70.3`, a última versão da linha 2 consultada, e adapter-static
em `3.0.10`, compatível com ela. Os manifests agora explicitam esses pisos já resolvidos.
SvelteKit `3.1.0` exige TypeScript `^6.0.0` e adapter-static `4.0.0` acompanha Kit 3.
Essa migração conjunta foi deliberadamente separada da atualização compatível de Kit 2.
TypeScript permanece `5.9.3`; o `latest` `7.0.2` não satisfaz o peer
`>=4.8.4 <6.1.0` de typescript-eslint `8.71.2`. Não foi forçada uma árvore incompatível.
O pnpm permanece fixado em `9.12.0`; não foi feita migração do gerenciador de pacotes.

## Verificação integrada

| Comando na raiz | Resultado |
| --- | --- |
| `cargo fmt --all -- --check` | Aprovado |
| `cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings` | Aprovado |
| `cargo test --workspace --exclude vertice-desktop --locked` | 140 testes aprovados |
| `corepack pnpm install --frozen-lockfile --strict-peer-dependencies` | Aprovado, sem resolução nova ou conflito de peers |
| `corepack pnpm wasm:build` | WASM real gerado |
| `corepack pnpm -C apps/web check` | 0 erros e 0 avisos |
| `corepack pnpm -C apps/web lint` | Aprovado |
| `corepack pnpm -C apps/web test` | 12 arquivos, 58 testes aprovados |
| `corepack pnpm -C apps/web build` | Build estático aprovado |
| `cargo build -p vertice-desktop --locked` | Debug Windows x64 aprovado |
| `corepack pnpm -C apps/desktop exec tauri build --no-bundle --ci` | Release Windows x64 aprovado, incluindo build estático |
| `corepack pnpm audit --json` | Saída 1: uma vulnerabilidade baixa restante |
| `cargo audit --version` | Indisponível: `no such command: audit` |
| `git diff --check` e conferência dos links locais dos registros | Aprovados; Git somente avisou sobre conversão futura de LF/CRLF fora de `apps/web` |

O primeiro comando release não encontrou `pnpm` no PATH do subprocesso. Foram gerados
shims com `corepack enable --install-directory` somente no diretório temporário aprovado,
adicionado ao PATH desse processo. Uma tentativa seguinte excedeu o timeout de 120 segundos;
a repetição com limite de 600 segundos terminou com saída 0. Não houve alteração do
`beforeBuildCommand` para contornar o build. Executável: `target/release/vertice-desktop.exe`.

Continuam os avisos preexistentes de licença no diretório do pacote WASM, fallback estático
em `index.html` e identificador Tauri com sufixo `.app`. Rolldown também emite diagnóstico
de tempo gasto em plugins; o build termina com saída 0.

### Laboratório com o motor real

Os cenários foram repetidos em dev e preview estático, sem fakes. No preview, asserções
compararam textos do corpo, fórmulas e valores dos campos com a coleta anterior à atualização.
Todos coincidiram; IDs aleatórios dos campos de expressão foram excluídos da comparação.
Todas as rotas responderam HTTP 200, sem respostas HTTP de erro.

| Rota | Entrada e resultado preservado |
| --- | --- |
| `/calculo/limites` | `sin(x)/x`, ponto `0`: limite `1`, descontinuidade removível |
| `/calculo/derivadas` | `x^2`, ponto `3`: valor `9`, inclinação `6`, tangente `6x−9`; seta direita atualiza para `3,1` |
| `/calculo/integrais` | `x`, de `0` a `1`: integral exata `1/2`, valor numérico `0,5` |
| `/vetores/operacoes` | `i × j = k`: soma `(1,1,0)`, escalar `0`, vetorial `(0,0,1)`; mesmo erro 3D preexistente |
| `/vetores/matrizes` | A por linhas `1,2,3,4`: determinante `−2`, posto `2`, inversível |
| `/vetores/sistemas` | `SPD 2×2`: solução `(2,1)` |

O smoke adicional verificou links dos cards, marca e sidebar, destaque da página ativa,
reload, voltar/avançar, teclado na tangente e alternância claro/escuro, sem erros JS nessas
ações. Uma tentativa inicial da automação dev expirou esperando `main` durante a inicialização;
não foi contada como aprovação. A repetição passou, assim como uma nova inicialização
com `--force` e espera explícita por resposta HTTP antes de abrir o navegador.

`effect_update_depth_exceeded` permanece reproduzível apenas no cenário de vetores,
como na linha de base. A visualização 3D não está aprovada nem foi corrigida nesta unidade.

## Vulnerabilidade restante e limites

- `cookie 0.6.0`, transitiva de SvelteKit 2: baixa, CVE-2024-47764,
  [GHSA-pxg6-pf52-xh8x](https://github.com/advisories/GHSA-pxg6-pf52-xh8x).
  Corrigida em `0.7.0`. O risco envolve nomes, paths e domains não confiáveis na
  serialização de cookies. O app é estático e não implementa escrita de cookies no código
  de produção, mas o pacote continua na árvore; o achado não foi silenciado.
  Não foi aplicado override fora da faixa declarada pelo framework. Reavaliar na migração
  conjunta de Kit/adapter/TypeScript ou em atualização oficial compatível de Kit 2.
- Sem scanner Rust instalado, vulnerabilidades e crates não mantidas no Cargo.lock
  **não foram verificadas**. A revisão/scanners da F0.6 continua obrigatória; não se declara
  o workspace Rust livre de vulnerabilidades.
- Não foram testados instaladores, assinatura, execução interativa desktop, Linux/macOS,
  Safari/Firefox, responsividade ou acessibilidade completas. Fidelidade ao protótipo
  continua não verificada, pois a fonte permanece pendente de recuperação.

## Evidência local e reversão

Artefatos em `%LOCALAPPDATA%/Temp/opencode/`:

- `f0-3-before/`: cópia prévia dos manifests e lockfiles selecionados, preservando trabalho anterior.
- `f0-3-{core,final}-*.log`, `f0-3-tauri-build.log`, `f0-3-final-desktop-release.log`:
  logs dos checks e builds. Logs `f0-3-rust-core-*` pertencem às tentativas com redirecionamento
  PowerShell; os comandos foram repetidos com `cmd /c` para preservar o código de saída nativo.
- `f0-3-npm-audit.json`: audit npm final.
- `f0-1-resume-f0-3-{before,after,dev,dev-cold}-functional.json` e capturas com os mesmos prefixos:
  cenários e erros antes/depois.
- `f0-3-verify.cjs` e `f0-3-verified.json`: comparação com asserções e smoke de navegação.

A automação reutiliza `f0-1-resume-functional.cjs` da F0.1 e o Playwright do cache local;
não constitui nova infraestrutura E2E do repositório. Para reprodução em outra máquina,
use os comandos acima e as entradas da tabela, servindo o build por `pnpm -C apps/web preview`.
Não mantenha dev e build simultâneos escrevendo `.svelte-kit`.

Não há migração de dados persistidos. Para reverter esta unidade, reverta **somente seus
hunks** nos manifests, lockfiles e quatro arquivos de navegação, reinstale com lockfile
congelado e reconstrua WASM/front/desktop. Não restaure arquivos inteiros a partir de `HEAD`
neste checkout sujo. As cópias locais ajudam a distinguir as mudanças desta execução.
Uma reversão também reintroduz os avisos de segurança anteriores.

## Fontes oficiais consultadas nesta execução

- Metadados de versões, engines e peers: <https://registry.npmjs.org/> e
  <https://crates.io/api/v1/crates>.
- [thiserror](https://github.com/dtolnay/thiserror/blob/master/README.md) e
  [changelog nalgebra](https://github.com/dimforge/nalgebra/blob/dev/CHANGELOG.md).
- [Tauri](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri/CHANGELOG.md) e
  [tauri-build](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-build/CHANGELOG.md).
- Vite: migrações [5–6](https://v6.vite.dev/guide/migration),
  [6–7](https://v7.vite.dev/guide/migration) e [7–8](https://vite.dev/guide/migration).
- [Plugin Svelte](https://github.com/sveltejs/vite-plugin-svelte/blob/main/packages/vite-plugin-svelte/CHANGELOG.md),
  [Vitest 4](https://v4.vitest.dev/guide/migration) e [Vitest 5](https://vitest.dev/guide/migration/).
- [ESLint 10](https://eslint.org/docs/latest/use/migrate-to-10.0.0),
  [eslint-plugin-svelte 3](https://sveltejs.github.io/eslint-plugin-svelte/migration/) e
  [prettier-plugin-svelte](https://github.com/sveltejs/prettier-plugin-svelte/blob/master/CHANGELOG.md).
- [KaTeX](https://github.com/KaTeX/KaTeX/blob/main/CHANGELOG.md) e advisory de `cookie` acima.
