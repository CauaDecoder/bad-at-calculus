# ADR 0001 — Rust + Tauri + Svelte

- **Status:** aceito
- **Data:** 2026-09-28

## Contexto
O projeto precisa de: cálculo numérico/simbólico rápido (arraste a 60 fps), interface moderna e
fácil de estilizar, distribuição desktop leve e, idealmente, uma versão web sem instalação.
Deve ser open source e convidativo a contribuidores.

## Decisão
- **Núcleo matemático em Rust**, organizado como workspace de crates.
- **Interface em Svelte 5 + TypeScript** (SvelteKit estático).
- **Desktop com Tauri 2**; **web com o núcleo compilado para WebAssembly**.

## Alternativas consideradas
- **Rust puro com egui/iced** — máxima performance, mas visual limitado e difícil de refinar.
- **Electron + TypeScript puro** — ecossistema enorme, mas instalador pesado e cálculo lento em JS.
- **React em vez de Svelte** — mais popular, porém mais verboso; Svelte gera menos código e é
  mais legível para quem está começando.
- **Python (PyQt/Jupyter)** — familiar para físicos, mas distribuição e performance de UI piores.

## Consequências
- ✅ Mesmo núcleo para desktop e web; alta performance; memória segura.
- ✅ UI com CSS completo → design system moderno.
- ⚠️ Duas linguagens no projeto; a fronteira (DTOs) precisa ser mantida em sincronia.
- ⚠️ Contribuidores de matemática precisam de Rust básico — mitigado por docs e exemplos.
