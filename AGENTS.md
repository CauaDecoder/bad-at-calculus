# AGENTS.md — guia para agentes de IA

Contexto mínimo para um agente (Claude Code, Codex, Cursor, Copilot…) contribuir sem quebrar a
arquitetura. Humanos também podem ler.

## O projeto em 30 segundos

**Vértice** é uma plataforma de estudos open source para exatas, computação (ADS) e arquitetura
(AU). Ela nasceu como **Math View**, cujo laboratório visual de Cálculo e VGA é hoje o módulo
**Laboratório**. Próximos módulos: Exercícios (correção automática), Autoria (criar/importar),
Revisão (FSRS) e, na Fase 3, contas e nuvem.
- Núcleo **Rust** (Clean Architecture) servido por **WASM** (web) e **Tauri** (desktop); servidor
  **Axum** só na Fase 3.
- Front **SvelteKit 5** estático.
- Local-first, sem contas até a F3.

Leia nesta ordem quando precisar de contexto:
1. [visão](docs/01-visao/visao-do-produto.md)
2. [roadmap](docs/02-produto/roadmap.md)
3. [arquitetura](docs/03-arquitetura/visao-geral.md)
4. o ADR do assunto em [docs/03-arquitetura/adr/](docs/03-arquitetura/adr/)

> Em transição: crates e pacotes ainda usam `mv-*` / `@math-view/*`. O renome para `vx-*` /
> `@vertice/*` é da Fase 0 ([ADR 0006](docs/03-arquitetura/adr/0006-plataforma-vertice.md)).
> Não renomeie nada fora do PR de renome.

## Estilo de resposta: caveman (padrão, sempre)

Todo agente responde no chat em **modo caveman**: resposta primeiro, sem cerimônia, todo fato
técnico preservado. Se a ferramenta tiver a skill `caveman` (ex.: Claude Code), use-a desde a
primeira resposta da sessão, sem esperar `/caveman`. Sem a skill, siga as regras abaixo.

- Resposta primeiro, depois motivo, depois próximo passo. Sem saudação, rodeio, recapitulação ou
  oferta de ajuda no fim.
- Frases curtas, uma ideia por frase, voz ativa. Pode omitir artigos; nunca omita "não", "nunca",
  "só", "exceto".
- Código, comandos, caminhos, números e mensagens de erro sempre literais.
- Idioma do usuário (PT-BR neste projeto). Comprime o estilo, não o idioma.
- **Prosa normal** (depois volta ao caveman) para: aviso de segurança, confirmação de ação
  irreversível, passos cuja ordem importa, usuário confuso e perguntas/opções ao usuário.
- **Não se aplica ao que fica gravado**: código, comentários, commits, docs, ADRs, PRs, issues e
  conteúdo de exercícios seguem escritos normalmente, conforme as regras deste arquivo.
- Desliga só quando o usuário pedir ("stop caveman" / "normal mode").

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
| `content/`                     | Banco público (CC BY-SA) | — (dados; validados pelo núcleo a partir da F1) |
| `content-private/`             | **Fora do git**      | material de terceiros; nunca ler para publicar, nunca commitar |
| `docs/`                        | Documentação PT-BR   | —                                              |

**Regra de ouro:** dependências apontam para dentro. Nunca importe `mv-infra` a partir de
`mv-application`, nem `@tauri-apps/*` ou `$lib/adapters` a partir de `lib/ui` ou
`lib/application` (o ESLint bloqueia).

**Contextos novos** (`assessment`, `content`, `review`, `progress`) são **módulos dentro das camadas
existentes**, não crates novas, e um contexto não importa internos de outro
([ADR 0007](docs/03-arquitetura/adr/0007-contextos-como-modulos.md)).

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
- LaTeX em atributos Svelte: use `formula={String.raw\`...\`}`, porque chaves `{}` em atributos
  literais são interpretadas como JavaScript.
- Resultados numéricos na UI: `equalsLatex(v)` (mostra `= 1/2` exato ou `≈ 1,23`).
- Mudou arquitetura? Adicione um ADR em `docs/03-arquitetura/adr/` (modelo em `template.md`).
- **Conteúdo de terceiros** (listas, apostilas, slides) **nunca** entra no git nem em `content/`.
  Exercícios públicos são autorais ou de fonte aberta compatível ([ADR 0012](docs/03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
- **Segurança**: Markdown/SVG/HTML vindos do usuário ou de IA são não confiáveis; sanitize. Nunca
  coloque segredos no código ou em logs ([modelo de ameaças](docs/06-seguranca/modelo-de-ameacas.md)).
- **IA no produto**: o motor calcula e corrige; o LLM só explica ou extrai
  ([estratégia de IA](docs/08-ia/estrategia-ia.md)).

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

## Papéis

Papéis de desenvolvimento usados por agentes (em Claude Code, como subagentes em
`.claude/agents/`). Cada papel tem missão, leitura obrigatória e Definition of Done (DoD). Um
agente sempre **declara o papel** que está exercendo e **não sai do escopo** dele: se o trabalho
exige outro papel, ele devolve a recomendação em vez de fazer.

### product-manager
- **Missão:** transformar necessidades de estudantes (exatas, ADS, AU) em problemas claros,
  histórias com critérios de aceite e prioridades no roadmap.
- **Lê:** visão, módulos, roadmap, PRDs em `docs/02-produto/`.
- **Entrega:** PRDs e histórias no formato "Como… quero… para…" com critérios Dado/Quando/Então;
  atualização de `roadmap.md` e `modulos.md`.
- **DoD:** cada história tem critério verificável e métrica; escopo "fora" explícito; nenhuma
  decisão técnica tomada no lugar do arquiteto.

### software-architect
- **Missão:** guardar a regra de dependência, os contratos (`dispatch`, formato de exercício) e o
  modelo de dados; registrar decisões.
- **Lê:** `docs/03-arquitetura/` inteiro, ADRs.
- **Entrega:** ADRs (`template.md`), diagramas Mermaid (C4), revisão de PRs que mudem fronteiras.
- **DoD:** alternativas e consequências registradas; diagramas atualizados; nenhum ADR antigo
  reescrito (só "substituído por").

### backend-developer
- **Missão:** implementar regras e casos de uso em Rust (motor, `assessment`, `content`, futuro
  `vx-server`) seguindo a receita acima.
- **Lê:** camadas, motor matemático, formato de exercícios, ADRs 0005/0007/0008/0009, clean-code.
- **DoD:** fmt + clippy `-D warnings` + testes verdes; teste para cada regra nova e cada erro;
  comando registrado em Rust **e** em `commands.ts`; sem `unwrap`.

### frontend-developer
- **Missão:** UI em SvelteKit/Svelte 5 (runes), visualizações (Canvas 2D, câmera 3D, Threlte
  quando houver WebGL), acessibilidade e design system.
- **Lê:** frontend, design system, princípios de UX, componentes.
- **DoD:** `check`, `lint` e `test` verdes; navegação por teclado; textos em `lib/ui/copy/`;
  Markdown do usuário sanitizado; sem import proibido pelo ESLint.

### qa-engineer
- **Missão:** garantir que funciona: estratégia de testes, casos a partir de gabaritos reais,
  testes de propriedade, E2E (Playwright), regressão de bugs.
- **Lê:** testes, PRD da funcionalidade, casos de uso.
- **Entrega:** testes e relatórios de verificação (o que rodou, saída, o que não foi verificado).
- **DoD:** todo bug corrigido tem teste que o reproduz; relatório honesto, sem omitir falhas.

### security-researcher
- **Missão:** revisar código e design contra **OWASP Top 10:2025, ASVS 5.0 e OWASP LLM Top 10
  2025**; manter o modelo de ameaças; rodar scanners.
- **Lê:** `docs/06-seguranca/`, ADRs de fronteira (0008, 0010, 0011, 0012).
- **Entrega:** relatório com achados por severidade (risco OWASP/ASVS, arquivo:linha, cenário de
  exploração, correção sugerida). **Não edita código.**
- **DoD:** cada achado é reproduzível ou explicitamente marcado como hipótese; nada de exploração
  fora do repositório/ambiente local.

### devops-engineer
- **Missão:** CI/CD, builds (WASM, Tauri, Docker multi-arch), deploy (Cloudflare, Oracle via
  Coolify), IaC (OpenTofu), observabilidade, backups; tudo em free tier.
- **Lê:** `docs/07-devops/`, modelo de ameaças (T2/T3).
- **DoD:** Actions fixadas por SHA e com `permissions` mínimas; segredos nunca em PR de fork;
  pipeline reproduzível; custo zero confirmado contra os limites documentados.

### ai-engineer
- **Missão:** funcionalidades de IA do produto: importador de PDF, dicas/tutor, `vertice-mcp`,
  busca semântica; port `LlmProvider`; avaliações.
- **Lê:** `docs/08-ia/estrategia-ia.md`, formato de exercícios, OWASP LLM Top 10.
- **DoD:** saída estruturada validada pelo JSON Schema; conjunto de avaliação versionado com
  métricas; conteúdo `private` nunca enviado sem consentimento; LLM nunca decide correção.

### content-author
- **Missão:** modelar exercícios no formato da Vértice e validar respostas contra gabaritos
  usando o motor; cuidar da taxonomia e das licenças.
- **Lê:** formato de exercícios, taxonomia, ADR 0012.
- **Entrega:** `*.ex.md` em `content/` (autorais, CC BY-SA) ou packs privados fora do git.
- **DoD:** cada resposta esperada conferida (calculada ou checada pelo motor); `source` e `license`
  preenchidos; nada de terceiros em `content/`.

## Onde ler mais

- Arquitetura: [docs/03-arquitetura/visao-geral.md](docs/03-arquitetura/visao-geral.md)
- Motor matemático: [docs/03-arquitetura/motor-matematico.md](docs/03-arquitetura/motor-matematico.md)
- Formato de exercícios: [docs/02-produto/formato-de-exercicios.md](docs/02-produto/formato-de-exercicios.md)
- Convenções: [docs/05-engenharia/convencoes.md](docs/05-engenharia/convencoes.md)
- Design system: [docs/04-design/design-system.md](docs/04-design/design-system.md)
- Segurança: [docs/06-seguranca/modelo-de-ameacas.md](docs/06-seguranca/modelo-de-ameacas.md)
