# ADR 0003 — Mesmo núcleo para WASM (web) e Tauri (desktop)

- **Status:** aceito — a forma de expor o motor foi refinada pelo [ADR 0005](0005-mv-api-dispatch-unico.md)
- **Data:** 2026-09-28

## Contexto
Estudantes querem abrir no navegador durante a aula; usuários frequentes preferem um app
instalado com acesso a arquivos. Manter dois motores seria inviável.

## Decisão
- O motor é exposto por **duas camadas de entrega finas**: `crates/mv-wasm` e
  `apps/desktop/src-tauri/src/commands.rs`, ambas chamando os mesmos casos de uso.
- O front acessa o motor apenas pela interface `MathGateway`; `createMathGateway()` escolhe o
  adapter em runtime (Tauri se `__TAURI_INTERNALS__` existir, senão WASM).

## Consequências
- ✅ Um único código matemático, testado uma vez.
- ✅ A UI é idêntica nas duas plataformas.
- ~~⚠️ Toda operação nova deve ser exposta nos dois adapters~~ — resolvido pelo ADR 0005: ambos chamam `mv_api::dispatch`.
- 🔭 Alternativa futura: usar o WASM também dentro do Tauri e eliminar o IPC, se medições
  mostrarem ganho.
