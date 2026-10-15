# Arquitetura — visão geral

A Vértice é um **monólito modular local-first**: um núcleo Rust (motor matemático + correção de
exercícios + regras de conteúdo) que roda **no navegador (WASM), no desktop (Tauri) e, na Fase 3,
no servidor (Axum)**, com uma interface SvelteKit. O código segue a **Clean Architecture**
(Robert C. Martin): camadas concêntricas em que **as dependências só apontam para dentro**.

Princípios:
1. **Mesmo núcleo em todo lugar.** Um exercício corrigido offline e no servidor dá o mesmo veredito.
2. **Local-first.** Sem conta, tudo funciona; a nuvem é um adapter a mais ([ADR 0010](adr/0010-local-first-na-fase-1.md)).
3. **Conteúdo como código.** O banco público é um diretório versionado; o privado nunca entra no git.
4. **Contextos como módulos** dentro das camadas, não microsserviços ([ADR 0007](adr/0007-contextos-como-modulos.md)).
5. **Contratos gerados.** Tipos Rust geram JSON Schema e tipos TS ([ADR 0008](adr/0008-formato-canonico-de-exercicio.md)).

## Sistema (C4 — nível 1: contexto)

```mermaid
flowchart LR
    EST(["Estudante"])
    AUT(["Autor: monitor, professor, colaborador"])
    DEV(["Contribuidor open source"])
    V["Vértice<br/>laboratório · exercícios · revisão"]
    GH[("GitHub<br/>código e banco público")]
    LLM["Provedor de LLM<br/>Ollama local ou API — opcional, F2"]
    CF["Cloudflare<br/>hospedagem estática, CDN, WAF"]

    EST -- "estuda, resolve, revisa" --> V
    AUT -- "cria e importa listas" --> V
    DEV -- "PRs de código e conteúdo" --> GH
    GH -- "build e deploy" --> CF
    CF -- "entrega o app" --> V
    V -. "importar PDF, dicas (com consentimento)" .-> LLM
```

## Containers (C4 — nível 2) por fase

### Fases 0–2: sem servidor próprio

```mermaid
flowchart TB
    subgraph Browser["Navegador / Tauri"]
        UI["App SvelteKit<br/>UI + estado (runes)"]
        CORE["Núcleo Rust<br/>WASM ou nativo<br/>motor · correção · conteúdo"]
        LDB[("Armazenamento local<br/>IndexedDB (web) · SQLite (desktop)")]
        IDX["Índice do banco público<br/>JSON estático + busca local"]
        UI -- "dispatch(command, json)" --> CORE
        UI --> LDB
        UI --> IDX
    end
    CDN["Cloudflare<br/>site estático + previews por PR"] --> Browser
    MCP["vertice-mcp (F2)<br/>servidor MCP local"] -- "mesmo dispatch" --> CORE2["Núcleo Rust nativo"]
```

### Fase 3: contas e nuvem ([ADR 0011](adr/0011-backend-axum-postgres.md))

```mermaid
flowchart TB
    subgraph Client["Navegador / Tauri"]
        UI["App SvelteKit"] --> CORE["Núcleo Rust (WASM)"]
        UI --> LDB[("Local DB")]
    end
    subgraph Edge["Cloudflare"]
        PAGES["Site estático"]
        WAF["WAF · rate limit · Tunnel"]
        R2[("R2<br/>assets e backups")]
    end
    subgraph Oracle["Oracle Cloud ARM (Coolify)"]
        API["vx-server<br/>Axum + mesmos casos de uso"]
        PG[("Postgres + pgvector")]
        IDP["IdP OIDC<br/>passkeys"]
        OTEL["OTel Collector"]
        API --> PG
        API --> IDP
        API --> OTEL
    end
    Client --> PAGES
    Client -- "HTTPS + token" --> WAF --> API
    API --> R2
    LDB <-. "sync (ADR futuro)" .-> API
```

Modelo de dados: [modelo-de-dados.md](modelo-de-dados.md). Segurança:
[modelo de ameaças](../06-seguranca/modelo-de-ameacas.md). Infra: [DevOps](../07-devops/ambientes-e-deploy.md).

## Contextos do núcleo

| Contexto | Responsabilidade | Status |
| -------- | ---------------- | ------ |
| `expression`, `calculus`, `vectors`, `geometry`, `linear_algebra` | Motor matemático do Laboratório | ✅ |
| `assessment` | `AnswerSpec`, correção, feedback de erro comum | 🔜 F1 |
| `content` | Exercício, lista, pack, taxonomia, validação, compilação de `*.ex.md` | 🔜 F1 |
| `review` | Estado FSRS dos cartões de revisão | 🔜 F2 |
| `progress` | Tentativas e domínio por habilidade | 🔜 F2 |

Os nomes de crates abaixo ainda são `mv-*`; o renome para `vx-*` é da Fase 0
([ADR 0006](adr/0006-plataforma-vertice.md)).

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
- [ADR 0006 — Plataforma Vértice: nome e módulos](adr/0006-plataforma-vertice.md)
- [ADR 0007 — Contextos como módulos dentro das camadas](adr/0007-contextos-como-modulos.md)
- [ADR 0008 — Formato canônico de exercício](adr/0008-formato-canonico-de-exercicio.md)
- [ADR 0009 — Correção de respostas no motor Rust](adr/0009-correcao-no-motor-rust.md)
- [ADR 0010 — Local-first na Fase 1](adr/0010-local-first-na-fase-1.md)
- [ADR 0011 — Backend futuro: Axum + Postgres](adr/0011-backend-axum-postgres.md) (proposto)
- [ADR 0012 — Conteúdo de terceiros e licenças](adr/0012-conteudo-de-terceiros-e-licencas.md)
- [ADR 0013 — Direção de design: claro neutro, telas A/D](adr/0013-direcao-de-design.md)
