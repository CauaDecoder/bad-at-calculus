# Layout e telas

## Workspace (tela principal)

```
┌──────────────┬──────────────────────────────────────────────┬──────────────────────┐
│ ◆ Math View  │  ⌘K  Buscar ou executar…            ☀/☾  ⤓   │  Inspetor            │
│              ├──────────────────────────────────────────────┤                      │
│ MÓDULOS      │                                              │  ┌ Função ─────────┐ │
│ ▸ Cálculo    │          │         ╲             ╱            │  │ f(x) = x^2      │ │
│   Vetores    │          │          ╲  ●       ╱             │  └─────────────────┘ │
│   Geometria  │          │     tangente╲     ╱               │  ┌ Ponto ──────────┐ │
│   Álg. linear│ ─────────┼───────────────╲─╱────────────     │  │ x₀  ━━━●━━━ -1.3│ │
│   Numérico   │          │               ╱╲                   │  │ f(x₀)     1.698 │ │
│   Física     │          │     f'(x)   ╱                     │  │ f'(x₀)   -2.606 │ │
│              │          │           ╱                        │  └─────────────────┘ │
│ EXPRESSÕES   │                                              │  ┌ Tangente ───────┐ │
│ ● f(x)=x²    │                                              │  │ y = -2.61x-1.70 │ │
│ ● f'(x)      │                              [－] [＋] [⌂]    │  └─────────────────┘ │
└──────────────┴──────────────────────────────────────────────┴──────────────────────┘
   232px                       flexível (canvas)                       320px
```

- **Sidebar (esquerda, 232px, recolhível):** navegação por módulos + lista de expressões
  (cada uma com bolinha de cor, visibilidade e menu).
- **Barra superior:** campo da command palette (estilo Mintlify), alternância de tema, exportar.
- **Canvas (centro):** gráfico com grade suave, eixos rotulados, controles de zoom flutuantes
  no canto inferior direito, ponto ativo arrastável.
- **Inspetor (direita, 320px, recolhível):** cards empilhados com dados do objeto selecionado,
  sliders e fórmulas em LaTeX.

## Command palette (`Ctrl/⌘ + K`)

```
┌────────────────────────────────────────────┐
│ 🔍  derivada                                │
├────────────────────────────────────────────┤
│ AÇÕES                                       │
│ ƒ′  Mostrar derivada de f          Ctrl D   │
│ ⟋   Mostrar reta tangente          Ctrl T   │
│ MÓDULOS                                     │
│ ∫   Cálculo › Derivação simbólica           │
│ DOCUMENTAÇÃO                                │
│ 📖  Regra da cadeia                          │
└────────────────────────────────────────────┘
```

## Barra de expressão (estilo v0)
Campo grande na parte inferior do canvas quando não há expressões: placeholder
"Digite uma função… ex: `sin(x)/x`", com preview LaTeX acima enquanto digita.

## Responsividade
| Largura     | Comportamento                                             |
| ----------- | --------------------------------------------------------- |
| ≥ 1280px    | Três colunas                                              |
| 960–1279px  | Inspetor vira drawer sobreposto                           |
| < 960px     | Sidebar vira menu; inspetor vira bottom sheet (tablets)    |

## Telas futuras
- **Vetores:** canvas com plano cartesiano, vetores como setas arrastáveis pela ponta; inspetor
  com componentes, módulo, ângulo e produto escalar.
- **Física — cinemática:** três gráficos empilhados sincronizados `s(t)`, `v(t)`, `a(t)` com um
  cursor de tempo compartilhado e botão de play.
