# ADR 0013 — Direção de design: claro neutro, telas A/D e movimento que explica

- **Status:** aceito
- **Data:** 2026-10-15

## Contexto
Antes das fases de código, a Vértice precisava de um visual principal para o app e para o site.
Partimos das referências do mantenedor (capturas de Remote, hilos, Replit, MagicPath, Tines,
Quicken e Shopify; sites premiados no Awwwards; componentes animados do uiarc e do metalforge)
e dos princípios de *revenue-centric design*. Foram prototipadas três telas (Início, Exercício,
Site), com três variantes radicalmente diferentes cada, testando também três direções visuais:
- **A** — claro neutro (índigo sobre zinco);
- **B** — papel quente (terracota + serifado);
- **C** — escuro expressivo (laranja sobre quase preto).

Detalhes, referências e critérios: [direção de design](../../references/design/README.md).

## Decisão
1. **Direção visual A — claro neutro** para app e site: fundo `#F7F7F8`, superfície branca, borda
   de 1px `#E7E7EA`, texto `#18181B`/`#6B6B76`, **um único acento índigo `#4F46E5`**. Tokens em
   [`tokens.css`](../../../apps/web/src/styles/tokens.css), documentados em
   [`design-system.md`](../../04-design/design-system.md). O tema escuro continua disponível.
2. **Telas escolhidas:**
   - **Início = variante A ("Para hoje").** Sidebar agrupada, uma tarefa principal no topo
     ("Continuar item 8"), lista "Para hoje" com motivo e ação, coluna com a prova (contagem
     regressiva) e "Primeiros passos" começando em 1 de 4.
   - **Exercício = variante D.** Laboratório dividido da A (enunciado e resposta à esquerda,
     gráfico vivo da mesma função à direita) com a **barra de progresso contínua da B** no
     cabeçalho, que cresce a cada parte acertada.
   - **Site = variante D.** Layout, texto e paleta da A (dor → prova → recursos → FAQ → CTA).
     No herói, **três botões de função da B** trocam o gráfico, a pergunta e a derivada. Da C
     ficou a **ideia** de que as coisas se mexem com o scroll, mas aplicada às peças da própria A,
     sem cena separada:
     - no recurso Laboratório, a tangente percorre a curva conforme a página rola;
     - no recurso Exercícios, as respostas se digitam sozinhas e recebem o veredito do motor;
     - no recurso Revisão, os cartões entram um a um, ordenados pela data de revisão.

     Textos, cartões e campos surgem aos poucos, em fade e em sequência, conforme a página rola
     (e apagam ao voltar). O demo do herói tem um leve *parallax*.
3. **Movimento explica, não decora.** Números rolam, gráficos se transformam (*morph*) em vez de
   redesenhar, o botão Verificar muda de estado (verificando → correto → errado), e a celebração
   fica no acerto. Tudo é interrompível e respeita `prefers-reduced-motion`. Os tokens de duração e
   easing foram ampliados para isso.
4. **Princípios de produto aplicados às telas** (da RCD):
   - nenhuma tela vazia, sempre com exemplo e uma ação primária;
   - número sempre com frase de direção;
   - CTA com microcopy e *click trigger* ("Grátis · sem cadastro · funciona offline");
   - herói do site com a dor primeiro e o produto funcionando como prova.

## Alternativas consideradas
- **B — papel quente** — acolhedor e diferente, mas o serifado e os pastéis disputam atenção com os
  gráficos, que são o protagonista; exigiria fonte serifada a mais no bundle.
- **C — escuro expressivo** — impacto "Awwwards" no site, mas cansativo para sessões longas de
  estudo e com contraste mais difícil nas cores de gráfico. A primeira versão da Site D copiava a
  cena de pontos da C; foi descartada porque o layout da A comunica melhor. Ficou só a ideia de
  movimento ligado ao scroll.
- **Manter o visual do Math View sem revisão** — já era próximo de A, mas sem a hierarquia de
  "o que fazer agora", a tarefa principal e as regras de movimento.

## Consequências
- ✅ Visual coerente com o que já existe no Laboratório: migração barata, mesmos componentes.
- ✅ As telas escolhidas viram a especificação visual da F1 (Início e Exercício) e do site (F0).
- ⚠️ Os protótipos são descartáveis e **não** foram promovidos a código de produção. Estão no
  branch `prototype/vertice-design` como fonte primária. As telas reais serão reescritas com os
  componentes, tokens e testes do app.
- ⚠️ A direção A ainda precisa do teste de 5 segundos do herói com estudantes dos três cursos.
- 🔭 Se o site precisar de mais personalidade, o movimento da C pode crescer sem trocar a paleta.
