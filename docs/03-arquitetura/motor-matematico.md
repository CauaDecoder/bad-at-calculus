# Motor matemático

Como o texto digitado vira números, gráficos e resoluções passo a passo.

```
"x^2 - 3x"  ──tokenizer──▶  tokens  ──Pratt parser──▶  Expr (AST)
                                                         │
     ┌──────────────┬──────────────┬─────────────────┬───┴──────────┬──────────────┐
     ▼              ▼              ▼                 ▼              ▼              ▼
 evaluate_at    to_latex      differentiate     analyze_limit   integrate     antiderivative
 (amostras)     (KaTeX)       (+ passos)        (laterais)      (Simpson)     (tabela)
```

## 1. Expressões — `mv_domain::expression`

Árvore imutável: `Number`, `Constant` (π, e), `Variable`, `Negate`, `Binary`, `Call` e
`Piecewise` (função por partes). Funções: `sin cos tan ln log exp sqrt cbrt abs`.

- `evaluate_at(variável, valor)` — resultados não finitos viram `DomainError` (a curva "quebra").
- `evaluate_constant()` — usada para números digitados em campos (`sqrt(2)/2`, `pi/3`).
- `simplified()` — regras pequenas e previsíveis: dobra constantes, mantém frações exatas e
  reduzidas (`6/4 → 3/2`), remove `+0`, `·1`, `^1`, junta potências (`x²·x → x³`), leva
  coeficientes para a frente e funções para o fim (`cos(x)·3x → 3x cos(x)`).
- `to_latex()` — só os parênteses necessários; justapõe como no papel (`2x`, `x cos(x)`) e usa
  `\cdot` só onde seria ambíguo (`2 \cdot 3`).

## 2. Parser — `mv_infra::parser::PrattParser`

Pratt parser escrito à mão (mensagens de erro com posição).

| Operador | Poder de ligação | Associatividade |
| -------- | ---------------- | --------------- |
| `+ -` | 10 | esquerda |
| `* /` e implícita (`2x`, `3(x+1)`) | 20 | esquerda |
| `-` unário, função sem parênteses (`sin x`) | 30 | prefixo |
| `^`, `²`, `³` | 40 | **direita** |

| Entrada | Interpretação |
| ------- | ------------- |
| `2x`, `(x+1)(x-1)` | multiplicação implícita |
| `sinx`, `sin x^2`, `sin(x)^2` | `sin(x)`, `sin(x²)`, `(sin x)²` |
| `|x - 1|`, `√x`, `π`, `e^x` | módulo, raiz, constantes |
| `2,5` | vírgula decimal |
| `f(x) = …`, `y = …` | prefixo ignorado |
| `{x < 1: x^2; x >= 1: 2x}` | função por partes (separe ramos com `;`) |
| `{-1 <= x < 2: 1; 0}` | condições encadeadas e ramo "senão" |

Limitação conhecida: `sin 2x` é lido como `sin(2)·x`; use `sin(2x)`.

## 3. Derivadas — `calculus::differentiation`

`differentiate(expr, "x")` aplica as regras de cima para baixo e **registra cada passo**
(`DerivationStep { rule, uses_chain_rule, expression, derivative }`): constante, soma, diferença,
constante multiplicando, produto, quociente, potência, exponencial (`a^u`, `e^u`), potência geral
(`u^v`), logaritmos, trigonométricas, raízes, módulo e funções por partes. A regra da cadeia é
marcada quando a função interna não é o próprio `x`. Cada resultado passa por `simplified()`.

`derivative_at` (diferença central, `h = 10⁻⁵`) continua sendo usado para amostrar `f'` no gráfico
e para achar **pontos críticos** (varredura de sinal de `f'` + bisseção; classifica máximo, mínimo
ou ponto estacionário).

## 4. Limites e continuidade — `calculus::limits`

Todo limite vira `t → 0⁺`: `x = a ± t`, `x = ±1/t` (infinito). Três estimativas, nesta ordem:

1. **Extrapolação de Richardson** em `t = 0,1/2ⁱ` — excelente para funções suaves
   (`sin(x)/x`, `(1 − cos x)/x²`), aceita só se concordar com a tendência dos valores diretos
   (evita respostas erradas em casos como `√x`);
2. **Δ² de Aitken** nos valores em `t = 10⁻ᵏ` — convergência geométrica (`√t`);
3. **Valor direto** bem perto do ponto — último recurso (sofre com cancelamento).

Se nada converge: sequência monótona que não desacelera → `±∞`; senão → não existe (oscila).
Resultados perto de inteiros são arredondados (`snap`). A continuidade em `a` verifica as três
condições e classifica a descontinuidade: **removível, salto, infinita ou oscilante**. A tabela
de aproximação usa `t = 0,1; 0,01; 0,001; 0,0001`.

## 5. Integrais — `calculus::integration` e `calculus::antiderivative`

- **Integral definida:** Simpson adaptativo (tolerância `10⁻¹⁰`, profundidade 40). Se `f` não
  estiver definida no intervalo ou a integral divergir → `NotIntegrable`. Limites invertidos
  trocam o sinal.
- **Somas de Riemann:** esquerda, direita, ponto médio e trapézio (até 1000 retângulos),
  devolvendo cada retângulo para o desenho.
- **Primitivas:** tabela de integrais imediatas + linearidade + substituição linear
  (`sin(3x+1)`, `(2x−5)⁴`, `1/x → ln|x|`). Toda primitiva é **conferida derivando-a**
  numericamente em alguns pontos; se não bater, não é mostrada.

## 6. Vetores e geometria — `vectors`, `geometry`

`Vector3` (2D = `z = 0`): soma, escalar, vetorial, norma, versor, ângulo, projeção, ortogonalidade,
paralelismo, produto misto. `Plane` (por 3 pontos ou ponto + normal) e `Line3` (por 2 pontos):
distância e pé da perpendicular. Tolerância geométrica: `10⁻⁹`.

## 7. Álgebra linear — `linear_algebra`

- **Gauss–Jordan com passos** (`row_reduce`): cada operação elementar (`Lᵢ ↔ Lⱼ`,
  `Lᵢ ← kLᵢ`, `Lᵢ ← Lᵢ + kLⱼ`) é registrada com a matriz resultante. O pivô prefere entradas
  `±1` (menos frações, como no caderno).
- **Determinante** a partir das operações (troca inverte o sinal; `Lᵢ ← kLᵢ` divide por `k`).
  A UI também mostra a fórmula 2×2 e a regra de Sarrus 3×3 com os números.
- **Inversa** escalonando `[A | I]`; **sistemas** escalonando `[A | b]` e classificando em
  SPD/SPI/SI pelos postos, com solução paramétrica no SPI.
- **Autovalores** via decomposição de Schur da `nalgebra` ([ADR 0004](adr/0004-bibliotecas-matematicas-no-dominio.md));
  autovetores = base do núcleo de `A − λI`; polinômio característico por Faddeev–LeVerrier.

## 8. Precisão numérica

- `f64` em todo o motor; comparações sempre com tolerância nomeada (`TOLERANCE`, `ZERO_TOLERANCE`).
- A exibição recupera formas exatas no front (`exact-format.ts`): frações, `√`, múltiplos de π e e.
- Sem números mágicos: toda constante tem nome e comentário.
