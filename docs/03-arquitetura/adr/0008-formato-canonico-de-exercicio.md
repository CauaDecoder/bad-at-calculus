# ADR 0008 — Formato canônico de exercício

- **Status:** aceito
- **Data:** 2026-10-14

## Contexto
Exercícios vêm de vários lugares: o banco público por PR, o editor do app, importação em lote com
gabarito separado e, na F2, PDFs lidos por IA. Precisamos de um formato que seja:
- validável por máquina, com erros por campo;
- confortável para pessoas que não programam (colaboradora de AU);
- versionável (o formato vai evoluir);
- igual em Rust (correção, servidor) e TypeScript (UI), sem dessincronizar, que é o risco
  "stringly typed" registrado no [ADR 0005](0005-mv-api-dispatch-unico.md).

Referências estudadas ([pesquisa](../../09-pesquisa/plataformas-de-referencia.md)): Perseus (JSON com
`itemDataVersion` e versão por widget), OATutor (JSON + CC BY), Numbas (JSON de exame), QTI 3.0
(XML de interoperabilidade), WeBWorK PG (Perl + LaTeX).

## Decisão
- **Canônico em JSON**, com `schemaVersion` global e `revision` por exercício. A spec está em
  [formato-de-exercicios.md](../../02-produto/formato-de-exercicios.md).
- **Os tipos Rust são a fonte da verdade.** Com `serde` + `schemars` geramos o **JSON Schema**, e
  com `ts-rs` (ou `specta`) os **tipos TypeScript**. Arquivos gerados ficam versionados e a CI
  falha se estiverem desatualizados.
- **Autoria em `*.ex.md`** (Markdown + YAML frontmatter + blocos `::: part`), compilado para o
  canônico por um comando do núcleo, que roda igual no app e na CI.
- **Pack `.vertice.zip`** para distribuição/importação, com gabarito opcional separado.
- Enunciados em **Markdown + LaTeX (KaTeX)**, sem HTML cru (sanitização em
  [modelo de ameaças](../../06-seguranca/modelo-de-ameacas.md)).
- Visualizações embutidas por `viz: {id, params}`, validadas contra o schema da visualização no
  registro do front.
- QTI 3.0 só como **exportação** futura.

## Alternativas consideradas
- **QTI 3.0 como formato interno** — padrão de mercado, mas XML verboso, sem tipos matemáticos
  ricos e caro de editar à mão.
- **Perseus JSON** — maduro, mas atrelado a widgets React do Khan e com suporte externo descontinuado
  (hoje mantido pelo fork OpenPerseus).
- **Só YAML** — fácil de ler, mas ambíguo (tipos implícitos, a armadilha `no` → `false`) e pior
  para máquinas. Por isso fica restrito ao frontmatter.
- **Schema escrito à mão em TS (Zod/Valibot)** — ótimo no front, mas o núcleo Rust precisaria
  duplicar tudo.

## Consequências
- ✅ Um contrato, três consumidores (núcleo, UI, IA). O JSON Schema também serve de *structured
  output* para o LLM do importador de PDF.
- ✅ Contribuir com conteúdo = editar Markdown; a CI valida.
- ⚠️ Adiciona `schemars` e `ts-rs` às camadas externas (`vx-api`), nunca ao domínio.
- ⚠️ Mudança de `schemaVersion` exige migração escrita e testada.
