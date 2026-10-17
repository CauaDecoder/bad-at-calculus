# Componentes

Componentes ficam em `apps/web/src/lib/ui/`. Regra: **recebem dados por props e emitem eventos**;
nunca chamam o motor matemático diretamente.

### Layout (`ui/layout/`)
| Componente    | Descrição                                                                        |
| ------------- | -------------------------------------------------------------------------------- |
| `AppShell`    | Sidebar de 248px + página; abaixo de 900px, menu expansível por teclado; link para pular ao conteúdo |
| `ModuleNav`   | Módulos e tópicos (de `ui/copy/navigation.ts`), página atual destacada           |
| `LegacyRedirect` | Entrada estática das URLs antigas; substitui a entrada no histórico e preserva query string e fragmento |
| `TopicPage`   | Título do tópico + descrição + corpo com altura total                            |
| `Workspace`   | Gráfico dominante + coluna de inspetor (tópicos visuais)                         |
| `SplitPanel`  | Dados à esquerda, resolução à direita (matrizes, sistemas)                       |

### Gráficos
| Componente    | Descrição                                                                        |
| ------------- | -------------------------------------------------------------------------------- |
| `GraphCanvas` | 2D: camadas (`curve`, `line`, `point`, `area`, `riemann`…), pan, zoom, ponto arrastável opcional (`role="slider"`) |
| `SpaceCanvas` | 3D ortográfico: setas, pontos, planos; girar, zoom, vista "Plano xy"             |

### Entradas
| Componente         | Descrição                                                                   |
| ------------------ | --------------------------------------------------------------------------- |
| `ExpressionInput`  | `f(x) =` monoespaçado, erro de sintaxe inline em PT-BR                      |
| `NumberField`      | Número digitado como matemática (`1/2`, `sqrt(3)`), destaque de erro        |
| `VectorInput`      | `u = ( , , )` com três `NumberField`s                                        |
| `MatrixInput`      | Grade de entradas + `SizeStepper`s de linhas e colunas                      |
| `SegmentedControl` | Uma opção entre poucas (tipo de limite, método de Riemann)                  |
| `SizeStepper`      | `− n +` para inteiros pequenos                                              |
| `Switch`           | Liga/desliga acessível (`role="switch"`)                                    |
| `ExampleChips`     | Exemplos clicáveis que preenchem os campos                                  |

### Resultados
| Componente      | Descrição                                                                     |
| --------------- | ----------------------------------------------------------------------------- |
| `MathFormula`   | LaTeX com KaTeX (herda a cor do contexto)                                     |
| `WorkedResult`  | Fórmula geral (discreta) + linhas da resolução com os números + resposta      |
| `StepList`      | Passos numerados: título, regra/operação e resultado                          |
| `Verdict`       | Sim/não com ícone (ex.: "u e v são ortogonais")                               |
| `ValueRow`      | "rótulo · valor" com números tabulares                                        |
| `ErrorNote`     | Erro ou aviso em linha                                                        |
| `Card`, `Icon`, `IconButton` | Superfície, ícones (Lucide, ISC), botão só com ícone          |

### Planejados
`ExpressionList` (várias funções), `CommandPalette`, `Tabs`, `Tooltip` rico.

## Contrato de um componente

```svelte
<script lang="ts">
  interface Props {
    label: string;            // obrigatório e descritivo
    value: number;
    onchange: (value: number) => void;   // eventos como callbacks
  }
  let { label, value, onchange }: Props = $props();
</script>
```

- Props tipadas com `interface Props`.
- Estilos com escopo no próprio componente usando **apenas tokens**.
- Estados: padrão, hover, foco visível, desabilitado, erro — todos desenhados.
- Um componente = um arquivo = uma responsabilidade. Passou de ~150 linhas? Divida.

## Botões
| Variante    | Aparência                                   | Uso                            |
| ----------- | ------------------------------------------- | ------------------------------ |
| `primary`   | fundo acento, texto branco                  | Uma ação principal por área    |
| `secondary` | superfície + borda                          | Ações comuns                   |
| `ghost`     | sem fundo, hover `surface-hover`            | Toolbars, ícones               |
| `danger`    | texto/borda `danger`                        | Remover                        |
Altura 32px (compacto) ou 36px; raio `--radius-md`; padding horizontal 12px.
