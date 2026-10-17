# ADR 0007 — Contextos como módulos dentro das camadas

- **Status:** aceito
- **Data:** 2026-10-14

## Contexto
A Vértice ganha contextos novos (`assessment`, `content`, `review`, `progress`) além do motor
matemático. Há duas formas de organizar o workspace Rust:
1. manter as **crates por camada** (`domain`, `application`, `infra`, `api`) e criar um módulo por
   contexto dentro de cada uma, que é o que já existe para `calculus`, `vectors` e `linear_algebra`;
2. criar **crates por contexto**, cada uma com as próprias camadas internas.

O projeto tem um mantenedor principal, colaboradores eventuais e agentes de IA. A Clean
Architecture já é verificável pelos `[dependencies]` dos `Cargo.toml`.

## Decisão
**Monólito modular com crates por camada e módulos por contexto.**
- `vx-domain/src/{assessment,content,review,progress}/`, regras puras e testes.
- `vx-application/src/use_cases/{assessment,content,…}/`, casos de uso e ports.
- `vx-api/src/{assessment,content}.rs`, comandos (`assessment.check`, `content.validate_pack`…) no
  mesmo `dispatch` ([ADR 0005](0005-mv-api-dispatch-unico.md)).
- No front, o mesmo padrão: `lib/domain/assessment.ts`, `lib/ports/exercise-repository.ts`,
  `lib/application/exercise-player.svelte.ts`, `lib/ui/exercises/`.
- Um contexto **não importa os internos de outro**. Se `assessment` precisa avaliar uma expressão,
  usa a API pública de `expression`, como um caso de uso usaria. A regra é verificada em revisão
  (agente `software-architect`) e, na Fase 1, por um teste de arquitetura (lista de `use` permitidos).

## Alternativas consideradas
- **Crates por contexto** (`vx-assessment`, `vx-content`…) — fronteiras forçadas pelo compilador,
  mas multiplica `Cargo.toml`, DTOs e camadas para um time pequeno. Pode ser adotado depois se
  algum contexto crescer o bastante para ter ritmo próprio.
- **Microsserviços** — não há escala nem equipe que justifique; custo operacional alto no free tier.

## Consequências
- ✅ Contribuidor novo segue a receita que já existe no [AGENTS.md](../../../AGENTS.md).
- ✅ O servidor da Fase 3 (`vx-server`) é só mais um driver, como `vx-wasm` e o Tauri.
- ⚠️ A fronteira entre contextos depende de disciplina e revisão; o teste de arquitetura vira
  obrigatório quando houver dois ou mais contextos novos.
