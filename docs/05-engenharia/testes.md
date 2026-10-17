# Estratégia de testes

```
            ▲  poucos   E2E (Playwright)       — fluxos críticos na interface
            │           Integração             — vx-wasm / commands com o parser real
            │           Casos de uso           — vx-application com fakes dos ports
            ▼  muitos   Unitários de domínio   — vx-domain, funções puras
```

## Domínio (`vx-domain`) — a base
- Testes no próprio arquivo (`#[cfg(test)] mod tests`).
- Use as funções das [listas de Cálculo A](../02-produto/casos-de-uso-calculo-a.md) como casos reais
  (os PDFs ficam em `content-private/`, fora do git — [ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)):
  tangente horizontal de `x³ − 3x + 4` em `x = ±1`, `f'(x) = 2x` para `x²` etc.
- Floats: compare com tolerância, nunca `==` em resultados calculados.

```rust
#[test]
fn derivative_of_x_squared_is_2x() -> Result<(), DomainError> {
    let slope = x_squared().derivative_at(3.0)?;
    assert!((slope - 6.0).abs() < TOLERANCE);
    Ok(())
}
```

## Casos de uso (`vx-application`)
- Substitua ports por **fakes** escritos à mão (sem bibliotecas de mock):
  `FakeParserReturningXSquared`, `FailingParser`.
- Teste o caminho feliz e cada tipo de erro.

## Adapters (`vx-infra`)
- Parser: tabela de casos `entrada → AST esperada` e `entrada inválida → posição do erro`.
- Considere *property-based testing* (`proptest`) para parser e simplificação:
  `parse(print(expr)) == expr`.

## Frontend
- `vitest` para `lib/domain` e `lib/application` (com `FakeCalculusEngine` e `FakeLinearAlgebraEngine`).
- Componentes: testar comportamento, não implementação.
- E2E (futuro): Playwright abrindo a versão web — digitar `x^2`, arrastar ponto, verificar inspetor.

## Comandos
```bash
cargo test --workspace --exclude vertice-desktop
pnpm -C apps/web test
```

## Regras
- Todo bug corrigido ganha um teste que o reproduz.
- Testes rápidos (suite de domínio < 1s), independentes e determinísticos.
- Nome do teste = comportamento esperado, em inglês, `snake_case` (Rust) ou frase (TS).
