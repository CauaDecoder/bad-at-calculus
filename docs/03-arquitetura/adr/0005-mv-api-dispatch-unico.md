# ADR 0005 — `mv-api`: contrato JSON único para WASM e Tauri

- **Status:** aceito (substitui a consequência "manter os dois adapters em sincronia" do ADR 0003)
- **Data:** 2026-09-28

## Contexto
Na v0.1 cada operação do motor era exposta duas vezes: uma função `#[wasm_bindgen]` em
`mv-wasm` e um `#[tauri::command]` em `src-tauri`, cada uma com seus DTOs. Com a v0.2/v0.3 seriam
11 operações duplicadas, e os erros chegavam à UI como texto em inglês que o front precisava
interpretar com expressões regulares.

## Decisão
Criar a crate **`mv-api`** (camada de adaptadores de interface), dona do contrato do motor:
- **comandos** nomeados (`calculus.limit`, `matrix.solve`…) em `mv_api::commands`;
- **DTOs** serializáveis (JSON em camelCase), espelhados nos tipos de `apps/web/src/lib/domain`;
- **erros estruturados** `ApiError { code, message, position?, field? }`;
- uma única função `dispatch(command, request) -> Result<Value, ApiError>`.

`mv-wasm` e `src-tauri` ficam com **uma função cada**, que só repassa para `dispatch`.
No front, um `EngineTransport` (WASM ou Tauri) é usado por adapters por assunto
(`CalculusEngineAdapter`, `VectorEngineAdapter`, `LinearAlgebraEngineAdapter`) que implementam
os ports.

## Alternativas consideradas
- **Continuar com funções tipadas por operação** — mais tipagem no binding, mas duplicação
  crescente e fácil de dessincronizar.
- **Gerar bindings automaticamente (ex.: `tauri-specta`, `tsify`)** — interessante no futuro;
  hoje adicionaria ferramentas e etapas de build.

## Consequências
- ✅ Operação nova = um handler em `mv-api` + um método no adapter TS. Funciona na web e no desktop.
- ✅ A UI traduz `code` para português (`$lib/ui/copy/engine-messages.ts`) e destaca o campo
  com erro via `field` (ex.: `u.y`, `a[1][2]`).
- ⚠️ O contrato é "stringly typed" na fronteira: nomes de comandos existem em Rust
  (`mv_api::commands`) e em TS (`adapters/engine/commands.ts`). Mantenha os dois iguais;
  os testes de `mv-api` cobrem os formatos de resposta.
