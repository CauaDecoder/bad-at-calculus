# Pesquisa — plataformas e tecnologias de referência

Levantamento feito em **2026-10-14** para modelar a Vértice. Cada item diz **o que é**, **o que
aproveitamos** e **o que evitamos**. Atualize quando uma decisão depender de um fato que mudou.

## 1. Avaliação e exercícios de exatas

| Projeto | O que é | Aproveitamos | Evitamos |
| ------- | ------- | ------------ | -------- |
| **STACK** (Edinburgh; Moodle/ILIAS) | Questões com resposta algébrica, validação da entrada, aleatorização, checagem linha a linha, CAS Maxima no servidor | Validar a entrada antes de corrigir; **feedback por erro comum** (árvores de resposta) | Depender de Moodle e de CAS no servidor |
| **Numbas** (Newcastle, Apache 2.0) | Avaliação matemática **toda no cliente**, pacotes SCORM; compara expressões com a resposta guardada | Correção offline por avaliação numérica | Formato SCORM como formato interno |
| **WeBWorK** + Open Problem Library | Sistema de tarefas com ~35 mil problemas abertos, metadados por curso/capítulo/seção, linguagem PG (Perl + LaTeX) | Biblioteca aberta com taxonomia hierárquica que cresce por contribuição | Linguagem de autoria baseada em Perl |
| **OATutor** (Berkeley, MIT + CC BY) | Tutor adaptativo com Bayesian Knowledge Tracing; conteúdo em JSON (OpenStax) com dicas e *scaffolds*; roda em GitHub Pages | Conteúdo como código, dicas em passos, domínio por habilidade, deploy estático | — |
| **Perseus** (Khan Academy) / **OpenPerseus** | Editor e renderizador de questões interativas em JSON; `itemDataVersion` + versão por widget | Versionamento do schema e das visualizações embutidas | Acoplamento a React/Khan; suporte externo descontinuado |
| **QTI 3.0** (1EdTech) | Padrão XML de interoperabilidade de itens e testes | Exportador futuro (F4) | Usar como formato interno (verboso) |

Fontes: [STACK](https://stack.maths.ed.ac.uk) · [STACK no Moodle](https://docs.moodle.org/en/question/type/stack) ·
[Numbas](https://numbas.org.uk) · [docs Numbas](https://docs.numbas.org.uk) ·
[WeBWorK OPL](https://webwork.maa.org/wiki/Open_Problem_Library) · [OATutor-Content](https://github.com/CAHLR/OATutor-Content) ·
[OpenPerseus](https://cdn.jsdelivr.net/npm/openperseus@1.2.0/README.md) · [QTI](https://www.1edtech.org/standards/qti/index)

## 2. Entrada matemática e visualização

| Projeto | Aproveitamos |
| ------- | ------------ |
| **MathLive + Compute Engine** (CortexJS, MIT) | Candidato a campo de resposta (teclado virtual, LaTeX/MathJSON). A comparação `isSame()` é estrutural, então a equivalência continua no motor Rust. Spike na F1 |
| **JSXGraph** (Bayreuth, MIT/LGPL) | Referência para construções geométricas (geometria descritiva) |
| **Mafs** (React) / **function-plot** (D3) | Referências de API para o registro de visualizações; não adotados (o Canvas próprio já é mais rápido e coeso) |
| **Threlte** (Svelte + three.js) | WebGL para superfícies 3D e cenas de AU (estruturas, épura) |
| **GeoGebra** | Referência pedagógica de geometria descritiva; licença não comercial, por isso não é embutido |

Fontes: [Compute Engine](https://mathlive.io/compute-engine/) · [compute-engine (GitHub)](https://github.com/cortex-js/compute-engine) ·
[JSXGraph](https://jsxgraph.uni-bayreuth.de/home/) · [function-plot](https://github.com/mauriciopoppe/function-plot) ·
[Threlte](https://cdn.jsdelivr.net/npm/@threlte/core@8.5.16/README.md)

## 3. Aprendizagem e memória

- **FSRS** (Open Spaced Repetition): agendador de revisão espaçada usado pelo Anki, com
  implementações em TS (`ts-fsrs`, FSRS v6) e Rust (`fsrs`, BSD-3), incluindo otimização de
  parâmetros pelo histórico. → Módulo Revisão. [open-spaced-repetition](https://github.com/open-spaced-repetition)
- **Knowledge tracing** (BKT, como no OATutor): estimar domínio por habilidade → Progresso (F2+).

## 4. Local-first e sync

- **Zero 1.0** (Rocicorp) é o sync engine mais maduro em 2026: replica Postgres e sincroniza
  subconjuntos por consulta. Pontos fracos: bundle ~230 KB gz, SSR ainda no roadmap. Existe binding Svelte.
- **ElectricSQL + PGlite**: Postgres em WASM no cliente, com paridade de engine com o servidor.
- **PowerSync, InstantDB, Jazz, Triplit**: alternativas, com maturidade variada.
- Decisão: adiar e isolar atrás de ports ([ADR 0010](../03-arquitetura/adr/0010-local-first-na-fase-1.md)).

Fontes: [Electric — alternativas](https://electric-sql.com/docs/reference/alternatives) · [Zero 1.0 (InfoQ)](https://infoq.com/news/2026/06/zero-version-1)

## 5. Documentos e IA

- **PDF → Markdown/LaTeX**: Marker (76% no olmOCR-bench, opção `--use_llm`), MinerU (fórmulas →
  LaTeX, Apache 2.0, 109 idiomas), Docling (mais voltado a RAG), olmOCR (texto limpo). Testar nas
  nossas listas antes de escolher. [comparativo 2026](https://themenonlab.blog/blog/best-open-source-pdf-to-markdown-tools-2026)
- **Observabilidade e avaliação de LLM**: Langfuse (MIT exceto `ee`, adquirido pela ClickHouse em
  01/2026; self-host pesado), promptfoo (MIT, evals na CI). [comparativo](https://www.respan.ai/market-map/compare/langfuse-vs-promptfoo)
- **Execução de código no navegador**: Pyodide; Monty (interpretador Python em Rust/WASM, 2026);
  Judge0 (servidor, 60+ linguagens) para a F3. [Monty + Pyodide](https://simonwillison.net/2026/Feb/6/monty-wasm-pyodide/)

## 6. Backend, auth e infra

- **Rust**: Axum 0.8 + Tokio + SQLx é o padrão consolidado; SeaORM como alternativa ORM.
  [guia 2026](https://devstarsj.github.io/2026/07/11/rust-backend-axum-sqlx-production-guide-2026/)
- **Auth (TS)**: Better Auth (plugin de passkeys, integração SvelteKit); o Auth.js passou a fazer
  parte do Better Auth (o passkey do Auth.js ainda é experimental); Lucia foi descontinuado.
  [Better Auth + SvelteKit](https://www.better-auth.com/docs/examples/svelte-kit) · [Auth.js passkeys](https://authjs.dev/getting-started/providers/passkey)
- **PaaS self-hosted**: Coolify v4 (estável desde 04/2026, ARM64, guia para Oracle) vs. Dokploy
  (mais leve, Swarm, pré-1.0). [comparativo](https://www.virtua.cloud/learn/en/concepts/coolify-vs-dokploy-self-hosted-paas) ·
  [Coolify + Oracle](https://coolify.io/docs/knowledge-base/server/oracle-cloud)
- **Free tiers**: Oracle ARM possivelmente reduzido a 2 OCPU/12 GB (ago/2026); Cloudflare D1 500 MB
  por banco. [Oracle 2026](https://terminalbytes.com/oracle-cloud-free-tier-changes-2026) ·
  [D1 limits](https://developers.cloudflare.com/d1/platform/limits) — detalhes em [infra-free-tier.md](../07-devops/infra-free-tier.md)

## 7. Segurança

- **OWASP Top 10:2025**: A03 *Software Supply Chain Failures* e A10 *Mishandling of Exceptional
  Conditions* são novos; SSRF foi incorporado a A01. [owasp.org/Top10/2025](https://owasp.org/Top10/2025/)
- **ASVS 5.0** (maio/2025): ~350 requisitos, 17 capítulos, 3 níveis. [ASVS](https://owasp.org/www-project-ASVS/)
- **OWASP Top 10 for LLM 2025**: novos *System Prompt Leakage* e *Vector and Embedding Weaknesses*.
  [resumo](https://www.hackerone.com/blog/owasp-top-10-llms-2025-how-genai-risks-are-evolving)
- Aplicação ao projeto: [docs/06-seguranca](../06-seguranca/owasp.md).

## 8. Arquitetura e Urbanismo

Não há ferramenta web open source consolidada para geometria descritiva: existem projetos
acadêmicos (HyperCAL3D na UFRGS, WinCAG, Geometry Mapping Tool) e uso do GeoGebra 3D. Isso é uma
**oportunidade**: épura de Monge sincronizada com vista 3D, mais estruturas (forças, treliças,
esforços) sobre o motor de vetores que já existe.
[HyperCAL3D](https://professor.ufrgs.br/fernando/publications/hypercal3d-aplicativo-de-apoio-ao-ensino-de-geometria-descritiva)

## 9. Agentes de desenvolvimento

- Claude Code lê `CLAUDE.md`; a forma recomendada de reaproveitar o `AGENTS.md` é a linha
  `@AGENTS.md` (import), que no Windows evita o symlink. Subagentes ficam em `.claude/agents/*.md`
  com frontmatter (`name`, `description`, `tools`, `model`).
  [subagents](https://code.claude.com/docs/en/subagents) · [import do AGENTS.md](https://travis.media/blog/claude-md-import-agents-md)
