# Módulos

A Vértice é organizada em **módulos de produto** (áreas da navegação) e em **disciplinas** (o
conteúdo). O Laboratório e os Exercícios são transversais a todas as disciplinas: uma disciplina
nova ganha visualizações no Laboratório e exercícios no banco.

Legenda: ✅ pronto · 🟡 parcial · 🔜 planejado (fase do [roadmap](roadmap.md) entre parênteses)

## Módulos de produto

| Módulo | Rota base | O que faz | Status |
| ------ | --------- | --------- | ------ |
| **Laboratório** | `/lab` | Visualizações interativas por disciplina (o antigo Math View) | ✅ Cálculo e VGA · 🔜 rotas em `/lab` (F0) |
| **Exercícios** | `/exercicios`, `/listas` | Banco, listas, resolver com correção automática, dicas, solução em passos, "Abrir no laboratório" | 🔜 (F1) — [PRD](prd-exercicios.md) |
| **Autoria** | `/autoria` | Criar/editar exercício com preview; importar em lote (pack, CSV, gabarito separado); importar PDF com IA | 🔜 editor e pack (F1) · PDF com IA (F2) |
| **Revisão** | `/revisao` | Repetição espaçada (FSRS) dos itens errados ou marcados; **caderno de erros** | 🔜 (F2) |
| **Plano de estudos** | `/plano` | Plano com data de prova, checklist por dia e simulados (inspirado no fluxo real de estudo de POO) | 🔜 (F2) |
| **Progresso** | `/progresso` | Domínio por habilidade, histórico, sequência de estudos | 🔜 local (F2) |
| **Conta e comunidade** | `/conta`, `/grupos` | Login, sync, listas compartilhadas, turmas, publicação de packs | 🔜 (F3–F4) |

Taxonomia de disciplinas, tópicos e habilidades: [taxonomia.md](taxonomia.md).

## Laboratório — o que já existe

Cada tópico é uma fatia vertical: regras em `mv-domain/src/<assunto>`, casos de uso em
`mv-application/src/use_cases`, comando em `mv-api`, estado em `apps/web/src/lib/application` e
telas em `apps/web/src/lib/ui/<assunto>`.

### Cálculo

| Tópico | Rota atual | O que tem |
| ------ | ---------- | --------- |
| ✅ Limites e continuidade | `/calculo/limites` | Limites laterais e no infinito, tabela de aproximação, bolinhas abertas/fechadas, três condições de continuidade, tipo de descontinuidade |
| ✅ Derivadas | `/calculo/derivadas` | Tangente arrastável, `f'(x₀)`, traçado de `f'`, derivada simbólica com cada regra, `f''`, pontos críticos |
| ✅ Integrais | `/calculo/integrais` | Área com sinal, somas de Riemann, primitiva e TFC |
| 🔜 Funções | — | Domínio, composição, inversa (Lista 01) |
| 🔜 Técnicas de integração | — | Substituição e partes com passos |

Todas as páginas aceitam funções por partes: `{x < 1: x^2; x >= 1: 2x}`.

### Vetores e Geometria Analítica

| Tópico | Rota atual | O que tem |
| ------ | ---------- | --------- |
| ✅ Operações com vetores | `/vetores/operacoes` | Vista 3D; escalar, vetorial, normas, ângulo, projeção, produto misto |
| ✅ Retas e planos | `/vetores/retas-e-planos` | Plano por 3 pontos, reta paramétrica, distâncias |
| ✅ Matrizes | `/vetores/matrizes` | Determinante, inversa, posto, escalonamento, autovalores |
| ✅ Sistemas lineares | `/vetores/sistemas` | Gauss–Jordan passo a passo, SPD/SPI/SI |
| 🔜 Cônicas e quádricas | — | Identificação, elementos e gráfico |
| 🔜 Transformações lineares | — | Mudança de base, efeito no plano |

Na Fase 0 as rotas passam para `/lab/calculo/...` e `/lab/vetores/...`, com redirecionamento das
antigas ([ADR 0006](../03-arquitetura/adr/0006-plataforma-vertice.md)).

## Disciplinas planejadas (Laboratório + Exercícios)

### Exatas
- **Física I** — cinemática `s(t)`, `v(t) = s'(t)`, `a(t) = v'(t)` sincronizados; projéteis; vetores força. (F2)
- **Métodos numéricos** — bisseção e Newton-Raphson animados; Euler e RK4; erro em log-log. (F2)
- **EDOs** — campo de direções e soluções. (F2+)
- **Cálculo multivariável** — superfícies 3D com WebGL (Threlte). (futuro)

### Computação (ADS)
- **Programação Orientada a Objetos (Java)** — exercícios de código com testes, UML gerado a partir
  das classes, caderno de erros. Java no navegador ainda é um ponto aberto (ver
  [estratégia de IA e execução de código](../08-ia/estrategia-ia.md#execução-de-código)). (F2–F3)
- **Algoritmos e estruturas de dados** — execução passo a passo de ordenação, busca, pilha/fila,
  árvores; gráfico de crescimento O(n). (F2)
- **Banco de dados** — SQL executado no navegador (sqlite-wasm/PGlite) com diagrama ER e resultado
  comparado ao esperado. (F2)
- **Redes** — calculadora e visualização de sub-redes. (futuro)

### Arquitetura e Urbanismo
- **Estruturas I** — forças aplicadas, decomposição e resultante (reaproveita o motor de vetores),
  equilíbrio, reações de apoio, treliças, diagramas de esforço cortante e momento fletor; sistemas
  estruturais. (F2, com a colaboradora de AU)
- **Geometria descritiva** — épura de Monge sincronizada com a vista 3D; verdadeira grandeza. (F2+)
- **Conforto ambiental** — carta solar e insolação por latitude e data. (futuro)
- **Desenho e escalas** — conversões de escala, áreas e perímetros. (futuro)

## Transversal
- ✅ Editor de expressões com LaTeX ao vivo e ajuda "Como digitar".
- ✅ Exemplos clicáveis em cada página (vários tirados das listas de Cálculo A).
- 🔜 Command palette (`Ctrl/⌘ + K`), exportação PNG/SVG, workspaces (F0–F1).
- 🔜 **Registro de visualizações**: toda visualização tem `id`, parâmetros tipados e URL com estado,
  para poder ser embutida num exercício (F1, [ADR 0008](../03-arquitetura/adr/0008-formato-canonico-de-exercicio.md)).
