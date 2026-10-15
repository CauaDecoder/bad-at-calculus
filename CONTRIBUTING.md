# Contribuindo com o Math View

Obrigado por querer contribuir! Este guia resume o essencial. Os detalhes vivem em `docs/`.

## 1. Antes de começar

1. Leia a [visão do produto](docs/01-visao/visao-do-produto.md) e a
   [visão geral da arquitetura](docs/03-arquitetura/visao-geral.md).
2. Configure o ambiente: [setup-ambiente.md](docs/05-engenharia/setup-ambiente.md).
3. Procure uma issue com a label `good first issue` ou abra uma descrevendo sua ideia.

## 2. Fluxo de trabalho

1. Faça um fork e crie um branch a partir de `main`:
   `feat/calculus-tangent-line`, `fix/parser-unary-minus`, `docs/design-tokens`.
2. Faça commits pequenos seguindo [Conventional Commits](https://www.conventionalcommits.org/pt-br/):
   ```
   feat(calculus): compute tangent line at a point
   fix(parser): handle implicit multiplication "2x"
   docs(architecture): add ADR for plotting engine
   ```
   Escopos comuns: `domain`, `application`, `infra`, `api`, `wasm`, `desktop`, `web`,
   `calculus`, `vectors`, `linear-algebra`, `parser`, `plot`, `docs`.
3. Abra um Pull Request pequeno e focado, preenchendo a descrição (o quê, por quê, como testar).

## 3. Idioma

- **Documentação** (`docs/`, READMEs): português (PT-BR).
- **Código** (nomes, comentários, mensagens de erro internas, commits): inglês.
- **Textos da interface**: via arquivos de tradução, nunca hardcoded.

## 4. Checklist do Pull Request

- [ ] Respeita a regra de dependência das camadas ([camadas.md](docs/03-arquitetura/camadas.md)).
- [ ] Nomes descritivos, funções pequenas que fazem uma coisa ([clean-code.md](docs/05-engenharia/clean-code.md)).
- [ ] Sem `unwrap()`/`expect()` em código de biblioteca; erros tipados com `Result`.
- [ ] Testes para toda regra nova no domínio e nos casos de uso ([testes.md](docs/05-engenharia/testes.md)).
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --workspace --exclude math-view-desktop` passando.
- [ ] `pnpm lint`, `pnpm check` e `pnpm -C apps/web test` passando (se mexeu no front).
- [ ] Operação nova no motor? Comando registrado em `mv-api` **e** em `adapters/engine/commands.ts`
      (receita em [AGENTS.md](AGENTS.md)).
- [ ] Documentação atualizada se mudou comportamento, arquitetura ou design.
- [ ] Decisão arquitetural relevante? Registre um [ADR](docs/03-arquitetura/adr/template.md).

## 5. Código de conduta

Este projeto segue o [Código de Conduta](CODE_OF_CONDUCT.md). Seja gentil.
