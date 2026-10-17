# Implementar o design aprovado

Leia este guia antes de implementar ou revisar uma tela. A decisão visual está no
[ADR 0013](../../03-arquitetura/adr/0013-direcao-de-design.md). Os tokens de produção estão em
[tokens.css](../../../apps/web/src/styles/tokens.css) e no
[design system](../../04-design/design-system.md).

## Usar a variante correta por tela

| Tela | Referência aprovada | Comportamento a preservar |
| --- | --- | --- |
| Início | A, `/prototype/inicio?variant=A` | "Para hoje", tarefa principal, motivo e ação por item, prova e primeiros passos |
| Exercício | D, `/prototype/exercicio?variant=D` | Enunciado e resposta à esquerda, laboratório da mesma função à direita, barra contínua de progresso no cabeçalho |
| Site | D, `/prototype/site?variant=D` | Layout claro da A, três botões de função no herói e movimento pelo scroll nas próprias seções |

A direção visual geral é **A, claro neutro**. A letra D descreve a composição aprovada de
Exercício e Site. Não significa tema escuro nem substitui o Início A. Os detalhes de mistura
estão na [escolha do mantenedor](../../references/design/README.md#escolha-do-mantenedor-2026-10-15).

## Recuperar a fonte do protótipo

O ADR 0013 cita o branch `prototype/vertice-design`, com código em
`apps/web/src/routes/prototype/` e `apps/web/src/lib/ui/prototype/`. Em 2026-10-15, a consulta
`git ls-remote --heads origin` retornou apenas `main` e `docs/vertice-system-design`.
O branch citado também não estava disponível no checkout. Essa consulta não comprova o que
ficou salvo no computador onde o protótipo foi criado.

No computador de origem, peça ao agente:

```text
Localize o protótipo aprovado da Vértice, preserve as alterações e informe branch, commit e caminhos.
```

Depois de recuperar a fonte, registre aqui a referência acessível e seu commit. Publique ou
transfira o branch somente com autorização do mantenedor. Evite trocar de branch num checkout
com alterações. Inspecione a fonte sem mudar de branch:

```bash
git show prototype/vertice-design:apps/web/src/routes/prototype/exercicio/+page.svelte
```

Para navegar pelo protótipo, use um checkout separado da referência recuperada. A instrução
antiga de `git switch` na pesquisa de design não deve descartar trabalho do checkout atual.
O protótipo serve para comparação. Reescreva a tela com os componentes e camadas de produção;
não incorpore seletor de variantes, mocks ou rotas `/prototype` ao build final.

Enquanto a fonte estiver ausente, use ADR 0013 e design system para implementar o que está
descrito. Marque a fidelidade ao protótipo como **não verificada**. Unidades de backend e CI
podem continuar. F0.5 e F1.7 só concluem a comparação visual quando a fonte for recuperada ou
o mantenedor aceitar explicitamente a comparação apenas documental.

## Aplicar tokens e estados reais

Use variáveis CSS existentes, Inter na UI, JetBrains Mono em entradas e números e KaTeX nas
fórmulas. Preserve tema claro e escuro, hierarquia e semântica de cores do design system.
Mudanças de token exigem atualização conjunta de código e documentação.

A sidebar usa a largura e o breakpoint do design system atual. "Minhas listas" e "Importar
lista" pertencem a Exercícios. "Biblioteca" identifica `/biblioteca`, disponível na F2.
O wireframe antigo em [layout e telas](../../04-design/layout-e-telas.md) não substitui os
valores atuais nem as variantes escolhidas.

Na F1, Início mostra continuação, listas e dados que já existem. Prova, FSRS e domínio por
habilidade entram quando a F2 implementar essas funções. Exemplos precisam estar identificados.
O checklist só registra uma ação como feita quando essa ação ocorreu.

Na F0, o Site D pode demonstrar o Laboratório existente. Identifique demonstrações e recursos
planejados. Não simule correção automática como funcionalidade já entregue antes da F1.

## Conferir antes de declarar a tela pronta

1. Compare a tela com a variante aprovada no mesmo tamanho de janela. Registre a referência
   e as diferenças necessárias, como os recursos ainda fora da fase.
2. Verifique desktop, viewport estreito e uso por teclado. Garanta foco visível, rótulos,
   mensagens de erro por campo e leitura do conteúdo sem depender de cor.
3. Teste tema claro e escuro. Confira contraste e legibilidade de fórmulas e gráficos.
4. Ative `prefers-reduced-motion`. O conteúdo deve permanecer completo e utilizável.
5. Teste os estados da tela: inicial, vazio, carregando, erro, sucesso e retomada quando houver.
   Confirme que dados, progresso e contadores vêm do estado real.
6. No Exercício D, verifique se função e ponto do laboratório correspondem ao exercício e se a
   barra progride por partes. No Site D, teste os três botões e o movimento por seção.

Guarde capturas de fixtures autorais no PR ou no artefato de verificação. Documente diferenças
e a plataforma efetivamente testada. Não publique capturas de material privado.
