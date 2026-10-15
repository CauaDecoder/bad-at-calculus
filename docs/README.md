# Documentação da Vértice

Índice de toda a documentação. As pastas são numeradas na ordem recomendada de leitura.

## 01 · Visão
- [Visão do produto](01-visao/visao-do-produto.md) — problema, público, princípios e não-objetivos.

## 02 · Produto
- [Módulos](02-produto/modulos.md) — Laboratório, Exercícios, Autoria, Revisão, Plano, Progresso; disciplinas por curso.
- [Roadmap](02-produto/roadmap.md) — fases F0–F4 e histórico do Math View.
- [PRD — Exercícios, Autoria e Importação](02-produto/prd-exercicios.md) — histórias, critérios de aceite, escopo da F1.
- [Formato de exercícios](02-produto/formato-de-exercicios.md) — exercício, partes, tipos de resposta, listas, packs.
- [Taxonomia](02-produto/taxonomia.md) — área → disciplina → tópico → habilidade.
- [Casos de uso — Cálculo A](02-produto/casos-de-uso-calculo-a.md) — listas reais mapeadas em funcionalidades.

## 03 · Arquitetura
- [Visão geral](03-arquitetura/visao-geral.md) — C4 (contexto e containers por fase), Clean Architecture, fluxo de dados.
- [Camadas](03-arquitetura/camadas.md) — o que cada camada faz e pode importar.
- [Modelo de dados](03-arquitetura/modelo-de-dados.md) — entidades, onde cada dado mora, schema relacional.
- [Frontend](03-arquitetura/frontend.md) — Svelte, ports do motor, estado, gráficos 2D/3D e resultados exatos.
- [Motor matemático](03-arquitetura/motor-matematico.md) — parser, derivação com passos, limites, integrais, vetores, matrizes.
- [ADRs](03-arquitetura/adr/) — registro de decisões arquiteturais (0001–0013).

## 04 · Design
- [Princípios de UX](04-design/principios-ux.md)
- [Design system](04-design/design-system.md) — direção A (claro neutro): cores, tipografia, layout, movimento.
- [Direção de design](references/design/README.md) — referências, RCD aplicado e protótipos escolhidos.
- [Layout e telas](04-design/layout-e-telas.md) — wireframes.
- [Componentes](04-design/componentes.md)
- [Referências visuais](04-design/referencias.md)

## 05 · Engenharia
- [Clean Code](05-engenharia/clean-code.md) — regras com exemplos em Rust e TypeScript.
- [Convenções](05-engenharia/convencoes.md) — nomes, pastas, erros, commits.
- [Testes](05-engenharia/testes.md)
- [Setup do ambiente](05-engenharia/setup-ambiente.md)

## 06 · Segurança
- [Modelo de ameaças](06-seguranca/modelo-de-ameacas.md) — STRIDE por fronteira de confiança.
- [OWASP na Vértice](06-seguranca/owasp.md) — Top 10:2025, ASVS 5.0, LLM Top 10, ferramentas.

## 07 · DevOps
- [Ambientes e deploy](07-devops/ambientes-e-deploy.md)
- [CI/CD](07-devops/ci-cd.md)
- [Infra em free tier](07-devops/infra-free-tier.md)

## 08 · IA
- [Estratégia de IA](08-ia/estrategia-ia.md) — importador de PDF, tutor, MCP, busca semântica, execução de código.

## 09 · Pesquisa
- [Plataformas e tecnologias de referência](09-pesquisa/plataformas-de-referencia.md) — o que aproveitamos de cada uma, com fontes.

## Material de apoio
- `content-private/` (fora do git) — listas, apostilas e slides de terceiros usados como casos
  reais. Ver [ADR 0012](03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md).
- [`references/`](references/README.md) — imagens e materiais de referência visual.
