# ADR 0006 — Plataforma Vértice: nome e módulos

- **Status:** aceito (o renome no código é executado na Fase 0)
- **Data:** 2026-10-14

## Contexto
O projeto nasceu como *Math View*, um laboratório visual de Cálculo e VGA. Ele agora vai virar uma
plataforma de estudos para exatas, computação (ADS) e arquitetura (AU), com exercícios, autoria,
revisão e, no futuro, contas e comunidade. O nome "Math View" descreve só uma parte e afasta
contribuidores de ADS e AU. O repositório no GitHub ainda se chama `bad-at-calculus`.

## Decisão
- A plataforma passa a se chamar **Vértice** (ASCII: `vertice`). O Math View vira o módulo
  **Laboratório**.
- Navegação por **módulos de produto**: Laboratório (`/lab`), Exercícios, Autoria, Revisão, Plano de
  estudos, Progresso ([módulos](../../02-produto/modulos.md)). As rotas atuais (`/calculo/*`,
  `/vetores/*`) passam para `/lab/...` com redirecionamento.
- Renome técnico, **na Fase 0** e em PR próprio:

| Hoje | Depois |
| ---- | ------ |
| repositório `bad-at-calculus` | `vertice` |
| crates `mv-domain`, `mv-application`, `mv-infra`, `mv-api`, `mv-wasm` | `vx-domain`, `vx-application`, `vx-infra`, `vx-api`, `vx-wasm` |
| `math-view-desktop` (Tauri) | `vertice-desktop` |
| `@math-view/web` | `@vertice/web` |
| workspace `.mathview` (planejado) | `.vertice` |

## Verificação do nome (2026-10-14)
- npm: pacote `vertice` livre (404). O escopo `@vertice` ainda precisa ser confirmado ao criar a org.
- crates.io: `vertice`, `vx-api`, `vx-domain` livres.
- GitHub: o usuário/org `vertice` existe, mas não importa, porque o repo fica em `CauaDecoder/vertice`.
- Existe uma editora/plataforma de e-learning espanhola chamada **Vértice** (vertice.org). Risco
  baixo para um projeto open source sem fins comerciais, mas vale evitar logotipo parecido. Se
  algum dia houver uso comercial, refazer a busca de marca (INPI).

## Alternativas consideradas
- **Manter "Math View"** — zero esforço, mas o nome não cobre ADS e AU.
- **Eixo, Nabla, Quadro** — "Nabla" é forte só para exatas; "Eixo" e "Quadro" são genéricos e mais
  sujeitos a colisão.

## Consequências
- ✅ Nome neutro entre os três cursos, curto e em PT-BR.
- ⚠️ O renome toca muitos arquivos (Cargo.toml, imports, ESLint, docs, ADRs antigos). Fazer num
  PR mecânico isolado, sem mudança de comportamento, com `cargo test` e `pnpm test` verdes.
- ⚠️ ADRs 0001–0005 continuam falando em `mv-*`. Não serão reescritos; este ADR registra o mapeamento.
