# Contribuindo com a Vértice

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

## 3. Contribuindo com conteúdo (exercícios)

Não precisa programar: um exercício é um arquivo Markdown.

1. Leia o [formato de exercícios](docs/02-produto/formato-de-exercicios.md) e a
   [taxonomia](docs/02-produto/taxonomia.md).
2. Crie `content/exercises/<disciplina>/<topico>/<nome>.ex.md`.
3. **Licença:** ao enviar, você licencia o conteúdo em [CC BY-SA 4.0](content/LICENSE). Só envie
   exercícios **seus** ou de fontes abertas compatíveis (com atribuição em `source`).
   **Nunca** copie listas, provas, apostilas ou slides de professores
   ([ADR 0012](docs/03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)). Se quiser usar
   uma lista da sua turma, importe-a no app como **pack privado**.
4. A CI (a partir da F1) valida o schema e confere se a resposta esperada é corrigível pelo motor.

Escopos de commit para conteúdo: `content`, `taxonomy`.

## 4. Idioma

- **Documentação** (`docs/`, READMEs): português (PT-BR).
- **Código** (nomes, comentários, mensagens de erro internas, commits): inglês.
- **Textos da interface**: via arquivos de tradução, nunca hardcoded.

## 5. Checklist do Pull Request

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
- [ ] Mexeu em importação de arquivos, Markdown/HTML do usuário, IA, auth ou CI? Revise contra o
      [modelo de ameaças](docs/06-seguranca/modelo-de-ameacas.md).
- [ ] Nenhum material de terceiros (PDF, `content-private/`) no diff.

## 6. Segurança

Encontrou uma vulnerabilidade? Não abra issue pública: siga o [SECURITY.md](SECURITY.md).

## 7. Código de conduta

Este projeto segue o [Código de Conduta](CODE_OF_CONDUCT.md). Seja gentil.
