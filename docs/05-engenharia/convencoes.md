# Convenções

## Idioma
| O quê                                  | Idioma |
| -------------------------------------- | ------ |
| Documentação (`docs/`, READMEs)         | PT-BR  |
| Identificadores, comentários, commits   | Inglês |
| Textos visíveis na interface            | Arquivos de tradução (PT-BR padrão) |

## Rust
- Nomes: `snake_case` (funções, módulos, arquivos), `PascalCase` (tipos, traits),
  `SCREAMING_SNAKE_CASE` (constantes).
- Um módulo por conceito; arquivo com o nome do conceito (`tangent_line.rs`).
- Casos de uso: `VerbNoun` + `Request` + método `execute`
  (`ComputeTangentLine`, `TangentLineRequest`).
- Ports: substantivo do papel (`ExpressionParser`, `WorkspaceRepository`).
  Adapters: tecnologia + papel (`PrattParser`, `JsonFileWorkspaceRepository`).
- Erros: um enum por camada (`DomainError`, `ApplicationError`), com `thiserror` fora do domínio.
- DTOs (`*Dto`) existem só nas camadas de entrega (`vx-wasm`, `src-tauri`).
- `pub` só no que precisa ser público; reexporte a API da crate em `lib.rs`.

## TypeScript / Svelte
- Arquivos: `kebab-case.ts`; componentes `PascalCase.svelte`; estado com runes em `*.svelte.ts`.
- Tipos: `interface` para formatos de objeto, `type` para uniões.
- Sem `any`; `strict` + `noUncheckedIndexedAccess` ligados.
- Imports com alias `$lib/...`; nunca `../../..` atravessando camadas.
- Props de componente sempre em `interface Props`.

## Estrutura de pastas (resumo)
```
crates/vx-domain/src/<assunto>.rs           entidades por assunto
crates/vx-application/src/ports/<port>.rs   um trait por arquivo
crates/vx-application/src/use_cases/<caso>.rs
crates/vx-infra/src/<adapter>/mod.rs
apps/web/src/lib/{domain,ports,adapters,application,ui}
```

## Git
- Branch principal: `main`. Branches: `feat/…`, `fix/…`, `docs/…`, `refactor/…`, `test/…`.
- [Conventional Commits](https://www.conventionalcommits.org/pt-br/): `tipo(escopo): descrição no imperativo`.
  Tipos: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `chore`, `ci`.
- PRs pequenos (idealmente < 400 linhas), um assunto por PR.
- Versionamento semântico (`MAJOR.MINOR.PATCH`); `CHANGELOG.md` gerado a partir dos commits.

## Formatação
- `.editorconfig`: UTF-8, LF, 2 espaços (4 em Rust).
- Rust: `rustfmt.toml` (100 colunas). TS/Svelte: Prettier (100 colunas, aspas simples).
