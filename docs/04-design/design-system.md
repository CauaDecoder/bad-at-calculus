# Design system

Fonte única da verdade no código: [`apps/web/src/styles/tokens.css`](../../apps/web/src/styles/tokens.css).
**Mudou um token? Atualize os dois arquivos juntos.** Componentes usam variáveis CSS, nunca hex direto.

Estética: neutra e calma (zinc), superfícies claras com borda fina, raios generosos, sombras
quase imperceptíveis e **um único acento** (índigo) — inspirada em dashboards SaaS modernos,
Mintlify e v0 (ver [referências](referencias.md)).

## Cores

### Interface
| Token                  | Claro     | Escuro    | Uso                                  |
| ---------------------- | --------- | --------- | ------------------------------------ |
| `--color-bg`           | `#fafafa` | `#09090b` | Fundo da janela                      |
| `--color-surface`      | `#ffffff` | `#18181b` | Cards, painéis                       |
| `--color-surface-hover`| `#f4f4f5` | `#27272a` | Hover de itens                       |
| `--color-border`       | `#e4e4e7` | `#27272a` | Bordas 1px, divisores                |
| `--color-text`         | `#18181b` | `#fafafa` | Texto principal                      |
| `--color-text-muted`   | `#71717a` | `#a1a1aa` | Rótulos, legendas                    |
| `--color-accent`       | `#4f46e5` | `#818cf8` | Ação primária, foco, seleção         |
| `--color-accent-soft`  | `#eef2ff` | `#1e1b4b` | Fundo de item selecionado            |
| `--color-success` / `warning` / `danger` | verde / âmbar / vermelho | — | Estados        |

### Gráficos (semânticas)
| Token                      | Claro     | Escuro    | Significado                        |
| -------------------------- | --------- | --------- | ---------------------------------- |
| `--color-plot-function`    | `#18181b` | `#fafafa` | Função principal `f(x)`            |
| `--color-plot-tangent`     | `#16a34a` | `#4ade80` | Reta tangente / secante            |
| `--color-plot-derivative`  | `#e11d48` | `#fb7185` | Derivada `f'(x)`                   |
| `--color-plot-point`       | `#4f46e5` | `#818cf8` | Ponto ativo / arrastável           |
| `--color-plot-series-4/5`  | ciano / âmbar | —     | Funções extras                     |
| `--color-plot-grid` / `axis` | cinzas suaves | —   | Grade e eixos                      |

As cores de tangente (verde) e derivada (vermelho) seguem a convenção das figuras que inspiraram o
projeto. Para daltonismo, a derivada também é desenhada **tracejada** e todas as curvas têm rótulo.

## Tipografia
- **UI:** Inter — `--font-sans`.
- **Expressões e números:** JetBrains Mono — `--font-mono` (números tabulares alinhados).
- **Matemática renderizada:** KaTeX.

| Token        | Tamanho | Uso                          |
| ------------ | ------- | ---------------------------- |
| `--text-xs`  | 12px    | Rótulos de eixo, legendas    |
| `--text-sm`  | 14px    | Padrão da interface          |
| `--text-md`  | 16px    | Entradas de expressão        |
| `--text-lg`  | 20px    | Títulos de painel            |
| `--text-xl`  | 24px    | Títulos de página            |

Pesos: 400 (texto), 500 (rótulos interativos), 600 (títulos). Evitar 700+.

## Espaçamento — grade de 4px
`--space-1` 4 · `--space-2` 8 · `--space-3` 12 · `--space-4` 16 · `--space-6` 24 · `--space-8` 32 · `--space-12` 48.
Padding interno de card: 16px. Gap entre cards: 12–16px. Margem do canvas: 16px.

## Raios, bordas e elevação
- Raios: `--radius-sm` 6px (chips, botões pequenos), `--radius-md` 8px (inputs, botões),
  `--radius-lg` 12px (cards, canvas), `--radius-full` (pílulas, avatares).
- Bordas: 1px `--color-border` em tudo que é superfície — a borda faz o trabalho, não a sombra.
- Sombras: `--shadow-sm` (cards), `--shadow-md` (popovers), `--shadow-lg` (command palette, modais).

## Movimento
- `--duration-fast` 120ms (hover, foco), `--duration-normal` 200ms (painéis, popovers).
- `--easing-standard`: `cubic-bezier(0.2, 0, 0, 1)`.
- Animações matemáticas (secante → tangente) podem ser mais longas, mas sempre interrompíveis.
- `prefers-reduced-motion`: desliga transições.

## Tema
Claro por padrão. O app define `data-theme="dark"` no `<html>` conforme a preferência do sistema
ou a escolha do usuário (salva localmente).

## Iconografia
[Lucide](https://lucide.dev) — traço 1.5px, 16/20px. Ícone sempre acompanhado de `aria-label`
ou texto.
