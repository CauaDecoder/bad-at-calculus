# ADR 0015 — Evidência de verificação das respostas esperadas

- **Status:** aceito pelo mantenedor em 2026-10-15
- **Data:** 2026-10-15
- **Relacionados:** [ADR 0008](0008-formato-canonico-de-exercicio.md) ·
  [ADR 0009](0009-correcao-no-motor-rust.md) · [ADR 0014](0014-biblioteca.md) ·
  [formato de exercícios](../../02-produto/formato-de-exercicios.md#12-evidência-de-verificação-verify-e-evidence) ·
  [pesquisa OpenAI Math](../../09-pesquisa/openai-math.md)

## Contexto

Hoje o compilador de `*.ex.md` corrige a resposta esperada contra ela mesma: a resposta precisa se
corrigir como "correta". Isso pega erro de sintaxe e de tipo, mas é tautológico. Não mostra que a
resposta esperada resolve o enunciado. Uma derivada errada no gabarito passa nessa checagem.

Três fatores pedem mais que isso:
- **O selo "verificado pelo motor"** da Biblioteca (ADR 0014; PRD, ponto 9) vale só para
  checagens e exercícios. Ele precisa de um registro do que foi conferido, como e com qual versão
  do motor, além de uma regra de invalidação.
- **A pesquisa sobre o OpenAI Math** separa afirmação, evidência e verificação. Lá, cada
  verificador grava versões, hashes, duração e resultado num `run.json`, e o catálogo de
  formalizações declara o **escopo** do que foi provado. Na Vértice, o exemplo `x/x` contra `1`
  mostra a mesma coisa em escala menor: amostragem numérica é evidência **no domínio amostrado**,
  não identidade ([ADR 0009](0009-correcao-no-motor-rust.md), consequências).
- **Regressões do corretor.** Uma mudança no motor que vire o veredito de um gabarito público hoje
  passaria despercebida.

Este assunto é do `content` e do `assessment`, não da Biblioteca. Por isso fica num ADR separado
do 0014, que só consome o resultado.

## Decisão

### 1. A evidência é gerada, não declarada

- Na **autoria**, cada parte pode trazer `verify:` (opcional), que diz **como** conferir a resposta
  esperada de forma independente:
  - `verify: solution` — roda o comando de `solution.engine` e compara o resultado principal com a
    resposta esperada;
  - `verify: { command, request, result }` — roda um comando do `dispatch` e compara o valor
    apontado por `result` (JSON Pointer, RFC 6901) com a resposta esperada;
  - `verify: { human: { by, against, at } }` — conferência humana (ex.: "gabarito oficial da Lista
    04"), com nome e data.
- No **canônico**, o compilador grava `parts[].evidence[]`. O autor nunca escreve evidência de
  motor à mão; o compilador rejeita `evidence` no `*.ex.md`.
- O mesmo vale para as perguntas de checagem dos roteiros (`*.guide.md`), que são `Part`s.

### 2. Forma da evidência

```jsonc
"evidence": [
  {
    "method": "symbolic",            // computed | sampled | symbolic | human
    "scope": { "kind": "full" },     // full | restricted (+ domain, note)
    "verdict": "confirmed",          // confirmed | mismatch | indeterminate
    "engineVersion": "0.4.0",        // versão do núcleo (workspace vx-*), ausente se human
    "contentHash": "sha256:9f2c…",   // hash do canônico da revisão, sem o campo evidence
    "seed": null, "samples": null,   // preenchidos quando method = sampled
    "by": null, "against": null, "at": "2026-10-15"
  }
]
```

| `method` | Significa | Força |
| -------- | --------- | ----- |
| `symbolic` | O motor obteve o resultado de forma independente, e ele coincide **exatamente** com o esperado: mesma forma após `simplified()` ou o mesmo valor exato (racional/constante) | Identidade, dentro do que o simplificador cobre |
| `computed` | O motor calculou um **valor numérico** de forma independente (integral por Simpson, autovalor…), e ele coincide com o esperado dentro da tolerância | Numérica, com a tolerância registrada |
| `sampled` | Resultado do motor e esperado concordam nos *N* pontos amostrados (ADR 0009) | Probabilística, **só no domínio amostrado**; `scope.domain` é obrigatório |
| `human` | Uma pessoa identificada conferiu contra uma fonte | Depende da pessoa e da fonte; nunca vira selo de motor |

- O `scope` é obrigatório:
  - `full` quando a evidência cobre toda a resposta;
  - `restricted` quando cobre parte dela (ex.: alguns elementos de um `set`) ou vale só sob
    hipótese (domínio, `x ≠ 0`), descrita em `note`.
- `choice`, `matching` e `self_graded` só admitem `human`.

### 3. Registro reprodutível na CI

- `vertice content build --evidence` gera **`content/evidence.lock.json`**, versionado, com uma
  entrada por `exerciseId@revision#parte` (e `guideId@revision#etapa/rótulo`). Cada entrada traz
  `contentHash`, `engineVersion`, `method`, `scope`, `seed`, `samples`, `verdict` e `durationMs`.
- A CI de `content/`:
  1. recalcula toda a evidência em memória;
  2. **falha** se algum `verify` declarado der `mismatch` ou `indeterminate`;
  3. **falha** se um veredito mudar em relação ao lock sem mudança de `contentHash` (regressão do
     corretor);
  4. **falha** se o lock estiver desatualizado. É a mesma política dos arquivos gerados do
     ADR 0008.
- O lock grava `engineVersion` (semver do núcleo), não o commit. Assim, PRs de código Rust que não
  mudam a versão não mexem no lock. O commit exato do motor vai para a proveniência do release
  (atestação SLSA, [modelo de ameaças](../../06-seguranca/modelo-de-ameacas.md), T2/T3).
- **Packs privados** recebem a mesma evidência, calculada localmente na importação ou na
  compilação e guardada no próprio pack. Nunca passam pela CI pública
  ([ADR 0012](0012-conteudo-de-terceiros-e-licencas.md)).

### 4. Selo "verificado pelo motor" e invalidação

- Um exercício (ou checagem) recebe `engine_verified` quando **toda parte corrigível** tem pelo
  menos uma evidência `symbolic`, `computed` ou `sampled` com `verdict: confirmed` para o
  `contentHash` atual.
- Evidência só `sampled` ou `restricted` aparece com qualificador ("por amostragem em
  x ∈ [−5, 5]").
- **Invalidação:**
  - nova revisão do exercício muda o `contentHash`, e a evidência antiga deixa de valer;
  - `engineVersion` diferente da do app faz o app reverificar localmente antes de mostrar o selo
    (custa microssegundos por parte);
  - se a reverificação falhar, o selo some e a parte é sinalizada para o autor.
- Evidência vinda de pack importado é sempre recalculada localmente. O selo nunca é confiado do
  arquivo (T1/T7).

### 5. Onde mora no código

- Tipos `Evidence`, `EvidenceMethod`, `EvidenceScope` e a regra do selo: domínio do `content`
  (`vx-domain/src/content/evidence.rs`).
- Caso de uso `VerifyExpectedAnswers` em `vx-application/src/use_cases/content/`. Ele usa a API
  pública do `assessment` para comparar e o port **`EngineCommandRunner`** para executar o comando
  de `verify`. O port é implementado em `vx-api` com o próprio `dispatch`, mantendo a inversão de
  dependência: a aplicação não conhece `vx-api`.

## Alternativas consideradas

- **Pôr a evidência no ADR 0014.** Um ADR a menos, mas a decisão vale para todo exercício, inclusive
  sem Biblioteca, e atualiza o contrato do ADR 0008. Misturar os dois dificultaria substituir um
  sem o outro.
- **Evidência declarada à mão pelo autor.** Simples, mas impossível de conferir, e é exatamente o
  "selo sem lastro" que o PRD quer evitar.
- **Só artefato da CI, sem arquivo versionado.** Menos ruído nos PRs, mas a evidência some com a
  retenção dos artefatos, não roda offline e não mostra regressão no diff.
- **Commit do motor no lock.** Reprodutibilidade máxima, mas todo PR de Rust mudaria o lock.
- **Segundo oráculo CAS** (SymPy via Pyodide, Maxima) para `symbolic`. Daria mais força, porém
  traz ~10 MB e um servidor ou etapa Python na CI. Fica como evolução se a medição mostrar muitos
  `indeterminate` (ADR 0009 já o prevê como *fallback*).
- **Prova formal (Lean).** Fora do escopo do produto, como concluiu a pesquisa OpenAI Math.

## Consequências

- ✅ O selo "verificado pelo motor" passa a ter lastro, escopo e regra de invalidação.
- ✅ Regressões do corretor aparecem na CI como mudança de veredito em gabarito real.
- ✅ Os campos novos são opcionais e aditivos: `schemaVersion` não muda (ADR 0008, §9 do formato).
- ⚠️ Evidência confirma que a resposta bate com o que o motor calculou **para o pedido em
  `verify`**. Se o pedido foi copiado errado do enunciado, a evidência confirma o problema errado.
  A revisão humana do `verify` continua necessária, e a tela de revisão do importador (F2) mostra
  o pedido ao lado do enunciado.
- ⚠️ Toda nova versão do núcleo regenera o lock inteiro, com diff grande, porém mecânico e revisável.
- ⚠️ Cada comando usado em `verify` precisa declarar seu "resultado principal" no contrato, ou o
  autor informa `result`.
- 🔭 Próximos passos:
  - `backend-developer`: `Evidence`, `VerifyExpectedAnswers` e `EngineCommandRunner` junto com o
    compilador da F1;
  - `devops-engineer`: job do lock;
  - `qa-engineer`: casos `x/x`, `sqrt(x^2)` contra `x` e domínio restrito como testes do escopo.
