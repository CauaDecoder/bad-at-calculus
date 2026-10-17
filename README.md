# Vértice

> Plataforma de estudos open source para exatas, computação e arquitetura: **estudar** com
> material de procedência clara, **ver** o conceito acontecendo, **praticar** com correção
> imediata e **revisar** no momento certo.

A Vértice nasceu como **Math View**, um laboratório visual de Cálculo e Vetores, que hoje é o
módulo **Laboratório**. O núcleo é escrito em **Rust** (motor matemático e correção de
exercícios) e a interface em **Svelte + TypeScript**. Roda no navegador (**WebAssembly**), no
desktop (**Tauri**) e, no futuro, num servidor **Axum**.

> O renome técnico local usa crates `vx-*` e pacotes `@vertice/*`
> ([ADR 0006](docs/03-arquitetura/adr/0006-plataforma-vertice.md)). O renome do repositório no
> GitHub e a confirmação da identidade para publicação externa continuam separados.

## Módulos

| Módulo | O que faz | Status |
| ------ | --------- | ------ |
| **Laboratório** | Visualizações interativas: limites, derivadas, integrais, vetores 3D, retas e planos, matrizes, sistemas | ✅ |
| **Exercícios** | Listas com correção automática de expressões, equações, conjuntos e mais; dicas; "Abrir no laboratório" | 🔜 F1 |
| **Biblioteca** | Material de estudo: papers, livros abertos, notas de aula e arquivos próprios; leitor de PDF com destaques que viram cartões; roteiros de leitura; licença, selo de confiança e status por item | 🔜 F1 (referências) · F2 (local) · F3–F4 |
| **Autoria** | Criar exercícios, importar em lote com gabarito, importar PDF com IA | 🔜 F1–F2 |
| **Revisão e Plano de estudos** | Repetição espaçada (FSRS), caderno de erros, plano até a prova | 🔜 F2 |
| **Contas e comunidade** | Sync, listas compartilhadas, turmas | 🔜 F3–F4 |

Disciplinas: Cálculo e VGA hoje; Física I, Métodos numéricos, POO, Algoritmos, Banco de Dados,
Estruturas (AU) e Geometria Descritiva planejadas — ver [módulos](docs/02-produto/modulos.md).

## O que já dá para estudar (Laboratório)

**Cálculo** — limites e continuidade (laterais, no infinito, tipo de descontinuidade); derivadas
com tangente arrastável e passos regra por regra; integrais com Riemann e TFC. Funções por partes:
`{x < 1: x^2; x >= 1: 2x}`.

**Vetores e Geometria Analítica** — operações em 3D, retas e planos, matrizes (determinante,
inversa, escalonamento, autovalores), sistemas lineares com Gauss–Jordan.

Resultados em forma exata (`√6/9`, `π/3`, `8/3`).

## Estrutura do repositório

```
crates/        núcleo Rust (Clean Architecture)
  vx-domain/        regras: expressões, cálculo, vetores, geometria, álgebra linear (+ correção, F1)
  vx-application/   casos de uso + ports (traits)
  vx-infra/         adaptadores: parser, persistência
  vx-api/           contrato: comandos, DTOs JSON, erros estruturados
  vx-wasm/          ponte WebAssembly (uma função: dispatch)
apps/
  web/              interface SvelteKit + TypeScript
  desktop/          empacotamento desktop com Tauri 2
content/       banco público de exercícios (CC BY-SA 4.0) — F1
docs/          documentação (PT-BR)
.claude/       agentes e skills de desenvolvimento
```

## Começando

Pré-requisitos e passo a passo: [docs/05-engenharia/setup-ambiente.md](docs/05-engenharia/setup-ambiente.md).

```bash
pnpm install
pnpm wasm:build   # compila o motor Rust para WebAssembly
pnpm dev          # interface web em http://localhost:5173
pnpm desktop:dev  # app desktop (Tauri)
cargo test --workspace --exclude vertice-desktop
```

## Documentação

Comece pelo [índice](docs/README.md): visão, PRDs de exercícios e da Biblioteca, arquitetura (C4 + ADRs), segurança
(OWASP), DevOps, IA e a pesquisa de referências.

Para implementar com agentes de terminal, use os [guias por fase](docs/05-engenharia/implementacao/README.md).
Cada unidade pode ser solicitada indicando apenas seu ID e arquivo.

## Contribuindo

Contribuições de código **e de conteúdo** são bem-vindas! Leia o [CONTRIBUTING.md](CONTRIBUTING.md).
Agentes de IA: veja [AGENTS.md](AGENTS.md). Vulnerabilidades: [SECURITY.md](SECURITY.md).

## Licença

- **Código:** MIT ou Apache 2.0, à sua escolha ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)).
- **Conteúdo** em `content/`: [CC BY-SA 4.0](content/LICENSE).
- Material de terceiros (listas, apostilas) **não** faz parte do repositório
  ([ADR 0012](docs/03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
