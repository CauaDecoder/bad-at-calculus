# Registro de execução das unidades

Use este arquivo para retomar o trabalho entre sessões de terminal. Os critérios de cada unidade
continuam no seu guia. O [roadmap](../../02-produto/roadmap.md) registra entregas de produto,
enquanto este registro aponta para a execução e a evidência de cada unidade.

## Situação inicial

Em 2026-10-15, foram escritos os guias de implementação. Nenhuma unidade F0–F4 foi executada
como parte dessa tarefa documental. As entregas anteriores continuam descritas no roadmap.
A ausência de registro de uma unidade não prova ausência de código. Confira o checkout.

A recuperação da fonte navegável do protótipo continua pendente conforme
[design-aprovado.md](design-aprovado.md#recuperar-a-fonte-do-protótipo).

## Registrar uma execução

Acrescente uma seção com o ID da unidade e a data. Use `parcial`, `bloqueada` ou `concluída`.
Indique os caminhos alterados e a evidência durável, como PR, commit, relatório de teste ou
artefato local autorizado. Se o relatório estiver no PR, faça referência a ele sem copiar a descrição.

Para trabalho ainda não commitado, registre os caminhos e os resultados mínimos necessários
à retomada. Não publique material privado, credenciais ou conteúdo de gabaritos neste arquivo.
Registre o motivo de checks indisponíveis e a próxima ação exata. Concluir uma sessão não
significa concluir a unidade.

O formato de registro está em [regras para agentes](regras-para-agentes.md#retomar-e-encerrar).
Se houver mais de um agente, deixe a atualização deste arquivo com o responsável por integração.

## Execuções

### F0.1 — 2026-10-15

- **Papel:** `qa-engineer`.
- **Estado:** parcial.
- **Evidência:** [linha de base do Laboratório](f0-1-linha-de-base.md), com ambiente,
  comandos, resultados, limitações e passos de reprodução.
- **Alterações desta unidade:** este registro e `f0-1-linha-de-base.md`. Código de produção,
  manifests, lockfiles e alterações locais anteriores foram preservados.
- **Verificação:** instalação com lockfile congelado; 12 arquivos e 58 testes do front
  aprovados; ESLint isolado aprovado. `check` e build falharam por WASM ausente. Lint oficial
  falhou em 116 arquivos; diagnóstico não mutante confirmou diferença CRLF/LF do checkout.
  As seis rotas do Laboratório foram abertas em Chromium e exibiram `500 Internal Error`
  por falha ao resolver o módulo WASM. `git diff --check` passou antes e após o relatório;
  links locais dos dois documentos foram conferidos.
- **Não verificado:** fmt, Clippy e testes Rust, pois `cargo` está indisponível; build WASM,
  pois `wasm-pack` está indisponível; resultados matemáticos e interação nas seis telas,
  bloqueados pelo artefato ausente. Desktop não testado.
- **Fonte visual:** `prototype/vertice-design` continua ausente nas referências locais e
  nos heads do remoto. Recuperação pelo mantenedor e comparação visual continuam pendentes,
  separadas dos bloqueios funcionais.
- **Próxima ação:** retomar F0.1 com Rust e `wasm-pack` disponíveis, gerar WASM, repetir os
  checks e executar os seis cenários definidos no relatório. Depois avaliar F0.2; não iniciada.

### F0.1 — retomada em 2026-10-15

- **Papel:** `qa-engineer`.
- **Estado:** concluída como coleta de linha de base, com falhas preexistentes registradas.
- **Evidência:** seção [retomada executada](f0-1-linha-de-base.md#retomada-executada--2026-10-15)
  do relatório, com versões, comandos, cenários e reprodução das falhas.
- **Alterações desta retomada:** `f0-1-linha-de-base.md` e este registro. Código de produção,
  manifests, lockfiles e trabalho anterior preservados; artefatos gerados ignorados pelo Git.
- **Verificação:** fmt e Clippy aprovados; 140 testes Rust aprovados; WASM real gerado;
  `check` sem erros ou avisos; 58 testes do front aprovados; build estático aprovado.
  Os seis cenários matemáticos foram executados em Chromium, no dev e no preview estático.
  Derivadas também atualizou ponto e inclinação pelo teclado.
- **Falhas conhecidas:** lint oficial reprovado (147 arquivos, incluindo gerados; diagnóstico
  CRLF com ignorados e ESLint isolado aprovados). Vetores calcula os valores esperados, mas
  emite `effect_update_depth_exceeded` nos dois modos; visualização 3D não aprovada.
  Encaminhamentos ao `frontend-developer` registrados, sem correção nesta unidade.
- **Fonte visual:** `prototype/vertice-design` continua ausente nas referências locais e
  heads remotos; recuperação pelo mantenedor e comparação visual pendentes, sem bloquear
  a conclusão da coleta.
- **Não verificado:** desktop, arraste por mouse, câmera 3D funcional, temas, responsividade,
  acessibilidade completa e fidelidade ao protótipo.
- **Próxima unidade:** F0.2 pode ser solicitada com falhas conhecidas; não iniciada.

### Correção do lint após F0.1 — 2026-10-15

- **Papel:** `frontend-developer`; correção autorizada separadamente, sem iniciar F0.2.
- **Alterações:** `.gitattributes` fixa LF somente em `apps/web/`; scripts `lint` e `format`
  em `apps/web/package.json` reutilizam o `.gitignore` da raiz. O ESLint também ignora os
  arquivos temporários `vite.config.*.timestamp-*`, que podem desaparecer durante os checks.
- **Preservação:** `format` normalizou finais de linha; o diff de código não contém mudanças
  em componentes, estilos, regras matemáticas, imports ou camadas. Regras arquiteturais do
  ESLint permanecem intactas. Dependências e lockfiles não mudaram.
- **Verificação:** lint oficial aprovado, `check` sem erros ou avisos e 58 testes aprovados.
  Lint e testes também passaram simultaneamente. Asserções com `ESLint.isPathIgnored`
  confirmaram exclusão dos gerados e manutenção do código de domínio no escopo do lint.
- **Pendências:** a falha 3D de vetores e a recuperação do protótipo continuam abertas.
  Os resultados históricos da coleta F0.1 permanecem no relatório, sem serem reescritos.

### F0.2 — 2026-10-15

- **Papéis:** `backend-developer`, com bindings e pacote web encaminhados ao
  `frontend-developer` em entrega separada e integrada nesta execução.
- **Estado:** parcial; renome técnico local aplicado, mas build desktop não aprovado.
- **Evidência:** [renome técnico local](f0-2-renome-tecnico.md), com escopo, comandos,
  verificações de preservação e reprodução do bloqueio desktop.
- **Alterações:** crates em `crates/vx-*`, manifests Cargo e pnpm, `Cargo.lock`, imports,
  bindings WASM/Tauri, `.gitignore`, configuração Tauri e documentos operacionais.
  Alterações anteriores preservadas; ADRs históricos e relatório F0.1 não foram reescritos.
  Nenhuma atualização de dependência ou mudança nas regras matemáticas.
- **Verificação:** fmt e Clippy aprovados; 140 testes Rust e 58 testes front aprovados;
  WASM real gerado; `check` sem erros e avisos; lint e build estático aprovados. Smoke direto
  do binding `vx_wasm` confirmou os seis resultados matemáticos da F0.1 e o erro de comando
  desconhecido. Metadados, lockfiles e equivalência mecânica dos 57 arquivos das crates
  conferidos; links operacionais e `git diff --check` aprovados.
- **Bloqueio:** `cargo build -p vertice-desktop --locked` falhou porque
  `apps/desktop/src-tauri/icons/icon.ico` não existe. A configuração e a ausência dos assets
  antecedem o renome. Não foram criados ícones provisórios nem removida a exigência do build.
- **Não verificado:** desktop executável/instaladores, Linux, macOS, migração de dados desktop
  e interação no navegador nesta unidade. Falha 3D e recuperação do protótipo permanecem abertas.
- **Pendências externas:** confirmar escopo npm e identificador Tauri antes de publicar;
  renome GitHub separado. Nenhuma publicação ou alteração do remoto foi realizada.
- **Próxima ação:** encaminhar os ícones aprovados ao `frontend-developer`, repetir build
  desktop Windows e encerrar os critérios da F0.2. F0.3 não foi iniciada.

### F0.2 — retomada com ícone provisório em 2026-10-15

- **Autorização:** o mantenedor pediu uma imagem simples temporária e nova tentativa de build.
- **Papéis:** asset encaminhado ao `frontend-developer`; build integrado pelo `backend-developer`.
- **Alterações:** `apps/desktop/src-tauri/icons/{icon.svg,icon.png,icon.ico,README.md}`.
  Ícone autoral provisório; não define a identidade visual final.
- **Verificação:** `cargo build -p vertice-desktop --locked` passou no Windows x64, saída 0.
  O bloqueio por ausência do ícone foi resolvido. `git diff --check` passou.
- **Não concluído:** build release pela CLI Tauri com `--no-bundle --ci`, cancelado pela
  reinicialização do servidor do agente. Precisa ser repetido; sem instaladores, assinatura
  ou publicação. O build debug aprovado permanece válido.

### F0.2 — build release repetido após reinicialização

- **Papel:** `backend-developer`.
- **Estado:** concluída como renome técnico local.
- **Verificação:** `corepack pnpm -C apps/desktop exec tauri build --no-bundle --ci` terminou
  com saída 0, incluindo build estático do front e compilação release Windows x64.
  Executável: `target/release/vertice-desktop.exe`.
- **Evidência:** [repetição concluída](f0-2-renome-tecnico.md#repetição-concluída-após-a-interrupção).
- **Limites:** sem instaladores, assinatura, publicação ou execução interativa. Linux e macOS
  não testados. Ícone provisório autorizado; confirmação do escopo npm e identificador Tauri
  permanece necessária antes de publicação. A CLI avisa contra o sufixo `.app` no identificador.
- **Próxima unidade:** F0.3 pode ser solicitada; não iniciada. Renome GitHub permanece externo.

### F0.3 — 2026-10-15

- **Papéis:** `backend-developer` e `frontend-developer`, com entregas separadas no
  [relatório de dependências](f0-3-dependencias.md).
- **Estado:** concluída como atualização compatível, com majors deliberadamente separadas
  e limitações de segurança registradas. Não significa que F0 esteja concluída.
- **Alterações:** manifests Cargo/npm, `Cargo.lock`, `pnpm-lock.yaml`, requisitos no setup
  e quatro arquivos de navegação para usar `resolve` e `RouteId`, exigidos pelo novo lint.
  Regras matemáticas, contratos, URLs e trabalho anterior foram preservados.
- **Verificação:** fmt e Clippy aprovados; 140 testes Rust e 58 testes web aprovados;
  `check` sem erros/avisos, lint, instalação congelada com peers estritos, WASM, build
  estático e builds desktop debug/release Windows x64 aprovados. Os seis cenários da F0.1
  preservaram resultados em dev e preview; comparação automatizada do preview antes/depois
  passou. Links, reload, voltar/avançar, teclado na tangente e tema também foram verificados.
- **Segurança:** audit npm passou de 13 achados para 1 baixo (`cookie` via SvelteKit 2).
  `cargo audit` não está instalado; segurança da árvore Rust não verificada.
  Kit 3/adapter 4/TypeScript e aumento do mínimo Rust do motor não foram incorporados.
- **Limites:** falha 3D `effect_update_depth_exceeded` e recuperação do protótipo continuam
  pendentes. Sem execução interativa desktop, instaladores, assinatura, Linux/macOS ou
  publicação. Detalhes e tentativas não aprovadas estão no relatório.
- **Próxima unidade:** F0.4 pode ser solicitada; F0.6 também tem dependências locais atendidas.
  Nenhuma delas foi iniciada. Scanners e revisão de segurança permanecem necessários na F0.6.

### F0.4 — 2026-10-15

- **Papel:** `frontend-developer`.
- **Estado:** concluída para navegação e identidade visual documental, sem antecipar Site D.
- **Evidência:** [navegação e identidade visual](f0-4-navegacao.md), com contrato das rotas,
  ambiente, cenários, capturas locais e limites de verificação.
- **Alterações:** tópicos movidos para `apps/web/src/routes/lab/`; entradas antigas preservadas
  por `LegacyRedirect.svelte`; entrada transitória do Site em `/`; shell, navegação, títulos,
  marca Vértice e textos em `lib/ui/copy/navigation.ts`; teste de contrato de navegação;
  documentação do front, componentes, módulos e checklist correspondente no roadmap.
- **Preservação:** trabalho anterior, dependências, lockfiles, motor e regras matemáticas
  mantidos. Sem conta, player de exercícios, servidor, Biblioteca funcional ou publicação.
- **Verificação:** linha de base com 58 testes; final com 61 testes em 13 arquivos, `check`
  sem erros/avisos, lint e build estático aprovados. Chromium no preview verificou sete rotas
  diretas com reload, 21 casos de URLs antigas com query/fragmento, histórico com substituição,
  links Site/Laboratório, tema persistido, teclado, menu móvel e breakpoints 899/900px.
  A navegação também passou servindo somente `build/`, sem fallback SPA ou servidor SvelteKit.
  Seis cenários matemáticos coincidiram com a coleta da F0.3. `git diff --check` aprovado.
- **Limites:** erro 3D `effect_update_depth_exceeded` preexistente continua. Fidelidade ao
  protótipo não verificada, pois a fonte permanece ausente. Desktop interativo, outros
  navegadores, auditoria completa de acessibilidade e deploy não executados.
- **Próxima unidade:** F0.5 pode ser solicitada, com fonte visual necessária à comparação
  final. F0.6 também tem dependências atendidas. Nenhuma delas iniciada nesta execução.

### F0.6 — 2026-10-15

- **Papéis:** `devops-engineer`; revisão T1–T3 no papel `security-researcher`, sem correção de
  código de produção pelo revisor.
- **Estado:** parcial; configuração local implementada, sem execução real em PR.
- **Evidências:** [CI e verificações locais](f0-6-ci-seguranca.md) e
  [revisão de segurança](../../06-seguranca/revisao-f0-6.md), com achados separados das correções.
- **Alterações:** workflows `ci.yml`/`security.yml`, composite `setup-web`, políticas cargo-deny
  e Semgrep, guard privado com testes, Renovate para Actions, `SECURITY.md`, CI/CD, ameaças e
  registros. Trabalho anterior preservado; sem alteração de motor, UI ou dependências.
- **Verificação:** fmt, Clippy, 140 testes Rust, check sem erros/avisos, lint, 61 testes web,
  WASM real e build estático aprovados. Guard: 9 testes aprovados e 1 ignorado no Windows;
  árvore e histórico de `HEAD` aprovados. Fixtures Semgrep confirmaram três regras bloqueantes.
  Audit npm produção sem achados; completo mantém o achado baixo de `cookie` da F0.3.
  Actionlint, Semgrep (171 alvos, zero achados) e Gitleaks aprovados. Cargo-audit terminou
  com quatro avisos; OSV bloqueou quatro pacotes e cargo-deny bloqueou manutenção/licenças,
  além de diagnóstico operacional Windows. Detalhes e encaminhamentos no relatório.
- **Segurança:** API confirmou `main` desprotegida e reporte privado desativado; aviso
  documental adicionado, sem alterar configurações remotas. Trap do parser profundamente
  aninhado reproduzido no WASM, encaminhado ao backend/front. Cabeçalhos continuam na F0.7.
- **Não verificado:** execução GitHub em PR/fork, CodeQL, Scorecard, deploy e assinatura.
  Demais scanners e resultados finais constam no relatório da unidade.
- **Próxima ação:** triar os achados dos scanners, organizar publicação autorizada em PR, registrar
  os checks reais e configurar os serviços externos indicados. Não avançar para F0.7 como se
   a dependência estivesse concluída. Nenhum commit, push, PR ou deploy nesta execução.

### F1.1 — 2026-10-15

- **Papel:** `backend-developer`.
- **Estado:** bloqueada antes de fixar o contrato público.
- **Dependência:** F0.2 registrada como concluída no renome técnico local; esta sessão não
  repetiu seus builds.
- **Bloqueio de contrato:** `visao-geral.md` atribui `AnswerSpec` a `assessment`, enquanto o
  diagrama e a API pública do ADR 0014 atribuem `Part/AnswerSpec` a `content`. A F1.1 exige
  resolver essa divergência com o `software-architect` antes de fixar a API pública e exige
  que conteúdo consuma a API pública do corretor. A localização de `AnswerSpec` precisa de
  decisão explícita; não foi escolhida unilateralmente pelo desenvolvedor.
- **Alterações:** somente esta entrada. Código, contratos, manifests, lockfiles e alterações
  anteriores foram preservados.
- **Verificação:** inspeção do estado Git, do código de entrada de `vx-domain` e dos documentos
  citados. Nenhum teste, build ou gerador foi executado, pois a implementação não começou.
- **Pendências:** revisão de contrato pelo `software-architect`, implementação dos tipos e
  validações, fixtures, geração reproduzível de JSON Schema/TypeScript e checks da unidade.
- **Próxima ação:** obter autorização para coordenar a revisão com o `software-architect`,
  resolver a propriedade de `Part` e `AnswerSpec` e retomar F1.1. F1.2 não foi iniciada.

### F1.1 — retomada autorizada em 2026-10-15

- **Papéis:** `backend-developer`, com revisão de contrato pelo `software-architect`,
  coordenada após autorização explícita do mantenedor.
- **Estado:** concluída no recorte estrutural da unidade; não conclui F1.
- **Evidência:** [contratos de conteúdo e referências](f1-1-contratos.md), com escopo,
  comandos de geração, cenários, verificações e limites. O
  [ADR 0016](../../03-arquitetura/adr/0016-contrato-de-conteudo-f1.md) resolve a divergência
  inicial: `Part` é de `content`, que consome `assessment::AnswerSpec` pela fachada pública.
- **Alterações:** módulos `shared`, `assessment` e `content` em `vx-domain`; DTOs, conversões,
  decoder, gerador e fixtures de contrato em `vx-api`; cinco schemas em `contracts/` e
  `apps/web/src/lib/domain/generated/content.ts`; scripts de geração no `package.json`,
  dependências externas de `vx-api`, `Cargo.lock` e documentação relacionada.
  Trabalho anterior preservado; sem alterações de UI, regras matemáticas ou comandos existentes.
- **Verificação:** linha de base com 140 testes Rust. Final com 186 testes Rust, fmt e Clippy
  `-D warnings` aprovados; geração determinística e check de contratos atualizados aprovados;
  `check` web sem erros/avisos, lint e 61 testes web aprovados; WASM e build estático aprovados.
  Revisão arquitetural final aprovou o contrato após correções com regressões de schema,
  caminhos de erro em elementos, identidade UUID e preservação dos números JSON técnicos.
- **Não verificado:** interação em navegador ou desktop, build desktop, comportamento das
  novas fachadas em execução WASM e aceites com pack privado. Sem auditoria das novas
  dependências ou execução em CI remota. Os limites não impedem o aceite estrutural da F1.1.
- **Pendências posteriores:** correção F1.2, compilador/catálogo F1.3, evidência F1.4,
  armazenamento/importação F1.5, visualizações/UI e checks de CI de conteúdo nas unidades
  correspondentes. `verify` e `evidence` continuam rejeitados até sua implementação.
- **Próxima unidade:** F1.2, dependente da F1.1 agora entregue. Nenhum commit, push, PR,
   publicação ou avanço para F1.2 nesta execução.

### F1.2 — primeiro grupo em 2026-10-15

- **Papel:** `backend-developer`.
- **Estado:** parcial; grupo `numeric` e `expression` sem `form`, dividido conforme as regras
  de entrega verificável. F1.2 não está concluída e F1.3 não foi iniciada.
- **Evidência:** [correção do primeiro grupo](f1-2-correcao.md), com contrato, algoritmo,
  limites, reprodução e pendências.
- **Alterações:** comparação pura em `vx-domain/src/assessment`, avaliação de AST com múltiplas
  variáveis preservando os métodos anteriores, `CheckAnswer` em `vx-application`, DTOs e
  `assessment.check` no `dispatch`, schemas e TypeScript gerados, comando registrado no front,
  fixtures autorais e harness de paridade native/WASM. Sem dependências novas, UI, persistência,
  pontuação ou evidência da F1.4. Alterações anteriores preservadas.
- **Verificação:** linha de base com 186 testes Rust; suite ampliada com 210 testes aprovados;
  fmt, Clippy `-D warnings`, geração/check de contratos, WASM, check web sem erros/avisos,
  lint, 61 testes web e build estático aprovados. As 27 fixtures deram respostas completas
  iguais no nativo e no binding WASM real executado no Node; limites recursivos não causaram
  trap nessa operação. `git diff --check` aprovado.
- **Cenários:** formas equivalentes, vírgula/fração/raiz/constante, tolerâncias, unidades
  opcionais, sintaxe com posição separada de erro matemático, feedback por equivalência,
  múltiplas variáveis, funções por partes, `x/x` contra `1` nos dois sentidos, `sqrt(x^2)`
  contra `x`, domínio restrito, pontos inválidos e amostragem insuficiente.
- **Não verificado:** navegador interativo, transporte/build desktop, Linux/macOS, CI remota,
  pack privado, latência do PRD e recarga offline. Teste nativo não comprova transporte Tauri.
- **Pendências:** equação e coleções; escolhas, associação e autoavaliação sem correção
  automática; checagem estrutural de `form`; contratos e paridade dos demais grupos.
  Tipos e formas ainda não executáveis são rejeitados explicitamente, nunca aceitos por omissão.
- **Próxima ação:** continuar F1.2 pelo grupo equação e coleções. Não marcar o corretor completo
  no roadmap nem avançar para F1.3 como se a dependência estivesse concluída. Nenhum commit,
  push, PR ou publicação nesta sessão.

### F1.2 — retomada dos grupos pendentes em 2026-10-15

- **Papel:** `backend-developer`.
- **Estado:** concluída no escopo de motor/contratos da unidade, sem concluir a fase F1.
- **Evidência:** [retomada da correção](f1-2-correcao.md#retomada--demais-grupos-e-checagem-de-forma),
  com gramática de submissão, regras, limitações e comandos executados.
- **Alterações:** domínio para equações por resíduos proporcionais, casamento bijetivo de
  conjuntos por caminhos aumentantes, composição ordenada, comparação objetiva e `form` na
  AST; extensão de `CheckAnswer`, parser limitado de entradas compostas, DTOs e contratos
  gerados para os nove tipos. Autoavaliação retorna `self_graded`, nunca `correct` automático.
  Acrescentado teste IPC de `engine` no Tauri com `MockRuntime`, usando as mesmas fixtures.
  Feature de teste somente em dependência de desenvolvimento; nenhuma versão nova de crate.
- **Verificação:** linha de base com 210 testes Rust; final com 225 testes do motor e um teste
  desktop aprovados. As 71 fixtures autorais passaram no contrato JSON e produziram respostas
  completas iguais no nativo, WASM real e IPC Tauri simulado. fmt, Clippy do motor e desktop,
  geração/check de contratos, build WASM, build desktop debug Windows x64, check web sem
  erros/avisos, lint, 61 testes web, build estático e `git diff --check` aprovados.
- **Preservação:** alterações anteriores, limites de licença/visibilidade e contratos canônicos
  estruturais mantidos. Sem player, armazenamento, compilador, evidência ou pontuação parcial;
  decisões de produto do PRD continuam abertas. Material privado não foi acessado.
- **Não verificado:** WebView/janela Tauri real, navegador interativo, release/instaladores
  desktop, Linux/macOS, CI remota, pack privado, latência do PRD e recarga offline. O IPC usa
  runtime simulado; esses resultados não constituem E2E do sistema operacional nem aceite F1.10.
- **Próxima unidade:** F1.3, com F1.1 e F1.2 atendidas; não iniciada nesta retomada. Nenhum
  commit, push, PR ou publicação. O roadmap marca somente o corretor no motor como entregue.

### Consolidação para publicação no repositório — 2026-10-16

- **Papel:** `devops-engineer`.
- **Autorização:** o mantenedor solicitou os commits das fases implementadas e o push para
  o repositório público existente, sem renomear o repositório nem publicar pacotes ou instaladores.
- **Commits:** `dbd6153` reúne o renome técnico, dependências do motor, contratos F1.1 e
  corretor F1.2; `50ec0ac` reúne dependências web, navegação F0.4, bindings e configuração de
  formatação; `4077d27` reúne CI, políticas e guardas da F0.6. A documentação e as evidências
  anteriores são consolidadas em commit separado. Essa organização registra o estado integrado
  existente, sem reconstruir artificialmente commits para cada execução histórica.
- **Proteção de conteúdo:** acrescentado `references/design/capturas/` ao `.gitignore`,
  alinhando o caminho alternativo à política já aplicada a `docs/references/design/capturas/`.
  Capturas comerciais permanecem locais; conteúdo privado não foi lido nem preparado para publicação.
- **Verificação:** fmt, Clippy do motor com `--locked` e `-D warnings`, testes Rust do motor,
  check dos contratos gerados, builds WASM e estático web, check web sem erros/avisos, lint e
  61 testes web aprovados. As 71 fixtures produziram respostas iguais no nativo e WASM; o teste IPC Tauri
  passou. Guard privado com 9 testes aprovados e 1 ignorado pela restrição de nomes do Windows;
  actionlint, Gitleaks sobre o índice preparado e os três commits técnicos, guard sobre árvore
  e intervalo de commits, e `git diff --check` aprovados.
- **Ambiente:** pnpm 9.12.0 via `corepack`. Checks bloqueados por permissões do sandbox foram
  repetidos com sucesso fora dele. Um `index.lock` vazio, datado do dia anterior, foi removido
  após confirmar a ausência de processo Git ativo. Rust avisou sobre cópia em vez de hard links
  no cache incremental; isso não reprovou os checks.
- **Limites:** scanners de dependências e SAST da F0.6 não foram repetidos nesta consolidação;
  seus achados anteriores continuam registrados. CI remota, revisão em PR/fork, CodeQL,
  Scorecard, proteção de branch, reporte privado, navegador e janela desktop reais não foram
  verificados nesta sessão. F0.6 permanece parcial; F0 e F1 não são declaradas concluídas.
- **Destino solicitado:** `origin/main`, em `CauaDecoder/bad-at-calculus`, confirmado público e
  sem commits remotos ausentes do checkout antes da consolidação. Nenhum force push previsto.
- **Próxima unidade:** F1.3 pode ser solicitada; pendências da F0.6 continuam no relatório
  correspondente. Nenhuma unidade nova foi implementada nesta consolidação.
