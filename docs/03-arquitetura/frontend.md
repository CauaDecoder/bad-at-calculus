# Arquitetura do frontend (`apps/web`)

SvelteKit 2 + Svelte 5 (runes) + TypeScript estrito, gerado como site estático
(`adapter-static`) para rodar dentro do Tauri e em qualquer host estático.

## Camadas do front

```
src/
├─ lib/
│  ├─ domain/        tipos e funções puras: calculus, vectors, linear-algebra,     ← não importa nada
│  │                 viewport, camera3d, axis-ticks, exact-format, format-number,
│  │                 matrix-draft, engine-error
│  ├─ ports/         CalculusEngine, VectorEngine, LinearAlgebraEngine,            ← importa domain
│  │                 PreferenceStore
│  ├─ adapters/
│  │  ├─ engine/     EngineTransport (Wasm/Tauri) + adapters por assunto           ← importa ports, domain, Tauri/WASM
│  │  └─ local-preference-store.ts
│  ├─ application/   FunctionGraph, DerivativeExplorer, LimitExplorer,              ← importa domain, ports
│  │                 IntegralExplorer, VectorLab, PlaneLab, MatrixLab, SystemLab,
│  │                 ThemeController, LatestRequest
│  ├─ composition/   engines.ts — cria os adapters e os entrega por contexto
│  ├─ ui/
│  │  ├─ components/ peças genéricas (Card, NumberField, VectorInput, MatrixInput,
│  │  │              StepList, WorkedResult, Verdict, SegmentedControl…)
│  │  ├─ graph/      GraphCanvas 2D + graph-painter + plot-layers
│  │  ├─ graph3d/    SpaceCanvas 3D + space-painter
│  │  ├─ calculus/   painéis e cenas de limites, derivadas e integrais
│  │  ├─ vectors/    resultados e cenas de vetores, retas e planos
│  │  ├─ linear-algebra/ resultados de matrizes e sistemas, passos do escalonamento
│  │  ├─ layout/     AppShell, ModuleNav, TopicPage, Workspace, SplitPanel
│  │  └─ copy/       textos em PT-BR: navegação, mensagens de erro, regras de derivação
│  └─ wasm/pkg/      gerado por wasm-pack (ignorado no git)
├─ routes/           entrada do Site (/), índice /lab e tópicos /lab/calculo/*, /lab/vetores/*
└─ styles/           tokens.css (design system), base.css
```

**Regras** (as três primeiras são verificadas pelo ESLint):
- `ui/` e `application/` não importam `adapters/`, `@tauri-apps/*` nem `$lib/wasm`.
- `application/` só conhece os ports; não sabe se está no Tauri ou no navegador.
- `domain/` não importa nada de fora dele.
- O `+layout.svelte` é o composition root: cria engines e tema uma vez (`provideEngines`,
  `provideTheme`); as páginas pegam com `useEngines()`/`useTheme()`.

### Navegação estática (F0.4)

- `/` é a entrada do Site; sua composição Site D pertence à F0.5. `/lab` reúne as disciplinas
  do Laboratório. A marca e “Sobre a Vértice” voltam ao Site; “Laboratório” abre `/lab`.
- As sete páginas antigas `/calculo/*` e `/vetores/*` continuam no build como entradas
  estáticas. `LegacyRedirect` usa `location.replace` para abrir o tópico em `/lab/...`,
  preservando query string e fragmento sem interpretar os parâmetros. Isso substitui a entrada
  antiga no histórico; não depende de redirecionamento HTTP de um servidor.
- Como o app já usa `ssr = false`, os redirecionamentos e as páginas dependem de JavaScript.
  O host deve servir os arquivos `.html` por suas URLs sem extensão, ou usar o fallback SPA
  existente. Os parâmetros de estado são preservados na URL; a desserialização tipada de
  visualizações continua prevista para F1.
- Módulos futuros aparecem com fase e rótulo “Planejado”, sem links. Abaixo de 900px, um botão
  com `aria-expanded` abre a navegação e a navegação entre páginas fecha o menu.

## Estado

- Uma classe com runes (`$state`, `$derived`) por página, em `*.svelte.ts`.
- `FunctionGraph` é compartilhado pelas páginas de cálculo (expressão, LaTeX, viewport, amostras);
  cada explorer compõe um e acrescenta seu estado.
- `LatestRequest` descarta respostas antigas: ao digitar ou arrastar rápido, só a última vale.
- Enquanto a expressão tem erro de sintaxe, o último gráfico válido continua na tela.

## Gráficos

| Necessidade                       | Tecnologia                     |
| --------------------------------- | ------------------------------ |
| Curvas 2D, áreas, retângulos      | Canvas 2D (`GraphCanvas` + camadas `PlotLayer`) |
| Vetores e planos em 3D            | Canvas 2D com câmera ortográfica própria (`camera3d.ts`) |
| Fórmulas                          | KaTeX                          |

Cada página monta uma lista de camadas (`curve`, `line`, `point`, `area`, `riemann`,
`vertical-line`…) a partir do estado; o `graph-painter` só desenha. Transformações de
coordenadas (`viewport.ts`, `camera3d.ts`) são puras e testadas.

## Resultados exatos

O motor calcula em `f64`; `exact-format.ts` reconhece inteiros, frações, raízes (`2√3`) e
múltiplos de π e e, e devolve LaTeX. Use `equalsLatex(v)` para mostrar `= \frac{1}{2}` ou
`\approx 1{,}2952` automaticamente.

## Testes
- `vitest` para `domain/` e `application/` (com `FakeCalculusEngine` e `FakeLinearAlgebraEngine`
  em `application/test-support.ts`).
- Playwright para fluxos ponta a ponta (futuro). Ver [testes.md](../05-engenharia/testes.md).
