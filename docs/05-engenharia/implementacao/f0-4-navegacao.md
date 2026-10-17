# F0.4 — Navegação e identidade visual

## Estado e escopo

- **Data:** 2026-10-15.
- **Papel:** `frontend-developer`.
- **Estado:** concluída para shell, identidade Vértice, navegação e compatibilidade estática.
- **Entradas:** F0.2 e F0.3 concluídas no registro; ADRs 0006 e 0013, arquitetura do front,
  design system, princípios de UX, componentes e guia de design aprovado.
- **Preservação:** o checkout já continha alterações extensas. Não houve restauração de
  arquivos, commit, atualização de dependências, mudança do motor ou publicação externa.

## Entrega

- `apps/web/src/routes/lab/+page.svelte` reúne os sete tópicos existentes. As páginas de
  Cálculo e VGA foram movidas integralmente para `/lab/calculo/...` e `/lab/vetores/...`.
- As sete páginas antigas continuam como entradas estáticas que renderizam
  `lib/ui/layout/LegacyRedirect.svelte`. O componente executa `location.replace` com o
  destino fixo resolvido por `$app/paths`, seguido da query string e do fragmento originais.
  Não interpreta nem reserializa o estado e não aceita um destino fornecido pelo usuário.
- O redirecionamento é executado no navegador, não é um HTTP 301/308. Exige JavaScript,
  assim como o restante do app (`ssr = false`). A substituição evita reabrir a URL antiga
  ao voltar pelo histórico. O host serve os `.html` pelas URLs sem extensão ou usa o
  fallback SPA existente; não são necessários endpoints de servidor.
- `/` é a entrada transitória do Site, com apresentação curta e link “Abrir Laboratório”.
  A marca e “Sobre a Vértice” voltam a `/`; “Laboratório” abre `/lab`. Não foi implementado
  o Site D da F0.5 nem o dashboard Início A da F1.
- `lib/ui/copy/navigation.ts` centraliza os novos textos e destinos. A navegação distingue
  módulo de produto (Laboratório) de disciplinas e tópicos. Exercícios, Autoria, Biblioteca,
  Revisão, Plano de estudos, Progresso e Conta e comunidade têm fase e rótulo “Planejado”,
  sem links para páginas inexistentes.
- Marca e títulos passam a Vértice. Tokens, fontes, cores e componentes existentes foram
  preservados. A marca textual não define um logotipo final nem altera os assets desktop.
- `AppShell.svelte` mantém sidebar de 248px a partir de 900px. Em viewports menores, o
  botão de menu controla `aria-expanded` e a visibilidade da sidebar. A navegação entre
  páginas fecha o menu; o link “Pular para o conteúdo” leva ao `main` focável.
- `navigation.test.ts` verifica os sete destinos implementados, a presença das entradas
  antigas com seus destinos e a ausência de links nos módulos planejados.

## Verificações executadas

Ambiente: Windows x64, Node `v24.16.0`, pnpm via Corepack e Chromium headless
`156.0.8078.4` via Playwright disponível no cache local. Python não está disponível;
a automação usou Node, sem instalar dependências ou alterar lockfiles.

| Verificação | Resultado |
| --- | --- |
| Linha de base: `corepack pnpm -C apps/web check`, `lint`, `test` | Sem erros/avisos, lint aprovado, 12 arquivos e 58 testes aprovados |
| Final: `corepack pnpm -C apps/web check` | 0 erros, 0 avisos |
| Final: `corepack pnpm -C apps/web lint` | Prettier e ESLint aprovados, sem desabilitar regras |
| Final: `corepack pnpm -C apps/web test` | 13 arquivos e 61 testes aprovados |
| `corepack pnpm -C apps/web build` | Build estático aprovado, usando o WASM real já gerado na F0.3 |
| `git diff --check` | Aprovado; avisos de conversão LF/CRLF fora de `apps/web` permanecem |
| Links locais dos sete documentos alterados | 54 destinos conferidos |

O build foi servido por `corepack pnpm -C apps/web preview --host 127.0.0.1 --port 4176 --strictPort`.
Os testes abaixo rodaram nesse build, não no servidor de desenvolvimento:

A automação de navegação também foi repetida em `http://127.0.0.1:4177`, servindo apenas
`apps/web/build` com um servidor HTTP local que resolve arquivos `.html`, sem fallback SPA
e sem servidor SvelteKit. Os mesmos 21 redirecionamentos e cenários de navegação passaram.

1. Abertura direta e reload dos sete tópicos `/lab/...`, com HTTP 200, título Vértice e
   exatamente um tópico ativo na navegação. A existência dos 14 arquivos HTML de tópicos
   novos e entradas antigas foi conferida, sem depender só do fallback de preview.
2. As sete URLs antigas em três formatos: sem estado, com query e fragmento, e só fragmento
   (21 casos). Query usada: `?expr=x%5E2&point=3&expr=sin%28x%29&empty=#estado%20salvo`.
   Destinos preservaram codificação, parâmetros repetidos, valor vazio e fragmento.
3. Site para Laboratório, card para tópico, Laboratório para Site, voltar e avançar.
   Uma visita antiga com JSON codificado na query e fragmento também confirmou substituição
   do histórico: voltar retorna a `/lab`, avançar retorna ao destino novo com estado intacto.
4. Alternância claro/escuro, persistência do tema após reload e controles de tema no menu móvel.
5. Teclado: link para pular conteúdo com Tab/Enter, foco no `main`, ponto da tangente com
   seta direita, menu com Enter/Espaço e fechamento após abrir um tópico.
6. Viewports de 1440px, 390px, 899px e 900px; sidebar e menu mudam no breakpoint documentado.
   Sem overflow horizontal nos cenários estreitos medidos; sidebar com 248px em 900px.
   Com redução de movimento, os sete cards continuam disponíveis.
7. Os seis cenários matemáticos da F0.3 foram repetidos pelas URLs antigas, já redirecionadas.
   Textos do conteúdo, fórmulas, valores dos campos e quantidade de erros coincidiram com a
   coleta anterior: limites, derivadas, integrais, vetores, matrizes e sistemas preservados.

Uma comparação adicional confirmou que os sete arquivos de tópicos movidos são idênticos
às versões em `HEAD`, normalizados apenas os finais de linha. A migração não alterou o
código interno dessas telas.

## Evidência local

Em `%LOCALAPPDATA%/Temp/opencode/`:

- `f0-4-build.log`: saída do build estático.
- `f0-4-navigation.cjs` e `f0-4-navigation.json`: automação com asserções e resultados.
- `f0-4-static-server.cjs` e `f0-4-static-navigation.log`: servidor local sem fallback e
  repetição aprovada servindo diretamente `build/`. O JSON de navegação guarda essa última execução.
- `f0-4-desktop-{light,dark}.png`, `f0-4-mobile-menu-{390,899}.png` e
  `f0-4-viewport-{390,899,900}.png`: capturas inspecionadas para layout, tema e foco.
- `f0-4-functional.log`, `f0-1-resume-f0-4-functional.json` e capturas com o mesmo prefixo:
  repetição dos cenários pelo script `f0-1-resume-functional.cjs` já utilizado na F0.1/F0.3.

Esses artefatos são locais, não arquivos publicados nem nova infraestrutura E2E do projeto.
Para reproduzir em outra máquina, execute os checks e o preview acima e repita os sete cenários
descritos nesta seção. A suíte versionada protege os destinos e as entradas de compatibilidade;
as interações requerem navegador.

## Limitações e próxima unidade

- A falha preexistente `effect_update_depth_exceeded` continua nas telas com visualização 3D
  de vetores/retas e planos. O smoke identifica esses erros separadamente; não aprova a
  visualização 3D nem apresenta sua correção como parte da F0.4. Não houve respostas HTTP
  de erro ou outros erros JS nos cenários executados.
- A fonte navegável do protótipo não foi recuperada nesta unidade. A revisão visual usou
  ADR 0013 e design system; **fidelidade ao protótipo não verificada**. Foram inspecionadas
  capturas claras, escuras e móveis, sem afirmar comparação com a fonte ausente.
- Não foram executados Rust/WASM novamente, desktop interativo, Firefox/Safari, leitor de
  tela, auditoria completa de acessibilidade, deploy ou configuração de host de produção.
- A preservação de parâmetros não implementa a desserialização tipada de visualizações da F1.
- Próxima unidade: F0.5, com recuperação da fonte visual para comparação final. F0.6 continua
  elegível pelas dependências locais. Nenhuma das duas foi iniciada nesta execução.
