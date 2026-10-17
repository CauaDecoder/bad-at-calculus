# Direção de design — Vértice

- **Status:** direção aprovada no [ADR 0013](../../03-arquitetura/adr/0013-direcao-de-design.md).
  A fonte navegável do protótipo precisa ser recuperada (ver [Protótipos](#6-protótipos)).
- **Data:** 2026-10-14
- **Objetivo:** definir o visual principal da plataforma (app e site) antes das fases de código,
  a partir das referências do mantenedor e da skill *revenue-centric-design* (RCD).

A implementação segue o [design system](../../04-design/design-system.md) e os tokens de
`apps/web/src/styles/tokens.css`. O [guia para agentes](../../05-engenharia/implementacao/design-aprovado.md)
define como conferir as variantes aprovadas e registrar a comparação visual.

---

## 1. Referências de app (capturas)

As imagens ficam em `capturas/`, que o git ignora, porque são telas de produtos comerciais
(curadoria Mobbin). A descrição abaixo é o que importa para o projeto.

| Arquivo | Produto | O que levar para a Vértice |
| ------- | ------- | -------------------------- |
| `home-ref-1.png` | Remote — dashboard | Saudação curta + lista **"Coisas para fazer"** com ícone, título, motivo e prazo. Coluna lateral com resumos. Sidebar agrupada por seção. → Tela **Início**: "Para hoje". |
| `overview-ref-1.png` | hilos — home do workspace | Data em versalete + título serifado grande; **4 blocos de KPI em tons pastel** ("Unread 4 — Across your rooms"), cada número com frase de contexto. Cards de documentos e conversas em fundos tintados. Caixa "Ask anything" flutuante. Ilustração com personalidade. |
| `chatbot-ref-1.png` | Replit — home | **Prompt central** ("what do you want to make?") com chips de categoria e "try an example prompt". Banner de novidade no topo; recentes embaixo. → Omnibox: "digite uma função ou cole um exercício". |
| `workspace-ref-1.png` | MagicPath — canvas vazio | Empty state com **4 cartões de caminho** (conectar, importar, começar de um sistema). Painel de threads à esquerda com sugestões. → Empty state de Autoria/Importar. |
| `boards-ref-1.png` | MagicPath — meus arquivos | Grade de cartões com **miniatura real** do conteúdo + "editado há 2 s"; cartão tracejado "Criar novo". Abas Todos/Arquivos/Bibliotecas. → Listas e packs. |
| `colors-ref-1.png` | Tines — recentes | Lista densa com ícone, título, meta ("3 ações · editado há 14 min") e **tags coloridas**. Aviso de limite inline. → Banco de exercícios com tags de habilidade/dificuldade. |
| `data-picker-ref-1.png` | Quicken — seletor de datas | Atalhos ("Próximos 30 dias", "Este mês") + calendário duplo + **"padrão para exibir"**. Calendário com pontos coloridos por dia. → Data da prova e calendário de revisões. |
| `fields-ref-1.png` | Shopify — adicionar produto | Formulário em cartões + **Setup guide "1 / 11 completed"** com passos expansíveis e "adicionar exemplo". → Editor de exercício + checklist de primeiros passos. |

Padrão comum: **app shell claro**, sidebar fina, conteúdo em cartões com borda de 1px, uma única
ação primária e **direção explícita** (o que fazer agora), não só dados.

## 2. Referências de site (Awwwards)

| Site | Paleta | O que levar |
| ---- | ------ | ----------- |
| [Heron AI](https://www.awwwards.com/sites/heron-ai) — SOTD 12/10/2026 | `#F15534` · `#282828` | Hero com interação em Canvas, *preloader*, narrativa por scroll (GSAP). |
| [Exebenus](https://www.awwwards.com/sites/exebenus) — Refokus | `#FF671C` · `#0E0F11` (+ `#F57327`, `#404040`) | Animação 3D/2D guiada pelo scroll; visual limpo e técnico ("drilling intelligence"). |
| [Cula](https://www.awwwards.com/sites/cula) — SOTD | `#E3E8EF` | "Mundo 3D" que explica como o dado é coletado; *information card* e *data card* flutuando sobre a cena. |
| [Arqitel](https://www.awwwards.com/sites/arqitel) — Refokus | `#0500FF` · `#060A29` | **Transição de pontos que se transformam** (*dot morphing*) e gráfico de barras animado; dados contando a história. Fala com o público de AU. |
| [Time to Refokus](https://www.awwwards.com/sites/time-to-refokus) | `#211339` · `#000000` | Busca assistida por IA (chatbot que fala) e navegação 3D de artigos. |

Leitura: os sites premiados contam **uma história com o próprio dado** (o gráfico é o herói), usam
um acento quente sobre fundo quase preto e animam em resposta ao scroll. Para a Vértice, o
equivalente natural é **o gráfico matemático vivo**: a tangente percorrendo a curva, a área de
Riemann se preenchendo, os vetores girando.

## 3. Referências de animação

| Fonte | Componente | Comportamento | Onde usar |
| ----- | ---------- | ------------- | --------- |
| [uiarc — line chart](https://uiarc.dev/components/line-chart) | Linha multi-série | Crosshair deslizante, legenda que liga/desliga séries, **caminhos que se transformam** ao trocar o intervalo (7/30/90 dias), série anterior tracejada, tabela para leitor de tela | Progresso: acertos por dia, este período × anterior |
| [uiarc — donut chart](https://uiarc.dev/components/donut-chart) | Rosca | Arcos que se transformam entre conjuntos; valor ativo **rola** para o centro | Domínio por tópico |
| [uiarc — number field](https://uiarc.dev/components/number-field) | Campo numérico | Steppers, setas do teclado, *scrub* arrastando o rótulo, dígitos que rolam | Parâmetros do Laboratório (`x₀`, `n` de Riemann) |
| [uiarc — morph select](https://uiarc.dev/components/morph-select) | Select | **O gatilho cresce até virar a lista**, destaque deslizante, *type-ahead* | Escolher disciplina, lista, tipo de resposta |
| [metalforge — interactions (pay)](https://metalforge.xyz/interactions#component=pay) | Botão de pagar | Botão que muda de estado em sequência (ocioso → processando → concluído) com micro-animação. Página em WebGL, não foi possível inspecionar o código | Botão **Verificar** da resposta |
| metalforge — carrier, worlds | — | Páginas retornaram 404 em 2026-10-14 | — |

Regras de movimento:
- **Movimento explica, não decora.** Toda animação mostra uma mudança de estado (resposta
  verificada, intervalo trocado, ponto arrastado).
- Duração: 120 ms (hover), 200–320 ms (estado), até 800 ms (celebração e desenho de gráfico).
  Easing padrão `cubic-bezier(0.2, 0, 0, 1)`; transformações (*morph*) com mola suave.
- **Interrompível** e respeitando `prefers-reduced-motion` (sem *morph*, só troca de opacidade).
- Números que mudam **rolam** (dígito por dígito); gráficos **se transformam**, não redesenham.

## 4. Revenue-Centric Design aplicado a um projeto open source

A Vértice não cobra. A "receita" de um projeto open source de estudo é **estudante ativo que
volta** e **contribuição**. As métricas da RCD viram:

| RCD | Na Vértice |
| --- | ---------- |
| Ativação / aha moment | **Primeira resposta correta verificada** (ou primeira tangente arrastada), medida em tempo desde a abertura. Meta: < 2 min |
| TTV (time-to-value) | Abrir o app → ver uma lista própria resolvível. Meta: < 5 min com pack de exemplo |
| Retenção D7 | Voltou para uma revisão em 7 dias |
| Expansão | Importou a própria lista; contribuiu com exercício; convidou a turma (F3+) |
| "Preço é filtro" | Não se aplica a preço. O filtro é o **ICP**: estudante de exatas/ADS/AU com prova marcada |

Métricas medidas **localmente** (sem telemetria por padrão, ADR 0010).

Princípios usados nos protótipos (com o mecanismo nomeado):

| Princípio RCD | Decisão de design |
| ------------- | ----------------- |
| *Never ship a blank dashboard* (empty state + sample data) | Primeiro acesso já mostra uma **lista de exemplo** em andamento e uma única ação: "Resolver o primeiro exercício" |
| *Progress effect / started-progress* | Checklist de primeiros passos começa em **1 de 4** (o app já fez a primeira: "Laboratório aberto") |
| *Zeigarnik effect* | Lista mostra "7 de 19" e o próximo item pela metade; revisões vencidas como laços abertos |
| *Declarative friction* | Uma pergunta de intenção no início: **"Qual é a sua próxima prova e quando?"** O plano de estudos e a revisão se ramificam a partir dela |
| *Default effect* | Disciplina e data já sugeridas (detectadas do pack importado); revisão FSRS ligada por padrão |
| *Actionable dashboard* / "informação sem contexto é spam" | Cada número vem com frase de direção: "Você errou a regra da cadeia 4× — revisar 3 itens" |
| F-pattern, KPI no canto superior esquerdo | O KPI principal é **dias até a prova + itens prontos**, no topo à esquerda |
| *Peak-end rule* / celebrar a ativação | Na primeira resposta correta: gráfico de domínio **subindo** e o check animado do botão Verificar |
| CTA com microcopy e *click trigger* | "Verificar resposta" + "Enter · erro de sintaxe não conta tentativa"; no site, "Abrir o laboratório" + "grátis, sem cadastro, funciona offline" |
| 5-second test, problema primeiro | Site abre com a dor: "O gabarito no fim do PDF diz que você errou. Não diz onde." |
| *Your promise is the size of your proof* | No site, a prova é o produto funcionando no herói (gráfico vivo e correção ao vivo), não ilustração |
| *No clicks out* | Site com um único link externo secundário (GitHub) |

## 5. Três direções visuais (uma por variante)

As variantes dos protótipos também testam paleta. Todas usam Inter (UI) + JetBrains Mono (números
e expressões); a B testa um serifado de títulos.

| | A — Claro neutro | B — Papel quente | C — Escuro expressivo |
| - | ---------------- | ---------------- | --------------------- |
| Inspiração | Remote, Shopify, Tines | hilos, Replit | Exebenus, Heron AI, Arqitel |
| Fundo / superfície | `#F7F7F8` / `#FFFFFF` | `#F6F4EF` / `#FFFDF9` | `#0E0F11` / `#16181C` |
| Texto / suave | `#18181B` / `#6B6B76` | `#1F1B16` / `#7A7268` | `#F2F2F3` / `#8D8F98` |
| Acento | Índigo `#4F46E5` | Terracota `#D9541E` | Laranja `#FF671C` |
| Apoio | Verde `#16A34A`, âmbar `#D97706` | Pastéis: salmão `#F6B9A3`, mostarda `#F3D98B`, rosa `#F1C6DA`, azul `#B9D4F1` | Azul elétrico `#3D5AFE`, verde `#3DDC97` |
| Títulos | Inter 600 | Serifado (protótipo: pilha do sistema; produção: Fraunces ou Instrument Serif via fontsource) | Inter 600, tamanhos grandes, tracking negativo |
| Tom | Ferramenta calma e confiável | Caderno pessoal, acolhedor | Laboratório técnico, "premiado" |

## 6. Protótipos

O ADR 0013 registra a fonte descartável no branch **`prototype/vertice-design`**, fora do `main`,
em `apps/web/src/routes/prototype/` e `apps/web/src/lib/ui/prototype/`. Na consulta ao remoto em
2026-10-15, esse branch não estava publicado. Recupere a fonte no computador de origem conforme
o [guia de design](../../05-engenharia/implementacao/design-aprovado.md#recuperar-a-fonte-do-protótipo).

Depois da recuperação, abra a referência em checkout separado, rode `pnpm dev` e acesse
`http://localhost:5173/prototype`. Troque de variante com a barra inferior ou `←` / `→`.
A barra só existe em modo de desenvolvimento. Não troque de branch descartando alterações locais.

| Tela | Rota | A | B | C |
| ---- | ---- | - | - | - |
| Início (app) | `/prototype/inicio` | **Para hoje**: sidebar + lista de tarefas + prova e checklist | **Prompt primeiro**: saudação serifada, omnibox, blocos pastel | **Painel de progresso**: rosca de domínio, linha com troca de intervalo, ações |
| Exercício | `/prototype/exercicio` | **Laboratório dividido**: enunciado e resposta à esquerda, gráfico vivo à direita | **Cartão em foco**: um item por vez, centralizado, teclado primeiro | **Caderno**: a lista inteira rolando, resposta inline por item, progresso fixo |
| Site | `/prototype/site` | **Problema → prova → CTA** (claro, clássico de conversão) | **Ferramenta no herói**: digite uma função e veja na hora | **Narrativa por scroll** (escuro, estilo Awwwards) |

Depois da primeira rodada vieram as variantes **D** (misturas escolhidas), descritas abaixo.

O que observar ao avaliar: o que o estudante entende em 5 segundos, onde está a única ação
primária, e se a tela diz **o que fazer** em vez de só mostrar dados. A resposta mais útil costuma
ser uma mistura ("o cabeçalho da B com a lista da A").

### Escolha do mantenedor (2026-10-15)

**Direção visual: A — Claro neutro** (índigo `#4F46E5` sobre zinco), nas três telas.

| Tela | Escolha | Rota | O que entrou de cada variante |
| ---- | ------- | ---- | ----------------------------- |
| Início | **A** | `/prototype/inicio?variant=A` | Variante A sem mudanças |
| Exercício | **D** = A + barra da B | `/prototype/exercicio?variant=D` | Layout dividido da A. O cabeçalho troca os pontos por item pela **barra de progresso contínua da B**, recolorida em índigo; ela cresce quando uma parte é acertada |
| Site | **D** = A + botões da B + ideia de movimento da C | `/prototype/site?variant=D` | Layout, texto e paleta da A. No herói, os **três botões de função da B** (`x^3 - 3x + 1`, `x^2`, `sin(x^2)`) trocam o gráfico, a pergunta e a derivada. Da C, só a **ideia** de movimento ligado ao scroll, aplicada às peças da A: no recurso Laboratório a tangente percorre a curva conforme a página rola; em Exercícios as respostas se digitam e recebem o veredito; em Revisão os cartões entram em ordem de data. Textos, cartões e campos **surgem aos poucos em fade** conforme a página rola (e apagam ao voltar), em sequência; leve *parallax* no demo do herói |

As variantes D são o padrão das rotas de Exercício e Site; as originais continuam acessíveis
pela barra para comparação. Uma primeira versão da Site D copiava a cena de pontos da C; o
mantenedor preferiu manter o layout da A e levar só a ideia de movimento.

## 7. Próximos passos
1. ~~Escolher (ou misturar) as variantes por tela e a direção visual.~~ Feito (acima).
2. ~~Registrar a decisão num ADR de design e atualizar `design-system.md` + `tokens.css`.~~ Feito: ADR 0013.
3. ~~Guardar os protótipos num branch descartável e removê-los do `main`.~~ Feito: `prototype/vertice-design`.
4. Teste de 5 segundos do herói do site com 3–5 colegas de cada curso (exatas, ADS, AU).
