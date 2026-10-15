# AGENTS.md — guia para agentes de IA

Contexto mínimo para um agente contribuir sem quebrar a arquitetura. Humanos também podem ler.

## Mapa do repositório

| Caminho                        | Camada               | Pode depender de                               |
| ------------------------------ | -------------------- | ---------------------------------------------- |
| `crates/mv-domain`             | Domain (entities)    | `std` + bibliotecas numéricas puras (`nalgebra`, ADR 0004) |
| `crates/mv-application`        | Application          | `mv-domain`                                    |
| `crates/mv-infra`              | Interface adapters   | `mv-application`, `mv-domain`                  |
| `crates/mv-api`                | Interface adapters   | `mv-infra`, `mv-application`, `mv-domain`, `serde` |
| `crates/mv-wasm`               | Frameworks & drivers | `mv-api` (uma função: `dispatch`)              |
| `apps/desktop/src-tauri`       | Frameworks & drivers | `mv-api` (um comando: `engine`)                |
| `apps/web/src/lib/domain`      | Front: domain        | nada                                           |
| `apps/web/src/lib/ports`       | Front: ports         | `domain`                                       |
| `apps/web/src/lib/application` | Front: application   | `domain`, `ports`                              |
| `apps/web/src/lib/adapters`    | Front: adapters      | `ports`, `domain`, Tauri/WASM                  |
| `apps/web/src/lib/composition` | Composition root     | tudo (cria os adapters)                        |
| `apps/web/src/lib/ui`          | Front: UI            | `application`, `domain`                        |

**Regra de ouro:** dependências apontam para dentro. Nunca importe `mv-infra` a partir de
`mv-application`, nem `@tauri-apps/*` ou `$lib/adapters` a partir de `lib/ui` ou
`lib/application` (o ESLint bloqueia).

## Como adicionar uma operação ao motor (receita)

1. **Domínio** (`mv-domain/src/<assunto>/`): a regra matemática + testes.
2. **Aplicação** (`mv-application/src/use_cases/<assunto>/`): caso de uso `VerboSubstantivo`
   com `Request`/`Report` + teste com fakes (`use_cases/test_support.rs`).
3. **API** (`mv-api/src/<assunto>.rs`): DTOs + handler; registre o comando em `commands` e em
   `dispatch`; teste o JSON de resposta.
4. **Front**: tipo em `lib/domain/<assunto>.ts`, método no port (`lib/ports/*-engine.ts`) e no
   adapter (`lib/adapters/engine/*-adapter.ts`), nome em `adapters/engine/commands.ts`.
5. **Estado** em `lib/application/*.svelte.ts` (use `LatestRequest` para ignorar respostas velhas).
6. **UI** em `lib/ui/<assunto>/` e a página em `src/routes/…`; textos em PT-BR em `lib/ui/copy/`.
7. `pnpm wasm:build` para o navegador enxergar a mudança.

## Regras rápidas

- Documentação e textos da UI em PT-BR; código, identificadores e commits em inglês.
- Funções pequenas (≤ 4 argumentos contando `self` — o clippy bloqueia), nomes descritivos
  ([clean-code](docs/05-engenharia/clean-code.md)).
- Sem `unwrap`/`expect`/`panic!` fora de testes; use `Result`.
- LaTeX em atributos Svelte: use `formula={String.raw\`...\`}` — chaves `{}` em atributos
  literais são interpretadas como JavaScript.
- Resultados numéricos na UI: `equalsLatex(v)` (mostra `= 1/2` exato ou `≈ 1,23`).
- Mudou arquitetura? Adicione um ADR em `docs/03-arquitetura/adr/`.

## Comandos

```bash
cargo fmt --all
cargo clippy --workspace --exclude math-view-desktop --all-targets -- -D warnings
cargo test --workspace --exclude math-view-desktop
pnpm wasm:build
pnpm -C apps/web check
pnpm -C apps/web lint
pnpm -C apps/web test
```

## Onde ler mais

- Arquitetura: [docs/03-arquitetura/visao-geral.md](docs/03-arquitetura/visao-geral.md)
- Motor matemático: [docs/03-arquitetura/motor-matematico.md](docs/03-arquitetura/motor-matematico.md)
- Convenções: [docs/05-engenharia/convencoes.md](docs/05-engenharia/convencoes.md)
- Design system: [docs/04-design/design-system.md](docs/04-design/design-system.md)
