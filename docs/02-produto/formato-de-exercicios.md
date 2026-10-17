# Formato de exercícios

Especificação do formato de exercícios, listas e packs da Vértice. A decisão e as alternativas estão
no [ADR 0008](../03-arquitetura/adr/0008-formato-canonico-de-exercicio.md); a correção, no
[ADR 0009](../03-arquitetura/adr/0009-correcao-no-motor-rust.md). As referências de estudo, os
materiais e os roteiros da Biblioteca seguem o [ADR 0014](../03-arquitetura/adr/0014-biblioteca.md)
(§11 e §13). A evidência de verificação das respostas segue o
[ADR 0015](../03-arquitetura/adr/0015-evidencia-de-verificacao.md) (§12).

Há **três representações** do mesmo exercício:

| Representação | Para quem | Onde |
| ------------- | --------- | ---- |
| **Autoria** — `*.ex.md` (Markdown + YAML) | Pessoas: PRs, editor, colaboradores que não programam | `content/`, editor do app |
| **Canônica** — JSON com `schemaVersion` | Máquinas: motor, armazenamento, API, IA | gerada pelo compilador; packs |
| **Pack** — `.vertice.zip` | Distribuição e importação em lote | arquivos do usuário |

A canônica é a fonte da verdade. Os **DTOs Rust em `vx-api`** (`serde`) geram o JSON Schema
(`schemars`) e os tipos TypeScript (`ts-rs`). Os tipos de domínio permanecem puros, com
conversões explícitas. O [ADR 0016](../03-arquitetura/adr/0016-contrato-de-conteudo-f1.md)
define a propriedade dos tipos e o recorte da F1.1: `Part` pertence ao `content`; `AnswerSpec`
pertence ao `assessment` e é consumido por sua API pública. `verify` e `evidence` deste
documento entram na F1.4, não na F1.1; até lá, sua presença é rejeitada explicitamente.

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
  "difficulty": 2,                                  // opcional: inteiro 1 (fixação) … 5 (desafio)
  "estimatedMinutes": 5,                            // opcional: inteiro positivo
  "assets": [ { "id": "grafico-a", "path": "assets/grafico-a.svg", "alt": "Parábola com mínimo em x = 1" } ],
  "source": {
    "kind": "authored",                             // authored | imported | adapted
    "citation": null,                               // ex.: "Lista 04, ex. 9 — MAT01199 UFRGS"
    "license": "CC-BY-SA-4.0",                      // SPDX; "proprietary" para material de terceiros
    "authors": ["Cauã"]
  },
  "references": [                                   // opcional; "Para estudar" (§11)
    {
      "type": "webpage",
      "title": "Regra da cadeia",
      "authors": ["Colaboradores da Wikipédia"],
      "url": "https://pt.wikipedia.org/wiki/Regra_da_cadeia",
      "license": "CC-BY-SA-4.0",
      "locator": null,
      "material": null
    }
  ],
  "visibility": "public",                           // public | private  (private nunca é exportado como público)
  "params": null                                    // reservado para exercícios aleatorizados (F2)
}
```

`difficulty` e `estimatedMinutes` podem estar ausentes ou ser `null`; nesse caso o metadado
não foi informado. Não há valor padrão inferido. Na F1.1, `skills` valida a forma dos
identificadores; sua existência no catálogo da taxonomia é conferida na F1.3.

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
  "points": 1,
  "evidence": [                                     // gerada pelo compilador, nunca escrita à mão (§12)
    { "method": "symbolic", "scope": { "kind": "full" }, "verdict": "confirmed",
      "engineVersion": "0.4.0", "contentHash": "sha256:…", "at": "2026-10-15" }
  ]
}
```

- `solution.engine` (opcional) pede ao motor os passos gerados (ex.: derivada regra por regra) em
  vez de passos escritos à mão. Usa os mesmos comandos do `dispatch` ([ADR 0005](../03-arquitetura/adr/0005-mv-api-dispatch-unico.md)).
- `viz` aponta para o **registro de visualizações**. O app valida `params` contra o schema da
  visualização e gera o link "Abrir no laboratório".
- `solution.engine.request` e `viz.params` transportam JSON técnico, representado no domínio
  por `shared::JsonValue`, sem dependência de serialização. A F1.1 verifica sua estrutura e
  números finitos, não executa comandos nem valida o registro de visualizações (F1.6).
  No round-trip Rust, inteiros `i64`/`u64` preservam valor e representação inteira, sem
  conversão intermediária para `f64`; isso mantém requests como `sampleCount` compatíveis
  com o motor. Floats continuam finitos. Não há promessa de precisão inteira arbitrária
  nem de preservação, em JavaScript `number`, de inteiros além de sua faixa exata.
- `evidence` (opcional, gerada) registra **como** a resposta esperada foi conferida, com que
  escopo e com qual versão do motor. Ver §12.

## 3. Tipos de resposta (`AnswerSpec`)

Todos aceitam `feedback` (erros comuns, ver §4). Números aceitam vírgula decimal, frações e
constantes (`1/2`, `sqrt(3)`, `pi/4`), como o resto do app.

| `type` | Campos | Corrige como | Exemplo de lista |
| ------ | ------ | ------------ | ---------------- |
| `numeric` | `value` (expressão constante), `tolerance {abs?, rel?}`, `unit?` | Avalia o valor exato e compara com tolerância | L04 ex. 1: `3`; ex. 6a: `6400*sqrt(10)` |
| `expression` | `expected`, `variables`, `domain?`, `form?` | Avalia nos pontos de amostragem (§5); `form` exige forma (ex.: `factored`, `expanded`) | L04 ex. 17: `-2/x^3 * cos(1/x^2)` |
| `equation` | `expected` (`lhs = rhs`), `variables` | `lhs − rhs` do estudante proporcional ao esperado (constante ≠ 0) | L04 ex. 3: `y = 5x - 16` |
| `set` | `values: ScalarAnswerSpec[]`, `ordered: false` | Correspondência um a um, sem reutilizar elementos | L04 ex. 14: `{-1, 0, 1}` |
| `tuple` | `values: ScalarAnswerSpec[]` (ponto, vetor) | Componente a componente, em ordem | VGA: vetor normal `(1, -2, 3)` |
| `matrix` | `rows: ScalarAnswerSpec[][]` | Elemento a elemento | VGA: inversa de A |
| `choice` | `options[] {id, text}`, `correct[]`, `multiple` | Igualdade de conjunto de ids | Objetivas |
| `matching` | `left[]`, `right[]`, `pairs {left: right}`, `partialCredit?` (booleano, padrão `false`) | Cada par; política de pontuação parcial ainda depende do PRD | L04 ex. 5: A–4, B–2, C–1, D–3 |
| `self_graded` | `rubric?` | Estudante compara com a solução e se autoavalia | "Esboce o gráfico de f" |
| `code` *(F2)* | `language`, `starter`, `tests[]` | Executa testes em sandbox | POO, algoritmos |
| `sql` *(F2)* | `schema`, `seed`, `expectedResult` | Compara o conjunto resultado | Banco de dados |

`ScalarAnswerSpec` é a união não recursiva de `numeric` e `expression`, com os mesmos campos
das variantes acima. Por exemplo, um elemento é `{ "type": "numeric", "value": "1/2" }`,
nunca apenas `"1/2"`. Cada expressão declara suas variáveis e restrições; cada elemento usa
sua própria tolerância. Coleções aninhadas e demais tipos como elementos são rejeitados.
`set` permite vazio e exige `ordered: false`; `tuple` não é vazia; `matrix` tem pelo menos
uma linha e uma coluna, com linhas de mesmo tamanho.

`expression.form` aceita `factored`, `expanded` e `no_negative_exponents`. A F1.1 apenas
valida o contrato; a checagem da forma matemática pertence à F1.2. `matching.partialCredit`
é somente preferência de autoria na F1.1: aceitar `true` não habilita pontuação parcial nem
define seu algoritmo. Fórmula, pesos e efeitos no veredito aguardam a decisão de produto.

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
- `samples`: inteiro de pelo menos 12, padrão 12 pontos pseudoaleatórios com semente fixa por
  exercício (resultado reprodutível). A F1.1 valida o campo; a F1.2 executa a amostragem.
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
`viz:`, `feedback:` e `verify:` são YAML.

```markdown
---
id: 0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90
title: Reta tangente a uma função racional
skills: [calculo.derivadas.regra-do-quociente, calculo.derivadas.reta-tangente]
difficulty: 2
source: { kind: authored, license: CC-BY-SA-4.0, authors: [Cauã] }
references:
  - { type: webpage, title: Regra do quociente, url: "https://pt.wikipedia.org/wiki/Regra_do_quociente", license: CC-BY-SA-4.0 }
---
Considere $f(x) = \dfrac{-x}{x^2+1}$.

::: part a
Determine a inclinação da reta tangente em $x = 1$.
answer: { type: numeric, value: "0" }
hints:
  - A inclinação da tangente é $f'(1)$.
viz: { id: calculus.derivative, params: { f: "-x/(x^2+1)", x0: 1 } }
verify: { command: calculus.tangent_line, request: { expression: "-x/(x^2+1)", x0: "1" }, result: /slope }
:::

::: part b
Escreva a equação da reta tangente nesse ponto.
answer: { type: equation, expected: "y = -1/2", variables: [x, y] }
verify: { human: { by: Cauã, against: "cálculo à mão", at: 2026-10-15 } }
:::
```

O compilador (`vertice content build`, F1) valida o schema, gera o JSON canônico e roda o corretor
com a própria resposta esperada (ela precisa se corrigir como "correta"). Quando a parte tem
`verify:`, ele também confere a resposta de forma independente e grava `evidence` (§12). Os nomes
de comando e de campo no `verify:` acima são ilustrativos; valem os do contrato do `dispatch`.

## 8. Pack (`.vertice.zip`)

```
manifest.json          metadados do pack (objeto abaixo)
exercises/*.json | *.ex.md
lists/*.json
assets/                imagens (PNG, JPEG, WebP, SVG sanitizado)
gabarito.json          opcional: { "<id ou número>": { "<parte>": AnswerSpec } }
gabarito.csv           opcional: item,parte,tipo,resposta
```

```json
{
  "schemaVersion": 1,
  "id": "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e91",
  "name": "Exercícios autorais de derivadas",
  "license": "CC-BY-SA-4.0",
  "visibility": "public",
  "lists": []
}
```

`id` é UUIDv7 obrigatório e permanece em reexportações; `name` não é identidade.
UUIDs aceitam letras hexadecimais maiúsculas ou minúsculas, mas unicidade e resolução de
referências ignoram essa diferença. A grafia recebida pode ser preservada; rótulos de partes
e números editoriais não ganham essa equivalência.
`description` é opcional. `lists` contém IDs UUIDv7 ordenados, sem duplicatas, de listas
incluídas no pack, não caminhos ou objetos embutidos; vazio permite exercícios avulsos.
IDs de exercícios e listas vêm dos documentos, não dos nomes de arquivo. As listas devem
referenciar exercícios e partes incluídos. O manifesto não sobrepõe licença ou visibilidade
dos documentos: exportação pública rejeita conteúdo privado ou incompatível, mesmo que o
manifesto diga `public`. Ver [ADR 0016](../03-arquitetura/adr/0016-contrato-de-conteudo-f1.md).

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

## 11. Referências (`references[]`, F1)

Lista opcional de material para estudar o assunto do exercício. O app mostra a lista como "Para
estudar". Na F1 cada referência é **só metadados e link**: o app não baixa, não guarda e não anexa
arquivo ([ADR 0014](../03-arquitetura/adr/0014-biblioteca.md)).

```jsonc
{
  "type": "book",                 // paper | book | notes | video | webpage
  "title": "Calculus Volume 1 — 3.6 The Chain Rule",
  "authors": ["Gilbert Strang", "Edwin Herman"],
  "url": "https://openstax.org/books/calculus-volume-1/pages/3-6-the-chain-rule",
  "doi": null,                    // ex.: "10.1000/xyz123"
  "arxiv": null,                  // ex.: "2401.01234v2"
  "isbn": null,
  "locator": "seção 3.6",         // capítulo, seção, páginas ou "12:30–15:00" em vídeo
  "license": "CC-BY-NC-SA-4.0",   // SPDX da edição citada, "proprietary" ou "unknown"
  "material": null                // F2: UUID de um Material da Biblioteca, quando existir
}
```

Regras, validadas pelo compilador com erro por campo:
- `title`, `type` e `license` são obrigatórios. Pelo menos um entre `url`, `doi`, `arxiv` e `isbn`
  também.
- `url` só aceita `https:` (ou `http:`). `javascript:`, `data:` e `file:` são rejeitados.
- A F1.1 aceita um subconjunto conservador: host ASCII/punycode ou IP válido, porta opcional
  numérica de 0 a 65535, sem credenciais, espaços literais, barras invertidas ou controles.
  Escapes percentuais incompletos ou de controles ASCII são rejeitados. Hosts Unicode devem
  usar punycode. HTTP e hosts locais são permitidos; validar o link não faz consulta DNS,
  download ou proteção de SSRF para um futuro consumidor de rede.
- Não existe campo de arquivo. Referência a material de terceiros é sempre link (ADR 0012).
- A licença é a da **edição** citada. Vários livros abertos mudam de licença entre edições ou
  títulos; os volumes de Cálculo da OpenStax, por exemplo, são CC BY-NC-SA 4.0.
- Identificadores de licença não vazios são preservados, sem validação contra catálogo SPDX
  completo. Referências aceitam licenças restritivas. Para publicar os documentos, a lista
  positiva inicial é `CC-BY-SA-4.0`, `CC-BY-4.0` e `CC0-1.0`; autoria pública no banco exige
  `CC-BY-SA-4.0`. Outra licença não recebe compatibilidade presumida e exige revisão explícita.
- Referências não mudam o veredito nem o Progresso.
- Campo opcional e aditivo: não muda `schemaVersion`.
- Na F2, `material` liga a referência a um item da Biblioteca. Os metadados continuam na
  referência, para ela funcionar sem a Biblioteca.
- Na F1, `material` fica ausente ou `null`; vínculos não nulos são rejeitados até F2.
  `locator` é texto opcional, não o futuro `Locator` estruturado de `shared`.

## 12. Evidência de verificação (`verify` e `evidence`)

Decisão no [ADR 0015](../03-arquitetura/adr/0015-evidencia-de-verificacao.md). Na autoria, cada
parte pode dizer **como** conferir a resposta esperada de forma independente. O compilador
executa e grava o resultado no canônico.

| Autoria (`verify:`) | O que o compilador faz |
| ------------------- | ---------------------- |
| `verify: solution` | Roda `solution.engine` e compara o resultado principal do comando com `answer` |
| `verify: { command, request, result }` | Roda o comando do `dispatch` e compara o valor em `result` (JSON Pointer) com `answer` |
| `verify: { human: { by, against, at } }` | Registra a conferência humana (quem, contra o quê, quando) |

Canônico (`parts[].evidence[]`):

| Campo | Valores |
| ----- | ------- |
| `method` | `symbolic` (coincidência exata após simplificação, ou valor exato) · `computed` (valor numérico calculado pelo motor, com tolerância) · `sampled` (equivalência por amostragem, §5) · `human` |
| `scope` | `{ kind: "full" }` ou `{ kind: "restricted", domain?, note }`. Em `sampled`, `domain` é obrigatório |
| `verdict` | `confirmed` · `mismatch` · `indeterminate` |
| `engineVersion` | Versão do núcleo que gerou a evidência (ausente em `human`) |
| `contentHash` | `sha256` do canônico da revisão, sem o campo `evidence` |
| `seed`, `samples` | Em `sampled`, para reproduzir |
| `by`, `against`, `at` | Em `human`: quem, contra qual fonte e quando. `at` também data as demais |

- O autor **não** escreve `evidence` no `*.ex.md`; o compilador rejeita.
- `choice`, `matching` e `self_graded` só aceitam `human`.
- O selo "verificado pelo motor" exige evidência de motor `confirmed` em toda parte corrigível,
  para o `contentHash` atual. Ele some quando a revisão muda ou quando a reverificação com outra
  versão do motor falha.
- No repositório, `content/evidence.lock.json` guarda a evidência de todo o banco público. A CI
  falha se ele estiver desatualizado ou se algum veredito mudar sem mudança de conteúdo.

## 13. Biblioteca: materiais e roteiros (F2)

Decisão no [ADR 0014](../03-arquitetura/adr/0014-biblioteca.md). Ficam em `content/library/` e
são compilados pelo mesmo `vertice content build`.

| Arquivo | Conteúdo |
| ------- | -------- |
| `content/library/materials/**/*.mat.md` | Um material: frontmatter com metadados; corpo com a nota de aula (`hosting: inline`) ou um resumo autoral |
| `content/library/guides/**/*.guide.md` | Um roteiro de leitura: frontmatter + blocos `::: step <id>` |
| `content/library/collections/*.yaml` | Coleções ("famílias" e edições) |

### 13.1 Material (`*.mat.md`)

```markdown
---
id: 0192f8a1-2b3c-7d4e-8f90-a1b2c3d4e5f6
revision: 1
kind: webpage                  # paper | book | notes | document | video | webpage
title: Regra da cadeia
locale: pt-BR
citation:                      # CSL-JSON em YAML
  author: [{ literal: Colaboradores da Wikipédia }]
  issued: { date-parts: [[2026, 10, 15]] }
  URL: https://pt.wikipedia.org/wiki/Regra_da_cadeia
arxiv: null
origin: open                   # authored | open | third_party
hosting: link                  # link | inline | file
rights:
  license: CC-BY-SA-4.0        # SPDX da edição/versão, "proprietary" ou "unknown"
  url: https://creativecommons.org/licenses/by-sa/4.0/
  checkedAt: 2026-10-15
  aiUse: unknown               # allowed | forbidden | unknown
skills: [calculo.derivadas.regra-da-cadeia]
sections:                      # opcional: habilidades por trecho
  - { locator: { section: "Demonstração" }, skills: [calculo.derivadas.regra-da-cadeia] }
aiGenerated: false
review: null                   # { by, at } dá o selo "revisado"
errata: []                     # só acréscimos: { at, kind: correction | retraction, note, revision?, reason?, replacedBy? }
---
Resumo autoral: o que o artigo cobre, para quem serve e que trecho ler primeiro.
```

Regras (política do ADR 0014, §4):
- Em `content/`, `origin: open | third_party` exige `hosting: link`. `inline` e `file` só valem
  para `authored` com `CC-BY-SA-4.0`.
- `skills` precisa de pelo menos uma habilidade existente na taxonomia.
- Resumos de editora não são copiados; o corpo é autoral.
- `revision` precisa aumentar quando o canônico muda.
- `errata` só recebe acréscimos. A CI compara com a base do PR.
- Status e selos são derivados, nunca escritos.
- Cada edição com licença própria é um material separado, agrupado numa coleção `editions`.

### 13.2 Roteiro de leitura (`*.guide.md`)

Frontmatter com os metadados do roteiro; corpo com texto de abertura e **blocos de etapa**
(`::: step <id>`, com `id` estável em `kebab-case`). Dentro de cada etapa, o texto é a orientação
ao estudante, e as linhas `read:`, `questions:`, `checks:`, `exercises:` e `viz:` são YAML. Uma
etapa pode combinar várias delas.

```markdown
---
id: 0192f8b0-4c5d-7e6f-8a9b-c0d1e2f3a4b5
revision: 1
title: Regra da cadeia, do conceito à prática
summary: Por que a derivada de $f(g(x))$ é $f'(g(x))\,g'(x)$, com leitura curta, gráfico e exercícios.
locale: pt-BR
skills: [calculo.derivadas.regra-da-cadeia]
prerequisites:
  skills: [calculo.derivadas.definicao]
  guides: []
estimatedMinutes: 40
source: { kind: authored, license: CC-BY-SA-4.0, authors: [Cauã] }
aiGenerated: false
review: null
errata: []
---
Neste roteiro você vai entender a regra da cadeia lendo, vendo e praticando.

::: step ler
Leia a seção sobre a regra da cadeia e preste atenção no exemplo com $\sin(x^2)$.
read: { material: 0192f8a1-2b3c-7d4e-8f90-a1b2c3d4e5f6, locator: { section: "Demonstração" } }
questions:
  - Por que aparece o fator $2x$ na derivada de $\sin(x^2)$?
:::

::: step ver
Mova o ponto e compare a inclinação com $2x\cos(x^2)$.
viz: { id: calculus.derivative, params: { f: "sin(x^2)", x0: 1 } }
:::

::: step checar
checks:
  - label: a
    prompt: Derive $g(x) = \cos(3x)$.
    answer: { type: expression, expected: "-3*sin(3*x)", variables: [x] }
    verify: { command: calculus.differentiate, request: { expression: "cos(3*x)" }, result: /derivative }
    onMiss: { step: ler }
:::

::: step praticar
exercises:
  - { exercise: 0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90, parts: [a, b] }
:::
```

Regras:
- `read.material` aponta para um material existente. No banco público, ele precisa ser público.
  Material só local do usuário (roteiros privados) aparece como "não disponível aqui" em outro
  dispositivo.
- `checks[]` usa o mesmo tipo `Part` do exercício (§2–§5), corrigido pelo `assessment.check`, com
  `verify` e `evidence` (§12). `onMiss` aponta para uma etapa (`step`) ou uma habilidade (`skill`)
  a rever. Checagens são formativas e não contam como domínio no Progresso.
- `questions` são abertas (sem correção) e servem para reflexão.
- `exercises[]` referencia exercícios existentes, opcionalmente com as partes.
- `viz` é validado contra o registro de visualizações, como no exercício.
- Em `content/`, o roteiro é `authored` e `CC-BY-SA-4.0`. Material de terceiros entra só por
  `read` com link.

### 13.3 Coleção (`content/library/collections/*.yaml`)

```yaml
id: 0192f8c0-…
kind: family                 # family | editions
title: Derivadas — regras de derivação
items:
  - { ref: guide:0192f8b0-4c5d-7e6f-8a9b-c0d1e2f3a4b5, role: main }
  - { ref: material:0192f8a1-2b3c-7d4e-8f90-a1b2c3d4e5f6, role: companion }
  - { ref: exercise:0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90, role: prerequisite }
```

`role`: `main` · `companion` · `prerequisite` · `alternative` · `edition`.
