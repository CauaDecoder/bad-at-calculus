# Camadas em detalhe

## 1. Domain — `crates/vx-domain`

**O que é:** a matemática. Tipos e funções puras que seriam verdade em qualquer aplicação.

**Contém**, organizado por assunto:

| Módulo           | Conteúdo                                                                         |
| ---------------- | -------------------------------------------------------------------------------- |
| `expression`     | `Expr` (AST), funções por partes, avaliação, `simplified()`, `to_latex()`        |
| `calculus`       | `RealFunction`, amostragem, limites e continuidade, derivação simbólica com passos, pontos críticos, integral (Simpson), somas de Riemann, primitivas |
| `vectors`        | `Vector3`: escalar, vetorial, norma, ângulo, projeção, produto misto             |
| `geometry`       | `Point2`, `Line3`, `Plane` (distâncias, projeções)                                |
| `linear_algebra` | `Matrix`, Gauss–Jordan com passos, determinante, inversa, sistemas, autovalores  |

**Pode importar:** `std` e bibliotecas numéricas puras (`nalgebra`) — ver
[ADR 0004](adr/0004-bibliotecas-matematicas-no-dominio.md).
**Não pode:** I/O, `serde`, parser, logs, frameworks, `unwrap`.

```rust
// ✅ Regra matemática pura, testável sem nada ao redor
pub fn cross(self, other: Self) -> Self {
    Self::new(
        self.y * other.z - self.z * other.y,
        self.z * other.x - self.x * other.z,
        self.x * other.y - self.y * other.x,
    )
}
```

## 2. Application — `crates/vx-application`

**O que é:** o que o estudante quer fazer. Cada caso de uso = uma intenção.

**Contém:**
- `use_cases/calculus/` — `SampleFunction`, `ComputeTangentLine`, `DifferentiateExpression`,
  `FindCriticalPoints`, `AnalyzeLimit`, `ComputeIntegral`, `FormatExpressionAsLatex`.
- `use_cases/vectors.rs` — `AnalyzeVectors`, `AnalyzePlane`.
- `use_cases/linear_algebra.rs` — `AnalyzeMatrix`, `SolveLinearSystem`.
- `use_cases/test_support.rs` — fakes dos ports para os testes.
- `ports/` — *traits* das dependências externas: `ExpressionParser`.
- `ApplicationError` — agrega erros do domínio e dos ports; `InvalidField` indica **qual campo**
  do formulário está errado (`u.y`, `a[1][2]`, `lower`).

Números digitados pelo estudante (`1/2`, `sqrt(3)`, `pi/4`) são interpretados pelo mesmo parser
das funções e avaliados como constantes — por isso frações e raízes são aceitas em qualquer campo.

**Pode importar:** `vx-domain`, `thiserror`.
**Não pode:** conhecer implementações (`PrattParser`), Tauri, WASM, arquivos.

## 3. Interface adapters — `crates/vx-infra` e `crates/vx-api`

- **`vx-infra`**: `parser::PrattParser` (implementa `ExpressionParser`), `storage` (planejado).
- **`vx-api`**: o contrato do motor — comandos, DTOs em JSON camelCase, `ApiError { code,
  message, position?, field? }` e a função `dispatch`. Ver [ADR 0005](adr/0005-mv-api-dispatch-unico.md).

**Podem importar:** `vx-application`, `vx-domain`, bibliotecas técnicas (`serde`…).
**Não podem:** conhecer Tauri, WASM ou a UI.

Para os contratos de conteúdo e correção da F1, `serde`, `schemars` e `ts-rs` ficam somente
em `vx-api`, não em `vx-infra` ou nas camadas internas. Os DTOs geram JSON Schema e TypeScript
e são convertidos explicitamente para os tipos puros. `content::Part` consome
`assessment::AnswerSpec` pela fachada pública Rust do domínio; casos de uso de conteúdo
consomem a fachada pública de aplicação do corretor, nunca `dispatch`. Ver
[ADR 0016](adr/0016-contrato-de-conteudo-f1.md).

## 4. Frameworks & drivers — `crates/vx-wasm`, `apps/desktop`, `apps/web`

**O que é:** a "cola" com o mundo real. **Uma função cada**, sem regras de negócio:

- `vx-wasm`: `dispatch(command, request)` → `vx_api::dispatch`.
- `apps/desktop/src-tauri`: comando `engine(command, request)` → `vx_api::dispatch`.
- `apps/web`: interface Svelte, com suas próprias camadas ([frontend.md](frontend.md)).

## Como adicionar uma funcionalidade (receita)

Exemplo: "distância entre duas retas reversas".

1. **Domínio:** `Line3::distance_to_line(&self, other) -> f64` + testes.
2. **Aplicação:** caso de uso (ou campo novo no `PlaneReport`) + teste com fakes.
3. **API:** DTO + handler em `vx-api/src/vectors.rs`; registrar o comando.
4. **Front:** tipo em `lib/domain`, método no port e no adapter, estado em `lib/application`,
   componente em `lib/ui/vectors`.
5. `pnpm wasm:build`.
6. **Docs:** atualizar [módulos](../02-produto/modulos.md) se for funcionalidade nova.

## Sinais de violação (code review)

- `use vx_infra` dentro de `vx-application` ou `vx-domain`.
- `serde` derivado em tipos do domínio (use DTOs em `vx-api`).
- Lógica em `vx-wasm` ou `commands.rs` além de repassar para `dispatch`.
- `import` de `@tauri-apps/*`, `$lib/wasm` ou `$lib/adapters` em `lib/ui` ou `lib/application`.
- Cálculo matemático dentro de componente Svelte (formatar para LaTeX é apresentação e pode).
