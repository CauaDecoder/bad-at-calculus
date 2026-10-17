# F0.2 — Renome técnico local

Data: 2026-10-15. Unidade: [F0.2](f0-fundacao.md#f02-executar-o-renome-técnico-isolado).
Papéis: `backend-developer`, com entrega mecânica de bindings e pacote web encaminhada ao
`frontend-developer`. Dependência: [F0.1](f0-1-linha-de-base.md#retomada-executada--2026-10-15),
concluída como coleta com falhas conhecidas. Estado inicial: **parcial**, pois o primeiro build
desktop não passou. A retomada autorizada está registrada ao fim deste relatório.

Estado atual: **concluída como renome técnico local**, após os builds desktop debug e release
Windows aprovados na retomada. Publicação externa e renome do GitHub permanecem separados.

## Escopo aplicado

- Diretórios e pacotes Rust: `crates/mv-{domain,application,infra,api,wasm}` passam a
  `crates/vx-{domain,application,infra,api,wasm}`. Imports e referências técnicas acompanham o
  mapeamento; `cargo fmt --all` reordena os imports após a troca de prefixo.
- Pacote Cargo desktop: `math-view-desktop` passa a `vertice-desktop`.
- Pacotes pnpm privados: raiz `math-view` passa a `vertice`; `@math-view/web` e
  `@math-view/desktop` passam a `@vertice/web` e `@vertice/desktop`.
- Script WASM e transporte web usam `vx-wasm` e `vx_wasm`. Comandos JSON, DTOs e regras
  matemáticas permanecem iguais. A chave local de preferências `math-view:` foi preservada,
  evitando perda de tema salvo; renomeá-la exigiria uma migração separada.
- Tauri usa produto e título `Vértice`, com identificador local `org.vertice.app`.
  Esse identificador e o escopo `@vertice` precisam de confirmação do mantenedor antes de
  qualquer publicação externa. A mudança de identificador pode separar os dados de uma instalação
  desktop antiga; migração de instalações existentes não foi testada.
- Manifests, ignore, documentação operacional, mapa de camadas e instruções de agentes usam os
  nomes novos. ADRs históricos e relatório F0.1 foram preservados.
- O campo `workspace.package.repository` deixou de conter o placeholder
  `https://github.com/<owner>/math-view` e aponta para o repositório atual,
  `https://github.com/CauaDecoder/bad-at-calculus`. O remoto e o repositório GitHub não foram renomeados.

Nenhuma dependência foi atualizada. `Cargo.lock` foi regenerado pelo Cargo somente para os
nomes internos e sua ordenação. `pnpm-lock.yaml` permanece idêntico. Alterações preexistentes,
incluindo normalização LF, correção do lint e documentação de produto, foram preservadas.
Branding e navegação da UI web permanecem para F0.4; não houve redesenho de telas.

## Ambiente e linha de base

Windows, alvo `x86_64-pc-windows-msvc`; Cargo `1.99.0`, `wasm-pack 0.15.0`, Node.js `v24.16.0`,
pnpm `9.12.0` via Corepack. O alvo `wasm32-unknown-unknown` está instalado.

Antes do renome, passaram `cargo fmt --all -- --check`, Clippy com `-D warnings`, os 140 testes
Rust (excluindo `math-view-desktop`), `check` com zero erros e avisos, lint oficial e os 58 testes
do front. A falha 3D `effect_update_depth_exceeded` da F0.1 continua fora do escopo.

## Checks após o renome

| Comando na raiz | Resultado |
| --- | --- |
| `cargo fmt --all -- --check` | Saída 0 após reordenação dos imports pelo rustfmt |
| `cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings` | Saída 0 |
| `cargo test --workspace --exclude vertice-desktop --locked` | Saída 0: 140 testes aprovados |
| `corepack pnpm wasm:build` | Saída 0: `vx_wasm.js` e `vx_wasm_bg.wasm` gerados |
| `corepack pnpm -C apps/web check` | Saída 0: zero erros e avisos |
| `corepack pnpm -C apps/web lint` | Saída 0: Prettier e ESLint aprovados |
| `corepack pnpm -C apps/web test` | Saída 0: 12 arquivos e 58 testes aprovados |
| `corepack pnpm -C apps/web build` | Saída 0: build estático com `vx_wasm_bg` |
| `cargo build -p vertice-desktop --locked` | Saída 1: ícone Windows ausente, detalhado abaixo |

Clippy e testes também passaram com `--target-dir target/rename-check`, evitando disputa pelo
diretório de build do desktop. As primeiras tentativas paralelas no diretório padrão atingiram
o timeout de 120 segundos enquanto o desktop compilava; os resultados acima são das repetições
concluídas, não dessas tentativas interrompidas. O primeiro `fmt --check` detectou apenas a
reordenação necessária dos imports após o renome; `cargo fmt --all` corrigiu essa diferença.

Avisos não bloqueantes já observados na F0.1: WASM sem cópia dos arquivos `LICENSE` no diretório
da crate e adapter-static sobrescrevendo `build/index.html` com fallback. Nenhum check foi
relaxado. A conferência dos links Markdown verificou 132 destinos locais nos documentos
operacionais alterados; `git diff --check` passou.

## Evidência estrutural

- `cargo metadata --no-deps --format-version 1 --locked` lista `vx-domain`, `vx-application`,
  `vx-infra`, `vx-api`, `vx-wasm` e `vertice-desktop`, com dependências internas nos nomes novos.
- `corepack pnpm install --frozen-lockfile` passou sem alterar o lockfile; `pnpm -r list --depth -1`
  identifica os três pacotes privados pelos nomes novos.
- Comparação local dos 57 arquivos das crates com `HEAD`, normalizando apenas nomes, branding
  técnico, finais de linha e ordenação das linhas, confirmou ausência de mudanças adicionais.
- Comparação dos blocos de `Cargo.lock`, normalizando nomes e ordenação, confirmou preservação
  de versões, dependências externas e checksums. `git diff -- docs/03-arquitetura/adr` está vazio.
- Smoke direto do novo `vx_wasm.js` com seu `vx_wasm_bg.wasm` real, em Node.js: limite de
  `sin(x)/x` em `0` igual a `1`; inclinação de `x^2` em `3` igual a `6`; integral de `x` em
  `[0,1]` igual a `0.5`; soma de `i` e `j` igual a `[1,1,0]` e escalar `0`; determinante de
  `[[1,2],[3,4]]` igual a `-2`; solução de `x+y=3`, `x-y=1` igual a `[2,1]`.
  Comando desconhecido continua retornando `unknown-command`. Comparações numéricas usaram
  tolerância `1e-6`. Isso verifica o binding real, não interação no navegador.

## Bloqueio desktop e limites

`cargo build -p vertice-desktop --locked` compilou as dependências, mas terminou com saída 1
no `tauri-build`:

```text
icons/icon.ico not found; required for generating a Windows Resource file during tauri-build
```

A configuração anterior já declarava `icons/icon.png` e `icons/icon.ico`; o diretório não
existia antes do renome e `git ls-files apps/desktop/src-tauri/icons` não lista arquivos.
A ausência é preexistente, embora o build desktop não tenha sido executado na F0.1.
Não foram criados ícones nem removida a exigência para contornar o erro.

Não verificados: executável desktop, instaladores, smoke desktop, Linux, macOS, migração de
dados desktop e interação no navegador nesta unidade. Nenhum pacote ou release foi publicado.
Recuperação do protótipo e correção do bug 3D continuam separadas.

## Retomada

1. Encaminhar ao `frontend-developer` os ícones aprovados, ou obter uma decisão explícita sobre
   os assets desktop; não tratar um ícone provisório como identidade pública aprovada.
2. Repetir `cargo build -p vertice-desktop --locked` e o build Tauri no ambiente Windows com os
   assets disponíveis; registrar qualquer bloqueio seguinte e os alvos efetivamente testados.
3. Confirmar identificador Tauri e escopo npm com o mantenedor antes de publicar; renome do
   repositório GitHub permanece uma operação externa separada.
4. Encerrar F0.2 somente após cumprir seus critérios. F0.3 não foi iniciada nesta execução.

## Retomada com ícone provisório — 2026-10-15

O mantenedor autorizou explicitamente criar uma imagem temporária e repetir o build.
O `frontend-developer` criou um ícone autoral simples, com fundo neutro e a letra V, em
`apps/desktop/src-tauri/icons/icon.svg`. A CLI Tauri existente gerou `icon.png` RGBA de
512 × 512 e `icon.ico`; somente esses dois binários, a fonte SVG e o README de reprodução
foram mantidos. Não houve alteração de dependências ou da identidade visual da UI.
O ícone não representa aprovação da identidade definitiva.

`cargo build -p vertice-desktop --locked` passou, com saída 0, no alvo
`x86_64-pc-windows-msvc`; o executável debug foi compilado. O bloqueio por ícone ausente foi
resolvido sem remover a configuração ou relaxar checks.

Foi iniciado também `corepack pnpm -C apps/desktop exec tauri build --no-bundle --ci` para
validar o executável release com o `beforeBuildCommand` real. No ambiente sem `pnpm` no PATH,
foram criados shims Corepack somente no diretório temporário aprovado e acrescentados ao PATH
do processo, sem mudar a configuração do projeto. O processo foi cancelado pela reinicialização
do servidor do agente; o build release não foi confirmado e precisa ser repetido.
Instaladores, assinatura, execução interativa e outros sistemas operacionais não foram testados.

### Repetição concluída após a interrupção

`corepack pnpm -C apps/desktop exec tauri build --no-bundle --ci` foi repetido e terminou com
saída 0. O `beforeBuildCommand` reconstruiu o front estático e o Cargo concluiu o perfil
`release` otimizado. Executável gerado: `target/release/vertice-desktop.exe`, Windows x64.
O build não gerou instaladores, não assinou nem publicou releases.

A CLI informou um aviso não bloqueante: `org.vertice.app` termina em `.app`, sufixo não
recomendado por conflitar com a extensão de bundles macOS. O identificador continua sujeito
à confirmação antes da publicação; não foi alterado nesta repetição. macOS e Linux continuam
não testados, assim como execução interativa do executável.

Com os checks anteriores e o build desktop disponível aprovados, F0.2 local está concluída.
F0.3 pode ser solicitada; não foi iniciada.
