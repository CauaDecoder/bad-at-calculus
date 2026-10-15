# Arquitetura — visão geral

O Math View segue a **Clean Architecture** (Robert C. Martin): o sistema é dividido em camadas
concêntricas e **as dependências de código só apontam para dentro**. As regras matemáticas (o que
mais importa) ficam no centro, isoladas de frameworks, UI e detalhes de entrega.

## Camadas

```mermaid
flowchart TB
    subgraph FD["Frameworks & Drivers"]
        WEB["apps/web<br/>Svelte + TS"]
        DESK["apps/desktop<br/>Tauri 2 · comando engine"]
        WASM["crates/mv-wasm<br/>função dispatch"]
    end
    subgraph IA["Interface Adapters"]
        API["crates/mv-api<br/>comandos · DTOs · ApiError"]
        INFRA["crates/mv-infra<br/>parser · storage"]
    end
    subgraph UC["Application — Use Cases"]
        APP["crates/mv-application<br/>calculus · vectors · linear_algebra · ports"]
    end
    subgraph EN["Domain — Entities"]
        DOM["crates/mv-domain<br/>expression · calculus · vectors · geometry · linear_algebra"]
    end

    WEB --> WASM
    WEB -. IPC .-> DESK
    DESK --> API
    WASM --> API
    API --> APP
    API --> INFRA
    INFRA --> APP
    INFRA --> DOM
    APP --> DOM
```

| Camada                | Onde                                  | Responsabilidade                                    |
| --------------------- | ------------------------------------- | --------------------------------------------------- |
| Domain (entities)     | `crates/mv-domain`                    | Conceitos e regras matemáticas puras                |
| Application           | `crates/mv-application`               | Casos de uso e *ports* (interfaces para o exterior) |
| Interface adapters    | `crates/mv-infra`, `crates/mv-api`    | Parser (implementa port) e contrato JSON do motor   |
| Frameworks & drivers  | `mv-wasm`, `apps/desktop`, `apps/web` | Entrega: navegador, desktop, interface              |

Detalhes e regras de importação: [camadas.md](camadas.md).

## Regra de dependência

- `mv-domain` depende só de `std` e de bibliotecas numéricas puras (`nalgebra`, [ADR 0004](adr/0004-bibliotecas-matematicas-no-dominio.md)).
- `mv-application` depende só de `mv-domain`. Define *traits* (ports) para tudo que precisa de fora.
- `mv-infra` implementa esses traits (inversão de dependência).
- `mv-api` monta os casos de uso com os adapters e define o contrato JSON ([ADR 0005](adr/0005-mv-api-dispatch-unico.md)).
- `mv-wasm` e `src-tauri` têm **uma função cada**, que só repassa para `mv_api::dispatch`.

A regra é verificável: basta olhar os `[dependencies]` de cada `Cargo.toml`.

## Fluxo de uma requisição: "limite de sin(x)/x em x → 0"

```mermaid
sequenceDiagram
    participant UI as Página /calculo/limites
    participant App as LimitExplorer (front application)
    participant Port as CalculusEngine (port)
    participant Ad as CalculusEngineAdapter + WasmTransport
    participant Api as mv_api::dispatch
    participant UC as AnalyzeLimit (use case)
    participant P as PrattParser (infra)
    participant D as RealFunction::analyze_limit (domain)

    UI->>App: estudante digita "sin(x)/x" e a = 0
    App->>Port: analyzeLimit("sin(x)/x", {kind: finite, point: "0"})
    Port->>Ad: implementação escolhida no composition root
    Ad->>Api: dispatch("calculus.limit", JSON)
    Api->>UC: execute(LimitRequest)
    UC->>P: parse("sin(x)/x") e parse("0")
    UC->>D: analyze_limit(Finite(0))
    D-->>UC: LimitAnalysis (laterais, limite, continuidade)
    UC-->>Api: LimitAnalysis
    Api-->>Ad: JSON { limit: {kind: finite, value: 1}, … }
    Ad-->>App: LimitAnalysis (tipo TS)
    App-->>UI: estado reativo → gráfico, tabela e veredito
```

## Por que duas entregas (WASM e Tauri)?

O mesmo núcleo Rust roda:
- **no navegador**, compilado para WebAssembly (zero instalação, ideal para aula);
- **no desktop**, via Tauri (acesso a arquivos, instalador leve, performance nativa).

A UI não sabe qual está usando: ela só conhece os ports (`CalculusEngine`, `VectorEngine`,
`LinearAlgebraEngine`). O `composition/engines.ts` escolhe o transporte em runtime.
Ver [ADR 0003](adr/0003-wasm-e-desktop.md) e [ADR 0005](adr/0005-mv-api-dispatch-unico.md).

## Decisões registradas
- [ADR 0001 — Rust + Tauri + Svelte](adr/0001-rust-tauri-svelte.md)
- [ADR 0002 — Clean Architecture](adr/0002-clean-architecture.md)
- [ADR 0003 — WASM e desktop com o mesmo núcleo](adr/0003-wasm-e-desktop.md)
- [ADR 0004 — Bibliotecas matemáticas puras no domínio](adr/0004-bibliotecas-matematicas-no-dominio.md)
- [ADR 0005 — `mv-api`: contrato JSON único](adr/0005-mv-api-dispatch-unico.md)
