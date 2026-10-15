# Casos de uso — Cálculo A

As listas de MAT01199 — Cálculo A (IME/UFRGS) são o **critério de aceitação real** do módulo de
cálculo: se um estudante consegue resolver/visualizar os exercícios no Laboratório, o módulo está
cumprindo seu papel. Vários exemplos clicáveis das páginas vêm daqui, e os testes do domínio usam
estas funções (as funções em si, não o texto das listas).

Os PDFs são material de terceiros e ficam em `content-private/calculo/`, **fora do git**. Na F1
eles viram um pack privado, e a Lista 04 é o critério de aceite dos Exercícios
([roadmap](roadmap.md#f1--exercícios-offline-mvp), [ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).

Legenda: ✅ disponível · 🟡 parcial · 🔜 planejado

## Lista 01 — Funções
| Exercício típico | Onde / status |
| ---------------- | ------------- |
| Equação da reta por ponto e coeficiente angular | 🟡 Derivadas mostra `y = mx + b` da tangente; ferramenta "reta" dedicada 🔜 |
| Reta paralela/perpendicular a `2x + y = 4` | 🔜 |
| Domínio de `1/(x−3)`, `√(2x+5)` | 🟡 o gráfico já mostra onde f não existe; análise de domínio 🔜 |
| Composição e inversa | 🔜 |

## Lista 02 — Limites
| Exercício típico | Onde / status |
| ---------------- | ------------- |
| `lim x→3⁻ f(x)` de funções por partes | ✅ Limites — digite `{x < 3: …; x >= 3: …}` |
| Limites laterais diferentes (salto) | ✅ Limites — exemplo "salto" |
| Limites no infinito | ✅ Limites — `x → +∞` / `x → −∞`, assíntota horizontal desenhada |
| Esboçar `f` com propriedades dadas | 🔜 modo "esboço" |

## Lista 03 — Continuidade
| Exercício típico | Onde / status |
| ---------------- | ------------- |
| Continuidade em um ponto | ✅ Limites — as 3 condições com ✓/✗ |
| Assíntotas de `(x²−2x−3)/(x²−9)` (ex. 3) | ✅ Limites — exemplos "removível (L3)" em x = 3 e "assíntota (L3)" em x = −3 |
| Tipo de descontinuidade | ✅ removível, salto, infinita, oscilante |

## Lista 04 + Estudo dirigido — Derivadas
| Exercício típico | Onde / status |
| ---------------- | ------------- |
| Derivada por definição (limite `h → 0`) | 🟡 definição mostrada; animação da secante 🔜 |
| Tangente horizontal de `x³ − 3x + 4` (ED ex. 5) | ✅ Derivadas — "Tangentes horizontais" lista x = ±1 |
| `−x/(x²+1)`: inclinação e reta tangente em x = 1 (ex. 9) | ✅ Derivadas — exemplo pronto |
| `(x³+2)⁴ cos x` (ex. 12), `sin x − cos x` (ex. 13) | ✅ Derivadas — passos com produto e cadeia |
| `x² √(5 − x²)` (ex. 11) | ✅ Derivadas — exemplo pronto |
| `d²y/dx²` (ex. 8) | ✅ Derivadas — `f''(x)` |
| `|x|` não derivável em 0 (ED ex. 4) | 🟡 o gráfico de `f'` mostra o salto; aviso explícito 🔜 |
| Combinar gráficos de `f` e `f'` (ex. 5) | 🟡 `f'` tracejada no mesmo gráfico; modo quiz 🔜 |
| Foguete `s(t) = t³/10` (ex. 6) | 🟡 digite `x^3/10`; módulo de física 🔜 |

## Integrais (próxima lista)
| Exercício típico | Onde / status |
| ---------------- | ------------- |
| Integral definida por primitiva (TFC) | ✅ Integrais — primitiva, `F(b) − F(a)` |
| Área com sinal / área entre curva e eixo | ✅ Integrais — área azul (+) e rosa (−) |
| Somas de Riemann | ✅ Integrais — 4 métodos, erro em relação à integral |
| Substituição e integração por partes | 🔜 |
