# Formato de exercícios

Especificação do formato de exercícios, listas e packs da Vértice. A decisão e as alternativas estão
no [ADR 0008](../03-arquitetura/adr/0008-formato-canonico-de-exercicio.md); a correção, no
[ADR 0009](../03-arquitetura/adr/0009-correcao-no-motor-rust.md).

Há **três representações** do mesmo exercício:

| Representação | Para quem | Onde |
| ------------- | --------- | ---- |
| **Autoria** — `*.ex.md` (Markdown + YAML) | Pessoas: PRs, editor, colaboradores que não programam | `content/`, editor do app |
| **Canônica** — JSON com `schemaVersion` | Máquinas: motor, armazenamento, API, IA | gerada pelo compilador; packs |
| **Pack** — `.vertice.zip` | Distribuição e importação em lote | arquivos do usuário |

A canônica é a fonte da verdade. Os **tipos Rust** (serde) geram o JSON Schema (`schemars`) e os
tipos TypeScript (`ts-rs`), então o contrato só é escrito uma vez.

## 1. Exercício (canônico)

```jsonc
{
  "schemaVersion": 1,
  "id": "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90",   // UUIDv7: ordenável por criação
  "revision": 3,                                    // incrementa a cada edição publicada
  "locale": "pt-BR",
  "title": "Reta tangente a uma função racional",
  "statement": "Considere $f(x) = \\dfrac{-x}{x^2+1}$.",   // Markdown + LaTeX
  "parts": [ /* Part[] — ao menos 1 */ ],
  "skills": ["calculo.derivadas.regra-do-quociente", "calculo.derivadas.reta-tangente"],
  "difficulty": 2,                                  // 1 (fixação) … 5 (desafio)
  "estimatedMinutes": 5,
  "assets": [ { "id": "grafico-a", "path": "assets/grafico-a.svg", "alt": "Parábola com mínimo em x = 1" } ],
  "source": {
    "kind": "authored",                             // authored | imported | adapted
    "citation": null,                               // ex.: "Lista 04, ex. 9 — MAT01199 UFRGS"
    "license": "CC-BY-SA-4.0",                      // SPDX; "proprietary" para material de terceiros
    "authors": ["Cauã"]
  },
  "visibility": "public",                           // public | private  (private nunca é exportado como público)
  "params": null                                    // reservado para exercícios aleatorizados (F2)
}
```

## 2. Parte

```jsonc
{
  "label": "a",
  "prompt": "Determine a inclinação da reta tangente em $x = 1$.",
  "answer": { /* AnswerSpec */ },
  "hints": [
    "A inclinação da tangente é $f'(1)$.",
    "Use a regra do quociente: $(u/v)' = (u'v - uv')/v^2$."
  ],
  "solution": {
    "steps": ["$f'(x) = \\dfrac{x^2 - 1}{(x^2+1)^2}$", "$f'(1) = 0$"],
    "engine": { "command": "calculus.differentiate", "request": { "expression": "-x/(x^2+1)" } }
  },
  "viz": { "id": "calculus.derivative", "params": { "f": "-x/(x^2+1)", "x0": 1 } },
  "points": 1
}
```

- `solution.engine` (opcional) pede ao motor os passos gerados (ex.: derivada regra por regra) em
  vez de passos escritos à mão. Usa os mesmos comandos do `dispatch` ([ADR 0005](../03-arquitetura/adr/0005-mv-api-dispatch-unico.md)).
- `viz` aponta para o **registro de visualizações**. O app valida `params` contra o schema da
  visualização e gera o link "Abrir no laboratório".

## 3. Tipos de resposta (`AnswerSpec`)

Todos aceitam `feedback` (erros comuns, ver §4). Números aceitam vírgula decimal, frações e
constantes (`1/2`, `sqrt(3)`, `pi/4`), como o resto do app.

| `type` | Campos | Corrige como | Exemplo de lista |
| ------ | ------ | ------------ | ---------------- |
| `numeric` | `value` (expressão constante), `tolerance {abs?, rel?}`, `unit?` | Avalia o valor exato e compara com tolerância | L04 ex. 1: `3`; ex. 6a: `6400*sqrt(10)` |
| `expression` | `expected`, `variables`, `domain?`, `form?` | Avalia nos pontos de amostragem (§5); `form` exige forma (ex.: `factored`, `expanded`) | L04 ex. 17: `-2/x^3 * cos(1/x^2)` |
| `equation` | `expected` (`lhs = rhs`), `variables` | `lhs − rhs` do estudante proporcional ao esperado (constante ≠ 0) | L04 ex. 3: `y = 5x - 16` |
| `set` | `values[]`, `ordered: false` | Correspondência um a um, cada elemento como `numeric`/`expression` | L04 ex. 14: `{-1, 0, 1}` |
| `tuple` | `values[]` (ponto, vetor) | Componente a componente | VGA: vetor normal `(1, -2, 3)` |
| `matrix` | `rows[][]` | Elemento a elemento | VGA: inversa de A |
| `choice` | `options[] {id, text}`, `correct[]`, `multiple` | Igualdade de conjunto de ids | Objetivas |
| `matching` | `left[]`, `right[]`, `pairs {left: right}` | Cada par; pontuação parcial opcional | L04 ex. 5: A–4, B–2, C–1, D–3 |
| `self_graded` | `rubric?` | Estudante compara com a solução e se autoavalia | "Esboce o gráfico de f" |
| `code` *(F2)* | `language`, `starter`, `tests[]` | Executa testes em sandbox | POO, algoritmos |
| `sql` *(F2)* | `schema`, `seed`, `expectedResult` | Compara o conjunto resultado | Banco de dados |

Partes com várias respostas (L04 ex. 4: `f(−2)` **e** `f'(−2)`) viram **duas partes** ou um `tuple`
com rótulos — prefira partes, porque o feedback fica por resposta.

## 4. Feedback por erro comum

Inspirado nas árvores de resposta do STACK. Cada `AnswerSpec` pode listar respostas erradas
previsíveis com uma mensagem:

```jsonc
"feedback": [
  { "matches": "-2/x^3 * sin(1/x^2)", "message": "Quase: a derivada de $\\sin u$ é $\\cos u \\cdot u'$." },
  { "matches": "cos(1/x^2)",          "message": "Faltou a regra da cadeia: multiplique pela derivada de $1/x^2$." }
]
```

`matches` é corrigido com o **mesmo** algoritmo da resposta certa (equivalência, não texto).

## 5. Amostragem (expressões e equações)

- `domain`: intervalos por variável (`{"x": [-5, 5]}`); padrão `[-10, 10]`, excluindo pontos onde
  a expressão esperada não é finita.
- `samples`: padrão 12 pontos pseudoaleatórios com semente fixa por exercício (resultado reprodutível).
- `tolerance`: padrão relativo `1e-9`, absoluto `1e-12`.
- Detalhes, limitações (ex.: funções que coincidem só em pontos isolados) e testes: ADR 0009.

## 6. Lista (coleção)

```jsonc
{
  "schemaVersion": 1,
  "id": "0192f7a4-…",
  "title": "Lista 04 — Derivadas",
  "description": "Reta tangente, regras de derivação e regra da cadeia.",
  "items": [
    { "exercise": "0192f7a4-…", "number": "1" },
    { "exercise": "0192f7a4-…", "number": "6", "parts": ["a", "b", "c"] }
  ],
  "source": { "kind": "imported", "citation": "MAT01199 — Cálculo A, UFRGS", "license": "proprietary" },
  "visibility": "private"
}
```

## 7. Formato de autoria (`*.ex.md`)

Frontmatter YAML com os metadados; corpo em Markdown com **blocos de parte** (`::: part <rótulo>`).
Dentro de cada bloco, o texto é o enunciado da parte e as linhas `answer:`, `hints:`, `solution:`,
`viz:` e `feedback:` são YAML.

```markdown
---
id: 0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90
title: Reta tangente a uma função racional
skills: [calculo.derivadas.regra-do-quociente, calculo.derivadas.reta-tangente]
difficulty: 2
source: { kind: authored, license: CC-BY-SA-4.0, authors: [Cauã] }
---
Considere $f(x) = \dfrac{-x}{x^2+1}$.

::: part a
Determine a inclinação da reta tangente em $x = 1$.
answer: { type: numeric, value: "0" }
hints:
  - A inclinação da tangente é $f'(1)$.
viz: { id: calculus.derivative, params: { f: "-x/(x^2+1)", x0: 1 } }
:::

::: part b
Escreva a equação da reta tangente nesse ponto.
answer: { type: equation, expected: "y = -1/2", variables: [x, y] }
:::
```

O compilador (`vertice content build`, F1) valida o schema, gera o JSON canônico e roda o corretor
com a própria resposta esperada (ela precisa se corrigir como "correta").

## 8. Pack (`.vertice.zip`)

```
manifest.json          { "schemaVersion": 1, "name", "description", "license", "visibility", "lists": [...] }
exercises/*.json | *.ex.md
lists/*.json
assets/                imagens (PNG, JPEG, WebP, SVG sanitizado)
gabarito.json          opcional: { "<id ou número>": { "<parte>": AnswerSpec } }
gabarito.csv           opcional: item,parte,tipo,resposta
```

Limites de importação (ver [modelo de ameaças](../06-seguranca/modelo-de-ameacas.md)): 50 MB
descompactado, 2.000 arquivos, 5 MB por imagem, caminhos relativos sem `..`, sem links simbólicos,
taxa de compressão máxima 100:1.

## 9. Versionamento

- `schemaVersion` muda só com quebra de compatibilidade; o app migra versões antigas ao importar.
- `revision` muda a cada edição publicada do exercício. Tentativas guardam `exerciseId + revision`,
  então o histórico continua válido depois de uma correção no enunciado.
- Visualizações têm versão própria no registro (`viz.id@versão`), como os widgets do Perseus.

## 10. Exportação para outros formatos
- **QTI 3.0** (XML, 1EdTech) — exportador na F4 para LMS.
- **Moodle GIFT/XML** — avaliar na F4.
