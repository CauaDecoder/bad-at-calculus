# Design system

Fonte única da verdade no código: [`apps/web/src/styles/tokens.css`](../../apps/web/src/styles/tokens.css).
**Mudou um token? Atualize os dois arquivos juntos.** Componentes usam variáveis CSS, nunca hex direto.

**Direção: A — claro neutro** ([ADR 0013](../03-arquitetura/adr/0013-direcao-de-design.md)).
Neutra e calma (zinco), superfícies claras com borda fina, raios generosos, sombras quase
imperceptíveis e **um único acento** (índigo). Inspirada em Remote, Shopify, Tines, Mintlify e v0
(ver [referências](referencias.md) e a [direção de design](../references/design/README.md)).

## Cores

### Interface
| Token                   | Claro     | Escuro    | Uso                                       |
| ----------------------- | --------- | --------- | ----------------------------------------- |
| `--color-bg`            | `#f7f7f8` | `#09090b` | Fundo da janela                           |
| `--color-surface`       | `#ffffff` | `#18181b` | Cards, painéis, sidebar                   |
| `--color-surface-hover` | `#f4f4f5` | `#27272a` | Hover de itens                            |
| `--color-border`        | `#e7e7ea` | `#27272a` | Bordas 1px, divisores                     |
| `--color-text`          | `#18181b` | `#fafafa` | Texto principal                           |
| `--color-text-muted`    | `#6b6b76` | `#a1a1aa` | Rótulos, legendas, motivo das tarefas     |
| `--color-accent`        | `#4f46e5` | `#818cf8` | **A** ação primária, foco, seleção, progresso |
| `--color-accent-hover`  | `#4338ca` | `#a5b4fc` | Hover/pressionado da ação primária        |
| `--color-accent-soft`   | `#eef2ff` | `#1e1b4b` | Item selecionado, pílulas, tags           |
| `--color-on-accent`     | `#ffffff` | `#09090b` | Texto sobre o acento                      |
| `--color-track`         | `#e4e4e7` | `#27272a` | Trilho de barras de progresso             |

### Estados
| Token | Claro | Suave (fundo) | Uso |
| ----- | ----- | ------------- | --- |
| `--color-success` / `-soft` | `#16a34a` | `#f0fdf4` | Resposta correta, item concluído |
| `--color-warning` / `-soft` | `#d97706` | `#fff7ed` | Erro comum, atenção |
| `--color-danger` / `-soft`  | `#dc2626` | `#fef2f2` | Resposta errada, ação destrutiva |
| `--color-info` / `-soft`    | `#0e7490` | `#ecfeff` | Revisão, informação neutra |

Contraste (WCAG): `text-muted` sobre `bg` 4,9:1; `accent` sobre `accent-soft` 5,6:1; branco sobre
`accent` 6,3:1; `info` sobre `info-soft` 5,2:1. Todos ≥ AA para texto normal.

Regras de uso:
- **Um acento por tela.** Só a ação primária usa o fundo `--color-accent`; as demais ações são
  botões com borda.
- Cores de estado aparecem com texto ou ícone, nunca sozinhas.
- Ícones de categoria (lista, revisão, erro, laboratório) usam o par cor + `-soft`.

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
- **Expressões, números e respostas:** JetBrains Mono — `--font-mono` (números tabulares alinhados).
- **Matemática renderizada:** KaTeX.

| Token            | Tamanho   | Uso                                              |
| ---------------- | --------- | ------------------------------------------------ |
| `--text-xs`      | 12px      | Rótulos de eixo, legendas, microcopy sob o CTA   |
| `--text-sm`      | 14px      | Padrão da interface                              |
| `--text-md`      | 16px      | Entradas de expressão, corpo                     |
| `--text-lg`      | 20px      | Títulos de painel, enunciado do exercício        |
| `--text-xl`      | 24px      | Títulos de seção                                 |
| `--text-2xl`     | 26px      | Título de página ("Boa noite, …")                |
| `--text-kpi`     | 56px      | Um número herói por tela (dias até a prova)      |
| `--text-display` | 36–58px   | Herói do site (`clamp`)                          |

Pesos: 400 (texto), 500 (rótulos interativos), 600 (títulos). Evitar 700+. Títulos grandes com
`letter-spacing` negativo (−0,02 a −0,035em).

## Espaçamento — grade de 4px
`--space-1` 4 · `--space-2` 8 · `--space-3` 12 · `--space-4` 16 · `--space-6` 24 · `--space-8` 32 · `--space-12` 48.
Padding interno de card: 20px (16px em telas pequenas). Gap entre cards: 16–20px.
Conteúdo do app: até 1120px; página com 32–40px de margem lateral (16px no celular).

## Raios, bordas e elevação
- Raios: `--radius-sm` 6px (chips, tags), `--radius-md` 8px (inputs, botões, itens da sidebar),
  `--radius-lg` 12px (canvas), `--radius-xl` 14px (**cards e painéis**), `--radius-2xl` 18px
  (superfícies grandes: demo do site, omnibox), `--radius-full` (pílulas, avatares, botão Verificar).
- Bordas: 1px `--color-border` em tudo que é superfície; a borda faz o trabalho, não a sombra. A
  tarefa principal ganha borda tingida com o acento.
- Sombras: `--shadow-sm` (cards), `--shadow-md` (popovers, morph select aberto), `--shadow-lg`
  (command palette, modais), `--shadow-xl` (demo do herói do site), `--shadow-accent` (CTA do site).

## Layout do app
- **Sidebar** de `--layout-sidebar-width` (248px): marca, busca (`Ctrl K`), grupos "Estudar"
  (Início, Laboratório, Exercícios, Revisão com contador, Plano de estudos) e "Biblioteca" (Minhas
  listas, Importar lista); usuário no rodapé. Some abaixo de 900px.
- **Início:** título + frase de direção; **uma tarefa principal** com barra de progresso e um
  único botão índigo; lista "Para hoje" (ícone, título, motivo, prazo, ação); coluna direita com
  a prova e os primeiros passos.
- **Exercício:** cabeçalho com ✕, nome da lista, **barra de progresso contínua** e `n/total`;
  coluna esquerda (enunciado, partes, resposta em mono, Verificar + Dica, microcopy) e coluna
  direita com o gráfico vivo da mesma função.
- Telas de referência: protótipos no branch `prototype/vertice-design`
  ([direção de design](../references/design/README.md#6-protótipos)).

## Movimento
| Token | Valor | Uso |
| ----- | ----- | --- |
| `--duration-fast` | 120ms | Hover, foco |
| `--duration-normal` | 200ms | Popovers, troca de aba |
| `--duration-slow` | 320ms | Mudança de estado (Verificar), painéis |
| `--duration-draw` | 800ms | Desenho de gráfico, celebração, *reveal* no scroll |
| `--easing-standard` | `cubic-bezier(0.2, 0, 0, 1)` | Padrão |
| `--easing-emphasized` | `cubic-bezier(0.2, 0.9, 0.1, 1)` | Entrar, crescer, barras de progresso |
| `--easing-morph` | `cubic-bezier(0.65, 0, 0.35, 1)` | Forma → forma (pontos, arcos) |

Regras ([ADR 0013](../03-arquitetura/adr/0013-direcao-de-design.md)):
- **Movimento explica, não decora**: só anima mudança de estado.
- Números que mudam **rolam**; gráficos **se transformam** (não redesenham); o botão Verificar
  passa por verificando → correto/errado com o check desenhado.
- Celebração só no acerto (pico) e no fim da lista.
- **No site, o movimento acompanha o scroll** e mostra o produto funcionando dentro das próprias
  seções: a tangente percorre a curva, a resposta se digita e é corrigida, os cartões de revisão
  entram em ordem. Nada de cena decorativa separada, e o texto continua legível parado.
- Animações matemáticas podem ser mais longas, mas sempre interrompíveis.
- `prefers-reduced-motion`: os tokens de duração vão a 0 e *morphs* viram troca simples.

## Tema
Claro por padrão. O app define `data-theme="dark"` no `<html>` conforme a preferência do sistema
ou a escolha do usuário (salva localmente).

## Iconografia
[Lucide](https://lucide.dev) — traço 1.5px, 16/20px. Ícone sempre acompanhado de `aria-label`
ou texto.
