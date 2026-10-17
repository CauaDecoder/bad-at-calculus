---
name: nova-operacao-motor
description: Receita para expor uma operação nova do motor Rust da Vértice (domínio → caso de uso → comando no dispatch do vx-api → port/adapter TS → estado → UI). Use quando for adicionar um cálculo, verificação ou comando novo ao motor, inclusive correção de respostas.
---

# Nova operação no motor

Siga os passos em ordem; cada passo tem seu teste. O workspace usa `vx-*` desde o renome da F0.2.

1. **Domínio** — `crates/vx-domain/src/<assunto>/`
   - Implemente a regra pura (sem I/O, sem serde). Erros em `DomainError`.
   - Testes no próprio arquivo (`#[cfg(test)]`), com casos reais e tolerância para floats.
2. **Aplicação** — `crates/vx-application/src/use_cases/<assunto>/`
   - Caso de uso `VerbNoun` com `Request`/`Report` e método `execute`.
   - Se precisar de algo externo (parser, storage), use um **port** (trait em `ports/`).
   - Teste com fakes de `use_cases/test_support.rs`: caminho feliz + cada erro.
3. **API** — `crates/vx-api/src/<assunto>.rs`
   - DTOs `serde` em camelCase, handler, nome do comando em `vx_api::commands` e o caso em `dispatch`.
   - Erros viram `ApiError { code, message, position?, field? }`.
   - Teste o JSON de resposta.
4. **Front: contrato**
   - Tipo em `apps/web/src/lib/domain/<assunto>.ts`.
   - Método no port `lib/ports/*-engine.ts` e no adapter `lib/adapters/engine/*-adapter.ts`.
   - Nome do comando em `lib/adapters/engine/commands.ts` (**igual** ao do Rust).
   - Mensagem PT-BR para cada `code` novo em `lib/ui/copy/engine-messages.ts`.
5. **Estado** — `lib/application/<algo>.svelte.ts` com runes; `LatestRequest` para descartar
   respostas velhas; teste com o fake do engine em `application/test-support.ts`.
6. **UI** — componentes em `lib/ui/<assunto>/`, página em `src/routes/…`, textos em `lib/ui/copy/`.
7. **Build e verificação**
   ```bash
   cargo fmt --all
    cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings
    cargo test --workspace --exclude vertice-desktop
   pnpm wasm:build
   pnpm -C apps/web check && pnpm -C apps/web lint && pnpm -C apps/web test
   ```

Se a operação muda uma decisão de arquitetura, crie um ADR (skill `novo-adr`).
