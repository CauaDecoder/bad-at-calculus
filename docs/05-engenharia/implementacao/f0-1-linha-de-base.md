# F0.1 — Linha de base do Laboratório

- Data: 2026-10-15.
- Papel: `qa-engineer`.
- Estado: **concluída como coleta de linha de base**, com falhas preexistentes registradas.
  A retomada no fim deste relatório completa os checks Rust e os seis cenários com WASM real.
  Isso não significa que todos os checks passaram ou que a comparação visual foi concluída.
- Referência: `main`, commit `1723e3a76c102494aefd23a85ca170d3abf84f7a`.
- Escopo: somente F0.1 de [F0](f0-fundacao.md). Não houve correção, renome, atualização de
  dependências, publicação ou avanço para F0.2.

## Coleta inicial — ambiente e preparação

As seções anteriores à retomada preservam os resultados da primeira coleta, sem Rust/WASM.

| Item | Ambiente observado |
| --- | --- |
| Sistema | Microsoft Windows 11 Home Single Language, `10.0.26200`, 64 bits |
| Shell | Windows PowerShell `5.1.26100.9549` |
| Node.js | `v24.16.0` |
| pnpm | `9.12.0`, invocado como `corepack pnpm` |
| Git | `2.50.0.windows.1` |
| Navegador | Chromium headless `156.0.8078.4`, Playwright `1.64.1` |
| Viewport | `1440 × 900` |
| Rust | `cargo`, `rustc` e `rustup` indisponíveis no PATH; sem instalação em `%USERPROFILE%/.cargo/bin` |
| WASM | `wasm-pack` indisponível; `apps/web/src/lib/wasm/pkg/` ausente |
| Finais de linha | `core.autocrlf=true`; arquivos amostrados com `i/lf w/crlf` |

O checkout já continha alterações documentais e arquivos não rastreados antes da execução,
incluindo os guias de implementação. Foram preservados. Portanto, o commit acima identifica
a base versionada, não um checkout limpo. Nenhum arquivo de produção ou lockfile foi alterado
por esta unidade.

`node_modules/` não existia. A preparação `corepack pnpm install --frozen-lockfile` terminou
com código 0, sem atualizar o lockfile. O pnpm direto não está no PATH; Corepack resolveu a
versão declarada no projeto. Foram gerados somente arquivos locais ignorados pelo Git.

Python também está indisponível. O helper Python de `webapp-testing` não pôde ser usado;
a inspeção foi feita com Playwright para Node.js, instalado no cache do npm, sem dependência
adicionada ao projeto. `npm.cmd` foi usado porque `npm.ps1` é bloqueado pela política de
execução do PowerShell; nenhuma política foi alterada.

## Resultados dos checks

Os comandos abaixo foram executados na raiz do repositório.

| Comando | Resultado |
| --- | --- |
| `cargo fmt --all -- --check` | Não executou o formatter: `cargo` não reconhecido; saída 1 |
| `cargo clippy --workspace --exclude math-view-desktop --all-targets -- -D warnings` | Não executou o Clippy: `cargo` não reconhecido; saída 1 |
| `cargo test --workspace --exclude math-view-desktop` | Não executou os testes Rust: `cargo` não reconhecido; saída 1 |
| `corepack pnpm wasm:build` | Saída 1: `wasm-pack` não reconhecido |
| `corepack pnpm -C apps/web check` | Saída 1: 2 erros, 0 avisos, ambos pelo módulo WASM ausente |
| `corepack pnpm -C apps/web lint` | Saída 1: Prettier apontou 116 arquivos; o encadeamento não chegou ao ESLint |
| `corepack pnpm -C apps/web test` | Saída 0: 12 arquivos, 58 testes aprovados |
| `corepack pnpm -C apps/web exec eslint .` | Diagnóstico adicional: saída 0 |
| `corepack pnpm -C apps/web build` | Saída 1: módulo WASM ausente |
| `git diff --check` | Saída 0 antes e após a escrita deste relatório |

O erro de `check` ocorre em `apps/web/src/lib/adapters/engine/wasm-transport.ts:5` e `:24`:

```text
Cannot find module '$lib/wasm/pkg/mv_wasm' or its corresponding type declarations.
```

O build falha com `[vite:load-fallback]` e `ENOENT` ao carregar o mesmo módulo.
São bloqueios do ambiente sem artefato gerado, não evidência de erro nas regras matemáticas.
Nenhum stub ou fake substituiu o WASM para produzir aprovação.

O teste inicial, executado em paralelo à preparação do SvelteKit, avisou sobre
`./.svelte-kit/tsconfig.json` ausente. A repetição após `svelte-kit sync` passou novamente:
12 arquivos, 58 testes, sem esse aviso. A suíte usa testes de domínio e aplicação, inclusive
fakes dos ports; ela **não** comprova integração com WASM ou funcionamento das telas.

### Diagnóstico separado do lint

Amostras: `apps/web/package.json`, `apps/web/src/lib/domain/calculus.ts` e
`apps/web/src/routes/calculo/derivadas/+page.svelte` têm LF no índice e CRLF no checkout.
O diagnóstico não mutante abaixo passou:

```powershell
corepack pnpm -C apps/web exec prettier --check . --end-of-line crlf --ignore-path ../../.gitignore
```

Isso localiza a diferença de formatação no checkout Windows. O comando oficial de lint
**continua reprovado**; o diagnóstico não altera a configuração nem relaxa o critério de aceite.
Uma primeira tentativa sem `--ignore-path` também incluiu 29 arquivos gerados de `.svelte-kit/`
após o build. Esses avisos de arquivos gerados não são uma nova contagem de falhas do código-fonte.
Não houve normalização dos arquivos ou correção de formatação nesta unidade.

## Laboratório no navegador

Servidor iniciado com:

```powershell
corepack pnpm -C apps/web dev --host 127.0.0.1 --port 5173 --strictPort
```

As seis rotas foram abertas em páginas novas do Chromium headless. A inspeção aguardou
`networkidle`, leu o texto renderizado, coletou erros e salvou capturas. Todas apresentaram
`500` / `Internal Error` no corpo. O documento inicial respondeu HTTP 200, mas módulos
JavaScript responderam HTTP 500: o status do documento sozinho não significa aprovação.

| Área | Rota aberta | Resultado funcional |
| --- | --- | --- |
| Limites | `/calculo/limites` | Bloqueado antes da interação |
| Derivadas | `/calculo/derivadas` | Bloqueado antes da interação |
| Integrais | `/calculo/integrais` | Bloqueado antes da interação |
| Vetores | `/vetores/operacoes` | Bloqueado antes da interação |
| Matrizes | `/vetores/matrizes` | Bloqueado antes da interação |
| Sistemas | `/vetores/sistemas` | Bloqueado antes da interação |

O console registrou falha ao importar `/.svelte-kit/generated/client/nodes/0.js`.
O servidor confirmou a causa na transformação do transporte:

```text
Failed to resolve import "$lib/wasm/pkg/mv_wasm" from "src/lib/adapters/engine/wasm-transport.ts". Does the file exist?
```

Artefatos locais auxiliares, não necessários para ler os resultados acima:

- `%LOCALAPPDATA%/Temp/opencode/f0-1-browser.cjs`: script de inspeção desta sessão.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-browser.json`: versão do navegador, rotas, erros e texto.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-{limites,derivadas,integrais,operacoes,matrizes,sistemas}.png`:
  capturas das falhas, sem material privado.

O script local usa o caminho do Playwright no cache desta máquina. Para portá-lo, ajuste
o `require` para a instalação local de `playwright`. O procedimento é abrir cada URL acima,
aguardar `networkidle`, verificar o conteúdo e os erros, e não aceitar HTTP 200 como único critério.

### Cenários pendentes para completar a linha de base

Após gerar o WASM real, repetir as seis rotas e registrar entradas, resultados e erros.
Usar estes casos autorais mínimos; são expectativas, **não resultados observados nesta sessão**:

1. Limites: `sin(x)/x`, ponto `0`; limite esperado `1`.
2. Derivadas: `x^2`, ponto `3`; inclinação esperada `6`. Mover o ponto e conferir atualização.
3. Integrais: `x`, intervalo `[0, 1]`; integral esperada `1/2`.
4. Vetores: `u=(1,0,0)`, `v=(0,1,0)`; soma `(1,1,0)` e produto escalar `0`.
5. Matrizes: `[[1,2],[3,4]]`; determinante esperado `-2`.
6. Sistemas: `x+y=3`, `x-y=1`; solução esperada `(2,1)`.

Ainda não foram verificados os cálculos dessas telas, interação com gráficos, teclado,
responsividade, temas, acessibilidade, build estático funcional ou desktop Tauri.

## Fonte visual: limitação independente

`git branch --all` mostrou apenas `main`, `origin/main`, `origin/docs/vertice-system-design`
e `origin/HEAD`. `git ls-remote --heads origin` retornou:

```text
61709c55a2715f472c804366fef73e0b3a8d40bd refs/heads/docs/vertice-system-design
1723e3a76c102494aefd23a85ca170d3abf84f7a refs/heads/main
```

`prototype/vertice-design` não está disponível nas referências locais ou nos heads remotos.
Não foram encontrados os diretórios de fonte `apps/web/src/routes/prototype/` e
`apps/web/src/lib/ui/prototype/`. Isso não prova ausência em outra máquina ou em histórico
não referenciado. Não houve troca de branch, fetch de referência inexistente ou publicação.

A fonte navegável continua **pendente de recuperação pelo mantenedor**, conforme
[design aprovado](design-aprovado.md#recuperar-a-fonte-do-protótipo). Fidelidade visual não
verificada. Essa limitação é distinta da ausência do toolchain Rust/WASM e não impediu a
coleta dos resultados disponíveis.

## Plano de retomada da coleta inicial

1. Disponibilizar Rust com `rustfmt` e `clippy`, o alvo `wasm32-unknown-unknown` e `wasm-pack`
   no ambiente de execução. Registrar as versões efetivamente usadas.
2. Repetir fmt, Clippy e testes Rust com os comandos da tabela.
3. Executar `corepack pnpm wasm:build`; depois `check`, `lint`, `test` e `build` do front.
   Registrar o lint oficial mesmo se o diagnóstico de CRLF passar.
4. Abrir as seis rotas e executar os cenários pendentes com o motor real. Anexar os resultados
   a este relatório, distinguindo falhas preexistentes de qualquer alteração futura.
5. Atualizar [progresso](progresso.md) somente com as novas evidências. F0.2 depende de F0.1
   concluída; esta sessão não libera o renome.

A recuperação do protótipo pode ocorrer separadamente. Nenhum item do roadmap foi marcado
como entregue por esta coleta parcial. Os links locais deste relatório e do registro de
execução foram conferidos e existem no checkout.

## Retomada executada — 2026-10-15

### Ambiente e escopo

Mesmo checkout e commit de referência da coleta inicial. As alterações documentais anteriores
continuam preservadas. Rust e `wasm-pack` agora estão disponíveis em `%USERPROFILE%/.cargo/bin`:

- `cargo 1.99.0 (5f94df478 2026-08-27)`.
- `rustc 1.99.0 (b940084d7 2026-09-28)`.
- `wasm-pack 0.15.0`.
- Alvos instalados: `wasm32-unknown-unknown` e `x86_64-pc-windows-msvc`.
- Node.js `v24.16.0`, pnpm `9.12.0` via Corepack.
- Chromium headless `156.0.8078.4`, Playwright para Node.js do cache anterior, viewport `1440 × 900`.
- Python continua indisponível: o executável encontrado no PATH é o alias da Microsoft Store.
  A tentativa de `with_server.py --help` não executou Python. Foi usado Playwright para Node.js.

Nenhum toolchain ou pacote foi instalado nesta retomada. O build gerou somente artefatos
ignorados pelo Git. Não houve alteração de código de produção, manifests, lockfiles ou
configuração de formatação. Não houve correção do bug encontrado nem início da F0.2.

### Checks repetidos

Comandos na raiz, com os mesmos nomes de pacote anteriores ao renome:

| Comando | Resultado na retomada |
| --- | --- |
| `cargo fmt --all -- --check` | Saída 0 |
| `cargo clippy --workspace --exclude math-view-desktop --all-targets -- -D warnings` | Saída 0 |
| `cargo test --workspace --exclude math-view-desktop` | Saída 0: 140 testes, nenhum reprovado |
| `corepack pnpm wasm:build` | Saída 0: pacote WASM real gerado |
| `corepack pnpm -C apps/web check` | Saída 0: 0 erros e 0 avisos |
| `corepack pnpm -C apps/web lint` | Saída 1: Prettier apontou 147 arquivos; ESLint não foi chamado pelo encadeamento |
| `corepack pnpm -C apps/web test` | Saída 0: 12 arquivos, 58 testes |
| `corepack pnpm -C apps/web exec eslint .` | Diagnóstico adicional: saída 0 |
| `corepack pnpm -C apps/web exec prettier --check . --end-of-line crlf --ignore-path ../../.gitignore` | Diagnóstico adicional, não mutante: saída 0 |
| `corepack pnpm -C apps/web build` | Saída 0; adapter-static gerou `apps/web/build` |

Distribuição Rust: `mv-api` 11, `mv-application` 18, `mv-domain` 92 e `mv-infra` 19.
`mv-wasm` e doc-tests não têm testes. O WASM foi verificado adicionalmente pelas telas abaixo.

A contagem oficial do lint inclui arquivos gerados de `.svelte-kit` e do pacote WASM, além
dos arquivos-fonte CRLF já registrados. Não representa 147 falhas novas do código-fonte.
`git ls-files --eol` reconfirmou `i/lf w/crlf` nas amostras. O lint oficial continua reprovado;
os diagnósticos isolados não o substituem. Encaminhar a normalização e a exclusão dos gerados
ao `frontend-developer`, sem misturar essa correção à coleta da F0.1.

O build WASM informou ausência de arquivos `LICENSE` no diretório da crate. O build estático
avisou `Overwriting build\\index.html with fallback page`. Ambos terminaram com saída 0;
nenhum pacote foi publicado.

### Cenários observados com o motor real

Os seis cenários foram executados tanto no Vite dev (`5173`) quanto no preview do build
estático (`4173`), em páginas novas. A coleta aguardou `networkidle`, leu resultados renderizados,
registrou erros JavaScript/respostas HTTP e salvou capturas. Não foram usados fakes ou stubs.

| Área e rota | Entrada e ação reproduzível | Resultado observado em dev e preview |
| --- | --- | --- |
| Limites, `/calculo/limites` | Clicar `sin(x)/x`; ponto `0` | Laterais e limite `1`; descontinuidade removível em `0` |
| Derivadas, `/calculo/derivadas` | `x^2`; definir `#x0-slider` como `3` | `f(x₀)=9`, inclinação `6`, tangente `y=6x−9` |
| Derivadas, interação | Focar o gráfico `Ponto de tangência x₀` e pressionar `ArrowRight` | Ponto `3,1`, inclinação `6,2`, tangente `y=6,2x−9,61` |
| Integrais, `/calculo/integrais` | Campo `f(x) =`: `x`; limites `0` e `1` | Integral exata `1/2`; valor numérico `0,5`; soma do ponto médio com 6 retângulos `0,5` |
| Vetores, `/vetores/operacoes` | Clicar `i × j = k`: `u=(1,0,0)`, `v=(0,1,0)` | Soma `(1,1,0)`, escalar `0`, vetorial `(0,0,1)`; **erro JavaScript, ver abaixo** |
| Matrizes, `/vetores/matrizes` | Preencher A por linha com `1,2,3,4` | `det(A)=−2`, posto `2`, matriz inversível |
| Sistemas, `/vetores/sistemas` | Clicar `SPD 2×2`: `x+y=3`, `x−y=1` | SPD e solução `(x,y)=(2,1)` |

As seis rotas responderam HTTP 200. Não houve respostas HTTP de erro durante os cenários.
Cinco rotas não emitiram `pageerror`. Vetores apresentou erro em todas as repetições, apesar
dos valores matemáticos corretos; portanto, o smoke dessa tela **não está aprovado integralmente**.

#### Falha preexistente: atualização reativa na tela de vetores

Reprodução mínima: iniciar dev ou preview, abrir `/vetores/operacoes` em contexto novo e
capturar `pageerror`. O erro já ocorre com os valores iniciais, antes de clicar em exemplos.
No dev:

```text
effect_update_depth_exceeded
Maximum update depth exceeded. This typically indicates that an effect reads and writes the same piece of state
https://svelte.dev/e/effect_update_depth_exceeded
```

O preview emite a versão reduzida: `https://svelte.dev/e/effect_update_depth_exceeded`.
A captura do dev após selecionar `i × j = k` mostra eixos, mas não os vetores esperados.
Os campos e resultados matemáticos ainda mudam. Isso não comprova funcionamento da câmera
ou do desenho 3D. O código de produção permanece igual ao commit de referência; o defeito
é preexistente, não introduzido por esta unidade.

Encaminhamento: `frontend-developer` deve diagnosticar a reatividade da visualização 3D,
incluindo `apps/web/src/lib/ui/graph3d/SpaceCanvas.svelte:53–58`, e adicionar teste de regressão
antes de corrigir. Esse trecho lê e escreve `camera`; a causa específica ainda não foi isolada.
Não foi aplicada correção no papel `qa-engineer`.

### Reproduzir o preview sem interferência do dev

Execute em sequência, sem manter Vite dev escrevendo os mesmos arquivos de `.svelte-kit`:

```powershell
corepack pnpm wasm:build
corepack pnpm -C apps/web build
corepack pnpm -C apps/web preview --host 127.0.0.1 --port 4173 --strictPort
```

Abra as rotas da tabela e repita as entradas. No teste automatizado, leia os valores de
`dd.value`, não todos os `dd`: a ajuda de sintaxe contém definições ocultas.

Uma tentativa inicial de preview após executar build e dev simultaneamente falhou antes de
renderizar `main`, com `Cannot read properties of undefined (reading 'data')`. Depois de parar
os dois servidores, repetir o build e iniciar somente preview, os seis cenários completaram.
Essa tentativa inicial não foi contada como aprovação nem como defeito confirmado do produto.

Artefatos locais desta retomada, separados dos arquivos da coleta inicial:

- `%LOCALAPPDATA%/Temp/opencode/f0-1-resume-functional.cjs`: automação das entradas, coleta e capturas.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-resume-{dev,preview}-functional.json`: resultados finais dos dois modos.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-resume-{dev,preview}-*-{result,x3}.png`: capturas dos cenários.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-resume-inspect.{cjs,json}`: inspeção inicial do DOM.
- `%LOCALAPPDATA%/Temp/opencode/f0-1-resume-build.log`: repetição isolada do build estático.

O script usa o Playwright do cache local, como na coleta anterior; exige ajuste do `require`
para outra máquina. A tabela de entradas e os comandos acima permitem reprodução manual
sem depender desses artefatos temporários. Erros registrados no JSON não devem ser ignorados
só porque o processo de coleta terminou com saída 0.

### Fonte visual, limites e encerramento

`git branch --all` e `git ls-remote --heads origin` foram repetidos. Permanecem somente as
referências da coleta inicial: `main` e `docs/vertice-system-design`; não apareceu
`prototype/vertice-design`. A recuperação pelo mantenedor e a fidelidade visual continuam
pendentes, separadas do resultado funcional. Não houve troca de branch nem publicação.

A F0.1 conclui a coleta reproduzível e o registro de falhas conhecidas, não a correção dessas
falhas. Não foram verificados desktop Tauri, arraste por mouse, câmera 3D funcional, temas,
responsividade, auditoria de acessibilidade ou comparação com o protótipo. O teste de teclado
registrado limita-se ao ponto de tangência; não comprova navegação completa por teclado.

Próxima unidade possível: F0.2, com as falhas acima conhecidas. Sua execução exige pedido
separado; não foi iniciada. Nenhuma entrega de renome, CI, design ou deploy do roadmap foi
marcada como concluída por esta coleta.
