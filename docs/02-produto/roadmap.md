# Roadmap

Versões pequenas, cada uma entregando algo utilizável. Datas são intencionais, não promessas.

## v0.0 — Fundação ✅
- Documentação, Clean Architecture, esqueleto de crates e do app web/desktop.

## v0.1 — "Tangente viva" ✅
- [x] Parser de expressões (Pratt): `2x`, `x²`, `sin x`, `|x|`, `e^x`, `pi`, vírgula decimal, prefixo `f(x) =`.
- [x] `GraphCanvas` em Canvas 2D: eixos, grade, rótulos, zoom (roda/botões/teclado) e pan.
- [x] Ponto arrastável, tangente, rastro de `f'`, inspetor com a equação da tangente.
- [x] Tema claro/escuro.

## v0.2 — Cálculo A ✅
- [x] **Limites e continuidade**: limites laterais, no infinito, tabela de aproximação, as três
  condições de continuidade e o tipo de descontinuidade (removível, salto, infinita, oscilante).
- [x] **Derivadas**: derivação simbólica com cada regra (produto, quociente, cadeia…), `f''`,
  pontos de tangência horizontal (máximo, mínimo, estacionário).
- [x] **Integrais**: integral definida, área com sinal, somas de Riemann (4 métodos, até 200
  retângulos na UI), primitiva pela tabela e Teorema Fundamental do Cálculo.
- [x] Funções por partes: `{x < 1: x^2; x >= 1: 2x}`.
- [x] Erros estruturados do motor (`ApiError` com código, posição e campo) — [ADR 0005](../03-arquitetura/adr/0005-mv-api-dispatch-unico.md).
- [x] Resultados exatos na tela (frações, raízes, múltiplos de π e e).

## v0.3 — Vetores e Geometria Analítica ✅
- [x] **Operações com vetores** em 3D: escalar, vetorial (com o determinante), norma, ângulo,
  projeção, versores, ortogonalidade, paralelismo, produto misto e coplanaridade.
- [x] **Retas e planos**: plano por 3 pontos (normal e equação geral), reta paramétrica,
  distância ponto–plano e ponto–reta, pé da perpendicular.
- [x] **Matrizes**: determinante (2×2, Sarrus, escalonamento), inversa por [A | I], posto,
  escalonamento passo a passo, transposta, traço, autovalores/autovetores, A + B, A·B.
- [x] **Sistemas lineares**: Gauss–Jordan passo a passo, classificação SPD/SPI/SI, solução paramétrica.

## v0.4 — Próximos passos em Cálculo e GA
- [ ] Derivação passo a passo também para `f''` e simplificação algébrica mais forte (fatoração).
- [ ] Integração por substituição e por partes com passos.
- [ ] Cônicas (circunferência, elipse, parábola, hipérbole) e quádricas.
- [ ] Amostragem adaptativa de curvas (hoje: uniforme + heurística de assíntota).
- [ ] Mudança de base e transformações lineares visualizadas no plano.

## v0.5 — Métodos numéricos e física
- [ ] Newton-Raphson e bisseção com iterações animadas.
- [ ] EDOs (Euler, RK4), campo de direções.
- [ ] Cinemática: `s(t)`, `v(t)`, `a(t)` sincronizados.

## v0.6 — Workspaces e desktop
- [ ] Salvar/abrir `.mathview`, exportar PNG/SVG, command palette.
- [ ] Ícones do app desktop e build Tauri verificado no CI.

## Futuro
- Cálculo multivariável (superfícies 3D com WebGL), plugins de módulos.
