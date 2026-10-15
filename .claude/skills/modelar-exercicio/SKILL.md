---
name: modelar-exercicio
description: Transforma um enunciado de exercício (texto, imagem ou PDF) e seu gabarito em um arquivo *.ex.md no formato da Vértice, com tipo de resposta, dicas, visualização e respostas conferidas. Use ao modelar listas de exercícios, públicas (content/) ou privadas (content-private/).
---

# Modelar exercício

Referências: `docs/02-produto/formato-de-exercicios.md`, `docs/02-produto/taxonomia.md`,
`docs/03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md`.

## 1. Decidir o destino (licença)
| Origem | Destino | `source` |
| ------ | ------- | -------- |
| Autoral (você criou) | `content/exercises/<disciplina>/<topico>/` | `kind: authored`, `license: CC-BY-SA-4.0` |
| Fonte aberta (OpenStax etc.) | `content/...` | `kind: imported`, licença original, `citation` |
| Lista, prova, apostila ou slide de professor | `content-private/packs/<nome>/` (fora do git) | `kind: imported`, `license: proprietary`, `citation`, `visibility: private` |

Na dúvida, privado.

## 2. Extrair a estrutura
- Separe **enunciado comum** (vai no corpo) e **partes** (`::: part a`, `::: part b`…).
- Um item com duas perguntas ("determine f(−2) e f'(−2)") vira **duas partes**.
- Figuras: salve em `assets/` com texto alternativo descritivo.

## 3. Escolher o tipo de resposta
| O enunciado pede | Tipo |
| ---------------- | ---- |
| Um número (`f'(2)`, altura em 40 s) | `numeric` (valor exato: `6400*sqrt(10)`) |
| Uma função/expressão (`dy/dx`) | `expression` com `variables` (e `domain` se houver `sqrt`/`ln`) |
| Uma equação de reta/plano | `equation` |
| "Todos os valores de x tais que…" | `set` |
| Ponto, vetor | `tuple`; matriz → `matrix` |
| Associe A–D com 1–4 | `matching` |
| Objetiva | `choice` |
| Esboço, demonstração, explicação | `self_graded` com `rubric` |
| Código (POO, algoritmos) | `self_graded` até a F2; depois `code` |

## 4. Conferir cada resposta
- Recalcule a resposta (Laboratório da Vértice, motor via testes, ou conta à mão registrada nas dicas).
- Compare com o gabarito oficial. **Divergência → registre** no relatório (não altere em silêncio).
- Para `expression`/`equation`, anote 2–3 formas equivalentes que devem ser aceitas (viram casos de
  teste do `qa-engineer`).

## 5. Enriquecer
- `hints`: 1–3, do geral para o específico, sem entregar a resposta.
- `feedback`: erros comuns previsíveis (ex.: esqueceu a regra da cadeia) com mensagem curta.
- `viz`: se existir visualização do tópico (ex.: `calculus.derivative` com `f` e `x0`).
- `skills`: 1–4 habilidades da taxonomia; `difficulty` 1–5.

## 6. Esqueleto
```markdown
---
id: <uuidv7>
title: <título curto>
skills: [<area>.<disciplina>.<topico>.<habilidade>]
difficulty: 2
source: { kind: authored, license: CC-BY-SA-4.0, authors: [<nome>] }
---
<enunciado comum em Markdown + LaTeX>

::: part a
<pergunta>
answer: { type: numeric, value: "<valor exato>" }
hints:
  - <dica 1>
:::
```

## 7. Relatório final
Tabela: item · parte · tipo · resposta esperada · conferida (✓ / divergência: …) · observações.
