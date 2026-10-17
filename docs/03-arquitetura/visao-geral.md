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
    V["Vértice<br/>laboratório · exercícios · biblioteca · revisão"]
    GH[("GitHub<br/>código e banco público")]
    LLM["Provedor de LLM<br/>Ollama local ou API — opcional, F2"]
    CF["Cloudflare<br/>hospedagem estática, CDN, WAF"]
    META["Serviços de metadados<br/>Crossref, arXiv — opcional, F2"]
    EXT["Sites externos<br/>papers, livros abertos, vídeos (só link)"]

    EST -- "estuda, lê, resolve, revisa" --> V
    AUT -- "cria e importa listas, notas e roteiros" --> V
    DEV -- "PRs de código e conteúdo" --> GH
    GH -- "build e deploy" --> CF
    CF -- "entrega o app" --> V
    V -. "importar PDF, dicas (com consentimento)" .-> LLM
    V -. "DOI/arXiv, sob ação do usuário" .-> META
    EST -. "abre o link fora do app" .-> EXT
```

## Containers (C4 — nível 2) por fase

### Fases 0–2: sem servidor próprio

```mermaid
flowchart TB
    subgraph Browser["Navegador / Tauri"]
        UI["App SvelteKit<br/>UI + estado (runes)"]
        CORE["Núcleo Rust<br/>WASM ou nativo<br/>motor · correção · conteúdo"]
        LDB[("Armazenamento local<br/>IndexedDB (web) · SQLite (desktop)<br/>+ arquivos da Biblioteca (F2)")]
        IDX["Índice do banco público<br/>JSON estático + busca local"]
        RDR["Leitor PDF.js (F2)<br/>Web Worker isolado"]
        LIDX["Índice da Biblioteca (F2)<br/>MiniSearch: público e privado separados"]
        UI -- "dispatch(command, json)" --> CORE
        UI --> LDB
        UI --> IDX
        UI --> RDR
        UI --> LIDX
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
        R2[("R2 privado<br/>assets, arquivos autorais do usuário, backups")]
    end
    subgraph Oracle["Oracle Cloud ARM (Coolify)"]
        API["vx-server<br/>Axum + mesmos casos de uso"]
        PG[("Postgres + pgvector")]
        IDP["IdP OIDC<br/>passkeys"]
        OTEL["OTel Collector"]
        AV["ClamAV<br/>varredura de upload"]
        API --> PG
        API --> IDP
        API --> OTEL
        API --> AV
    end
    Client --> PAGES
    Client -- "HTTPS + token" --> WAF --> API
    API --> R2
    Client -- "upload por URL pré-assinada (só arquivo autoral)" --> R2
    LDB <-. "sync (ADR futuro)" .-> API
```

Arquivos de terceiros guardados pelo usuário **nunca** sobem para o R2. Sincronizam só os
metadados, destaques e anotações deles ([ADR 0014](adr/0014-biblioteca.md)).

Modelo de dados: [modelo-de-dados.md](modelo-de-dados.md). Segurança:
[modelo de ameaças](../06-seguranca/modelo-de-ameacas.md). Infra: [DevOps](../07-devops/ambientes-e-deploy.md).

## Contextos do núcleo

| Contexto | Responsabilidade | Status |
| -------- | ---------------- | ------ |
| `expression`, `calculus`, `vectors`, `geometry`, `linear_algebra` | Motor matemático do Laboratório | ✅ |
| `shared` | `SkillId`, `Visibility`, `License` na F1.1; `ContentHash` na F1.4; `Locator` estruturado e `EntityRef` só com consumidor ([ADR 0016](adr/0016-contrato-de-conteudo-f1.md)) | F1.1 entregue; expansão incremental |
| `assessment` | `AnswerSpec`, `ScalarAnswerSpec`, correção, feedback de erro comum; API pública consumida pelo `content` | Tipos F1.1 entregues; correção F1.2 pendente |
| `content` | Exercício, lista, pack, taxonomia, validação, compilação de `*.ex.md`, `references[]`, evidência de verificação ([ADR 0015](adr/0015-evidencia-de-verificacao.md)) | Tipos e validações F1.1 entregues; compilação e evidência pendentes |
| `review` | Estado FSRS dos cartões de revisão (alvo: parte de exercício ou nota/destaque) | 🔜 F2 |
| `progress` | Tentativas e domínio por habilidade | 🔜 F2 |
| `library` | Material, coleção, destaque, roteiro de leitura; política de licença e hospedagem; status e errata ([ADR 0014](adr/0014-biblioteca.md)) | 🔜 F2 |

As crates usam `vx-*` desde o renome técnico local da F0.2
([ADR 0006](adr/0006-plataforma-vertice.md)). Os ADRs históricos preservam os nomes anteriores.

`Part` pertence ao `content`; seu `answer` usa o `AnswerSpec` público do `assessment`, sem
dependência inversa. DTOs, `serde`, `schemars` e `ts-rs` deste contrato ficam em `vx-api`.
O [ADR 0016](adr/0016-contrato-de-conteudo-f1.md) fixa coleções, manifesto de pack e o recorte
F1.1/F1.4 sem antecipar a Biblioteca.

## Camadas

```mermaid
flowchart TB
    subgraph FD["Frameworks & Drivers"]
        WEB["apps/web<br/>Svelte + TS"]
        DESK["apps/desktop<br/>Tauri 2 · comando engine"]
        WASM["crates/vx-wasm<br/>função dispatch"]
    end
    subgraph IA["Interface Adapters"]
        API["crates/vx-api<br/>comandos · DTOs · ApiError"]
        INFRA["crates/vx-infra<br/>parser · storage"]
    end
    subgraph UC["Application — Use Cases"]
        APP["crates/vx-application<br/>calculus · vectors · linear_algebra · ports"]
    end
    subgraph EN["Domain — Entities"]
        DOM["crates/vx-domain<br/>expression · calculus · vectors · geometry · linear_algebra"]
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
| Domain (entities)     | `crates/vx-domain`                    | Conceitos e regras matemáticas puras                |
| Application           | `crates/vx-application`               | Casos de uso e *ports* (interfaces para o exterior) |
| Interface adapters    | `crates/vx-infra`, `crates/vx-api`    | Parser (implementa port) e contrato JSON do motor   |
| Frameworks & drivers  | `vx-wasm`, `apps/desktop`, `apps/web` | Entrega: navegador, desktop, interface              |

Detalhes e regras de importação: [camadas.md](camadas.md).

## Regra de dependência

- `vx-domain` depende só de `std` e de bibliotecas numéricas puras (`nalgebra`, [ADR 0004](adr/0004-bibliotecas-matematicas-no-dominio.md)).
- `vx-application` depende só de `vx-domain`. Define *traits* (ports) para tudo que precisa de fora.
- `vx-infra` implementa esses traits (inversão de dependência).
- `vx-api` monta os casos de uso com os adapters e define o contrato JSON ([ADR 0005](adr/0005-mv-api-dispatch-unico.md)).
- `vx-wasm` e `src-tauri` têm **uma função cada**, que só repassa para `vx_api::dispatch`.

A regra é verificável: basta olhar os `[dependencies]` de cada `Cargo.toml`.

## Fluxo de uma requisição: "limite de sin(x)/x em x → 0"

```mermaid
sequenceDiagram
    participant UI as Página /calculo/limites
    participant App as LimitExplorer (front application)
    participant Port as CalculusEngine (port)
    participant Ad as CalculusEngineAdapter + WasmTransport
    participant Api as vx_api::dispatch
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
- [ADR 0014 — Biblioteca: contexto `library`, material de estudo e roteiros](adr/0014-biblioteca.md) (aceito)
- [ADR 0015 — Evidência de verificação das respostas esperadas](adr/0015-evidencia-de-verificacao.md) (aceito)
- [ADR 0016 — Fronteiras e contrato mínimo de conteúdo na F1](adr/0016-contrato-de-conteudo-f1.md) (aceito; implementação pendente)
