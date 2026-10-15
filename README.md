# Math View

> Um laboratório visual, rápido e open source para matemática e física.

Math View é um software para estudantes de física e exatas explorarem **gráficos, cálculo,
vetores, geometria e métodos numéricos** com uma interface minimalista, moderna e funcional.

O núcleo matemático é escrito em **Rust** (rápido e seguro) e a interface em **Svelte + TypeScript**,
rodando como app desktop (**Tauri**) ou direto no navegador (**WebAssembly**).

## O que já dá para estudar

**Cálculo**
- **Limites e continuidade** — limites laterais e no infinito, tabela de aproximação, as três
  condições de continuidade e o tipo de descontinuidade.
- **Derivadas** — arraste o ponto e veja a tangente e `f'` sendo traçada; a derivada simbólica
  aparece **regra por regra** (produto, quociente, cadeia…), com `f''` e pontos críticos.
- **Integrais** — área com sinal, somas de Riemann e o Teorema Fundamental do Cálculo com a primitiva.

**Vetores e Geometria Analítica**
- **Operações com vetores** em 3D — escalar, vetorial, ângulo, projeção, produto misto,
  cada um resolvido com os seus números.
- **Retas e planos** — plano por três pontos, reta paramétrica, distâncias.
- **Matrizes** — determinante (Sarrus), inversa, posto, escalonamento passo a passo, autovalores.
- **Sistemas lineares** — Gauss–Jordan passo a passo e classificação SPD/SPI/SI.

Funções por partes (`{x < 1: x^2; x >= 1: 2x}`), frações e raízes em qualquer campo (`1/2`,
`sqrt(3)`), e resultados em forma exata (`√6/9`, `π/3`, `8/3`).

Detalhes em [docs/02-produto/modulos.md](docs/02-produto/modulos.md) e no
[roadmap](docs/02-produto/roadmap.md) (próximos: cônicas, técnicas de integração, métodos
numéricos e física).

## Estrutura do repositório

```
crates/        núcleo Rust (Clean Architecture)
  mv-domain/        a matemática: expressões, cálculo, vetores, geometria, álgebra linear
  mv-application/   casos de uso + ports (traits)
  mv-infra/         adaptadores: parser, persistência
  mv-api/           contrato do motor: comandos, DTOs JSON, erros estruturados
  mv-wasm/          ponte WebAssembly para o navegador (uma função: dispatch)
apps/
  web/              interface Svelte + TypeScript
  desktop/          empacotamento desktop com Tauri 2
docs/          documentação do projeto
```

## Começando

Pré-requisitos e passo a passo: [docs/05-engenharia/setup-ambiente.md](docs/05-engenharia/setup-ambiente.md).

```bash
pnpm install
pnpm wasm:build   # compila o motor Rust para WebAssembly
pnpm dev          # interface web em http://localhost:5173
pnpm desktop:dev  # app desktop (Tauri)
cargo test --workspace --exclude math-view-desktop
```

## Documentação

Comece pelo [índice da documentação](docs/README.md).

## Contribuindo

Contribuições são muito bem-vindas! Leia o [CONTRIBUTING.md](CONTRIBUTING.md) e o
[guia de Clean Code](docs/05-engenharia/clean-code.md). Agentes de IA: veja [AGENTS.md](AGENTS.md).

## Licença

Dual licenciado sob [MIT](LICENSE-MIT) ou [Apache 2.0](LICENSE-APACHE), à sua escolha.
