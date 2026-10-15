# Módulos

Os módulos seguem as disciplinas do curso; cada um tem **tópicos** (uma página cada).
Cada tópico é uma fatia vertical: regras em `mv-domain/src/<assunto>`, casos de uso em
`mv-application/src/use_cases`, comando em `mv-api`, estado em `apps/web/src/lib/application` e
telas em `apps/web/src/lib/ui/<assunto>`.

Legenda: ✅ pronto · 🔜 planejado

## Cálculo

| Tópico | Rota | O que tem |
| ------ | ---- | --------- |
| ✅ Limites e continuidade | `/calculo/limites` | Limites laterais e no infinito, tabela de aproximação, bolinhas abertas/fechadas no gráfico, três condições de continuidade, tipo de descontinuidade |
| ✅ Derivadas | `/calculo/derivadas` | Tangente arrastável, `f'(x₀)`, traçado de `f'`, derivada simbólica com cada regra (e cadeia), `f''`, pontos críticos |
| ✅ Integrais | `/calculo/integrais` | Área com sinal, somas de Riemann (esquerda, direita, ponto médio, trapézio), primitiva e TFC |
| 🔜 Funções | — | Domínio, composição, inversa (Lista 01) |
| 🔜 Técnicas de integração | — | Substituição e partes com passos |

Todas as páginas aceitam funções por partes: `{x < 1: x^2; x >= 1: 2x}`.

## Vetores e Geometria Analítica

| Tópico | Rota | O que tem |
| ------ | ---- | --------- |
| ✅ Operações com vetores | `/vetores/operacoes` | Vista 3D; produto escalar e vetorial (com determinante), normas, ângulo, projeção, versores, áreas, ortogonal/paralelo, produto misto e coplanaridade |
| ✅ Retas e planos | `/vetores/retas-e-planos` | Plano por 3 pontos (normal, equação geral), reta paramétrica, distâncias ponto–plano e ponto–reta |
| ✅ Matrizes | `/vetores/matrizes` | Determinante (2×2, Sarrus, escalonamento), inversa por [A \| I], posto, escalonamento passo a passo, transposta, traço, autovalores e autovetores, A + B, A·B |
| ✅ Sistemas lineares | `/vetores/sistemas` | Gauss–Jordan passo a passo, SPD/SPI/SI, solução paramétrica |
| 🔜 Cônicas e quádricas | — | Identificação, elementos e gráfico |
| 🔜 Transformações lineares | — | Mudança de base, efeito no plano |

Todos os campos numéricos aceitam frações e raízes (`1/2`, `sqrt(3)`, `pi/4`), e os resultados
aparecem em forma exata sempre que possível.

## 🔜 Métodos numéricos
- Raízes: bisseção, Newton-Raphson (com as iterações).
- EDOs: Euler, Runge-Kutta 4, campo de direções.
- Erro e convergência em gráficos log-log.

## 🔜 Física
- Cinemática: `s(t)`, `v(t) = s'(t)`, `a(t) = v'(t)` sincronizados.
- Lançamento de projéteis, campos vetoriais, osciladores.

## Transversal
- ✅ Editor de expressões com LaTeX ao vivo e ajuda "Como digitar".
- ✅ Exemplos clicáveis em cada página (vários tirados das listas de Cálculo A).
- 🔜 Workspaces (`.mathview`), exportação PNG/SVG, command palette (`Ctrl/⌘ + K`).
