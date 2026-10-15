# ADR 0009 — Correção de respostas no motor Rust

- **Status:** aceito
- **Data:** 2026-10-14

## Contexto
A correção precisa:
- aceitar formas equivalentes (`y = 5x − 16` ≡ `5x − y − 16 = 0`; `2x(5−x²)^{1/2} − x³(5−x²)^{−1/2}` ≡ forma simplificada);
- funcionar **offline** (local-first) e dar o **mesmo veredito** no servidor quando houver contas
  (base para listas avaliadas e antifraude);
- reaproveitar o que o motor já tem: parser Pratt, avaliação de expressões, resultados exatos.

Referências: Numbas corrige expressões no cliente avaliando em pontos aleatórios; STACK usa CAS
(Maxima) no servidor com árvores de resposta e feedback por erro comum; o Compute Engine da CortexJS
compara MathJSON, mas a equivalência simbólica geral exige lógica extra.

## Decisão
Implementar o contexto **`assessment` no núcleo Rust** (domínio + caso de uso `CheckAnswer` +
comando `assessment.check` no `dispatch`), rodando em WASM, Tauri e, na F3, no `vx-server`.

Algoritmo por tipo (spec em [formato-de-exercicios.md](../../02-produto/formato-de-exercicios.md#3-tipos-de-resposta-answerspec)):
1. **Validar a entrada** com o parser. Erro de sintaxe → `invalid_input` com posição (não conta
   como tentativa).
2. **`numeric`**: avaliar as duas constantes e comparar com tolerância absoluta/relativa.
3. **`expression`**: avaliar esperada e resposta em *N* pontos pseudoaleatórios do domínio, com
   **semente fixa por exercício**, descartando pontos em que a esperada não é finita. Exigir
   concordância em todos os pontos válidos e um mínimo de pontos válidos (senão o resultado é
   "indeterminado" e cai para revisão manual).
4. **`equation`**: `g = lhs − rhs` do estudante e `h` do esperado; correto se `g/h` for constante
   ≠ 0 nos pontos amostrados, onde `h ≠ 0`.
5. **`set`/`tuple`/`matrix`**: casamento elemento a elemento usando 2–3.
6. **`form`** (opcional): checagem estrutural na AST (ex.: "fatorada", "sem expoente negativo") para
   exercícios do tipo "simplifique".
7. **Feedback por erro comum**: as respostas em `feedback[].matches` são testadas com o mesmo
   algoritmo **depois** da resposta correta.

## Alternativas consideradas
- **CAS completo (SymPy via Pyodide, Maxima)** — equivalência simbólica mais forte, mas ~10 MB no
  navegador e/ou servidor obrigatório; latência alta. Fica como fallback opcional futuro para casos
  "indeterminados".
- **Compute Engine (JS)** — bom para entrada (MathLive), mas duplicaria o parser e não roda no
  servidor Rust.
- **Comparar strings normalizadas** — frágil; rejeita formas corretas.

## Consequências
- ✅ Mesmo veredito em todo lugar; correção em microssegundos; testes no domínio com os gabaritos
  das listas (casos reais).
- ⚠️ Amostragem numérica pode, em tese, aceitar funções diferentes que coincidem nos pontos
  sorteados. Mitigação: N ≥ 12, domínio adequado e pontos fora de inteiros; documentar o limite.
- ⚠️ Funções com domínio restrito (`sqrt`, `ln`) precisam de `domain` no exercício ou inferência
  de domínio (o motor já detecta onde `f` não é finita).
- 🔭 Testes de propriedade (`proptest`): toda resposta esperada se corrige como correta, e
  perturbações pequenas (`+ 1`, troca de sinal) não.
