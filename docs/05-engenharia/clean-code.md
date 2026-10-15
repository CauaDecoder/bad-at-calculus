# Clean Code no Math View

Resumo prático das regras de *Clean Code* (Robert C. Martin) aplicadas ao projeto.
Fontes: [resumo de wojteklu](https://gist.github.com/wojteklu/73c6914cc446146b8b533c0988cf8d29) e
[RD Shipit — Clean Code](https://medium.com/rd-shipit/clean-code-23580b4e556c).

Objetivo: código que **um humano ou um agente de IA entende lendo uma vez**.

## Regras gerais
- Siga as convenções da linguagem (`rustfmt`, `clippy`, Prettier, ESLint).
- **KISS:** a solução mais simples que funciona.
- **Regra do escoteiro:** deixe o código um pouco melhor do que encontrou.
- Encontre a causa raiz de um bug antes de corrigir.

## Nomes
- Descritivos e sem ambiguidade; pronunciáveis e buscáveis.
- Sem números mágicos: nomeie a constante.
- Sem prefixos de tipo (`strName`, `IParser`) nem abreviações obscuras.
- Termos do domínio matemático em inglês correto: `slope`, `derivative`, `tangent_line`, `magnitude`.

```rust
// ❌
let d = (f(x + 0.00001) - f(x - 0.00001)) / 0.00002;
// ✅
const DERIVATIVE_STEP: f64 = 1e-5;
let slope = (forward - backward) / (2.0 * DERIVATIVE_STEP);
```

## Funções
- **Pequenas** e fazem **uma coisa só** (clippy limita a 60 linhas; mire em < 20).
- Poucos argumentos (≤ 3). Mais que isso → agrupe em um struct (`TangentLineRequest`).
- **Sem argumentos booleanos** (flags): crie duas funções.
- Sem efeitos colaterais escondidos: função que calcula não salva, não loga, não altera estado global.
- Ordene de cima para baixo: quem chama vem antes de quem é chamado.

```ts
// ❌ flag argument
plot(fn, true);
// ✅ intenção explícita
plotWithDerivative(fn);
```

## Comentários
- Explique-se no código primeiro; comente o **porquê**, nunca o **o quê**.
- Doc comments (`///`, `/** */`) em todo item público: o que faz, e por quê existe se não for óbvio.
- Sem código comentado — o git lembra por você.
- Avisos de consequência são bem-vindos (`// f64 comparison: always use TOLERANCE`).

## Estrutura do código
- Conceitos separados verticalmente por linha em branco; código relacionado próximo.
- Variáveis declaradas perto do uso.
- Linhas ≤ 100 colunas.

## Objetos e estruturas de dados
- Esconda a estrutura interna (campos privados + métodos) quando houver invariantes
  (`RealFunction`); use structs de dados simples quando não houver (`Point2`).
- Prefira **value objects** a primitivos soltos: `Point2` em vez de `(f64, f64)`.
- **Lei de Deméter:** fale só com vizinhos diretos (`a.b().c().d()` é um cheiro).
- Prefira polimorfismo (traits, enums com `match`) a cadeias de `if/else`.

## Tratamento de erros
- Rust: `Result<T, E>` com erros tipados. Sem `unwrap`/`expect`/`panic!` fora de testes
  (clippy bloqueia).
- TS: erros do motor chegam como exceções nos adapters e são convertidos em estado
  (`errorMessage`) na camada de aplicação — a UI nunca recebe exceção crua.

## Testes
- Um conceito por teste, nome que descreve o comportamento:
  `derivative_of_x_squared_is_2x`, `rejects_division_by_zero`.
- Rápidos, independentes, repetíveis. Detalhes em [testes.md](testes.md).

## Code smells a evitar
| Smell            | Sintoma no projeto                                              |
| ---------------- | --------------------------------------------------------------- |
| Rigidez          | Mudar o parser exige mexer na UI                                 |
| Fragilidade      | Ajustar `DERIVATIVE_STEP` quebra testes de outro módulo           |
| Imobilidade      | Não dá para usar o cálculo de tangente fora do Tauri              |
| Complexidade desnecessária | Generics/traits sem segunda implementação à vista      |
| Repetição        | Mesma fórmula em Rust e em TS — o TS deve só exibir               |
| Opacidade        | Funções com nomes como `calc`, `process`, `handle`                |
