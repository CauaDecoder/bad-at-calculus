# ADR 0004 — Bibliotecas matemáticas puras no domínio

- **Status:** aceito
- **Data:** 2026-09-28

## Contexto
O ADR 0002 definiu que `mv-domain` não depende de nada além de `std`. Com a v0.3 (Vetores e
Geometria Analítica) surgiu a necessidade de autovalores de matrizes quaisquer, inclusive
complexos. Implementar a decomposição de Schur à mão seria longo, frágil e fora do foco do
projeto — enquanto bibliotecas maduras já resolvem isso.

## Decisão
Bibliotecas **puramente numéricas** podem ser dependências do domínio, desde que:
- não façam I/O, não tenham estado global nem dependam de frameworks;
- não vazem para a API pública do domínio (o domínio expõe seus próprios tipos:
  `Matrix`, `Eigenvalue`… e converte internamente);
- sejam declaradas em `[workspace.dependencies]` com comentário explicando o uso.

Hoje: **`nalgebra`** (autovalores via Schur em `linear_algebra/eigen.rs`).

O que **continua escrito à mão**, por ser o conteúdo que o estudante precisa ver passo a passo:
escalonamento de Gauss–Jordan, derivação simbólica, limites, primitivas e integrais numéricas.

## Alternativas consideradas
- **Manter o domínio 100% `std`** — exigiria implementar QR/Schur; custo alto, risco de bugs numéricos.
- **Colocar `nalgebra` em `mv-infra` atrás de um port** — cerimônia sem ganho: cálculo de
  autovalores é uma regra matemática, não um detalhe de infraestrutura trocável.

## Consequências
- ✅ Autovalores reais e complexos corretos com pouco código.
- ✅ O domínio segue testável sem mocks e compilável para WASM (`nalgebra` é Rust puro).
- ⚠️ O tamanho do WASM cresceu (~360 KB). Aceitável; revisar se passar de 1 MB.
- ⚠️ Revisões de PR devem barrar bibliotecas com I/O, `serde` ou frameworks no domínio.
