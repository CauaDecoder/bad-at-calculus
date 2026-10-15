# Visão do produto — Vértice

> **Vértice** é uma plataforma de estudos open source para cursos de exatas, computação e
> arquitetura: **ver** o conceito acontecendo, **praticar** com correção imediata e **revisar** no
> momento certo. Começou como o *Math View*, um laboratório visual de Cálculo e Vetores, que hoje é
> o módulo **Laboratório** da plataforma.

O nome vem do ponto onde as arestas se encontram, e esse é o ponto comum entre os três cursos que
motivam o projeto: o vértice de uma parábola (Cálculo, GA), o vértice de um grafo (Computação) e o
vértice de um projeto (Arquitetura). Ver [ADR 0006](../03-arquitetura/adr/0006-plataforma-vertice.md).

## O problema

Estudantes de exatas alternam entre ferramentas que não conversam: uma calculadora gráfica para
plotar, um CAS para derivar, PDFs de listas com o gabarito na última página, uma IDE para os
exercícios de programação, flashcards em outro app e anotações soltas sobre "os erros que eu
sempre cometo". Cada uma tem uma interface diferente, e muitas são pesadas, fechadas ou datadas.

Faltam três coisas, juntas e no mesmo lugar:

1. **Ver a matemática acontecendo.** Arrastar um ponto e observar a tangente e a derivada se
   formando, decompor uma força num diagrama de corpo livre, ver uma consulta SQL percorrendo as
   tabelas.
2. **Praticar com retorno imediato.** Digitar `y = 5x - 16` e saber na hora se está certo, mesmo
   escrito como `5x - y = 16`, com uma dica sobre o erro específico ("faltou a regra da cadeia").
3. **Lembrar no dia da prova.** Rever exatamente os exercícios errados, no intervalo que a ciência
   da memória recomenda, num plano de estudos com prazo.

## A proposta

| Pilar | O que significa |
| ----- | --------------- |
| **Visual primeiro** | Todo conceito que pode ter uma representação interativa tem. Exercícios abrem a visualização com os mesmos dados ("Abrir no laboratório"). |
| **Prática com correção** | Respostas matemáticas, numéricas, conjuntos, associação, código e autoavaliação, corrigidas pelo mesmo motor Rust que desenha os gráficos. |
| **Conteúdo aberto e próprio** | Banco público com licença aberta (CC BY-SA) que cresce por PR, e pacotes **privados** para o material de cada turma, que nunca saem da máquina do estudante. |
| **Local-first** | Funciona offline e sem conta. Contas e sincronização chegam depois, sem mudar o núcleo. |
| **Rápido e leve** | Núcleo em Rust (WASM no navegador, nativo no desktop e, no futuro, no servidor). |
| **Aberto a contribuir** | Código legível por humanos e agentes de IA, documentado, com ADRs. |

## Público

| Persona | Necessidade principal |
| ------- | --------------------- |
| Estudante de exatas (Física, Engenharias) | Entender Cálculo, GA e Física visualmente e treinar as listas com correção |
| Estudante de ADS / Computação | Praticar algoritmos, POO e SQL com execução e feedback |
| Estudante de Arquitetura e Urbanismo | Estruturas (forças, equilíbrio, esforços), geometria descritiva e conforto ambiental com visualização |
| Estudante em semana de prova | Plano de estudos com prazo, revisão dos erros e simulados |
| Monitor / professor | Montar uma lista com correção automática a partir de um PDF ou planilha |
| Contribuidor | Adicionar um exercício, uma visualização ou uma disciplina sem entender o projeto inteiro |

## Princípios de produto

1. **Mostrar, não só calcular.** Resultado numérico sempre acompanhado de visualização quando ela existir.
2. **Feedback imediato.** Mudou a entrada, mudou o gráfico; respondeu, recebeu a correção.
3. **Ensinar, não resolver por você.** Dicas em passos e IA que conduz o raciocínio (estilo socrático),
   nunca entregam a resposta antes de uma tentativa.
4. **Teclado primeiro, mouse bem-vindo.** Atalhos e command palette.
5. **Notação de verdade.** Entradas e saídas em LaTeX (KaTeX); código com realce de sintaxe.
6. **Privacidade por padrão.** Material de terceiros e dados de estudo ficam no dispositivo até o
   usuário escolher sincronizar.
7. **Offline e leve.** Funciona sem internet; instalador pequeno (Tauri).

## Não-objetivos (por enquanto)

- Ser um CAS completo como Mathematica/Maple, ou um LMS completo como Moodle.
- Contas, comunidade e colaboração em tempo real **antes da Fase 3** do [roadmap](../02-produto/roadmap.md).
- Redistribuir listas, apostilas ou slides de professores sem licença (ver [ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
- Aplicativo mobile nativo (a versão web deve funcionar bem em tablets e celulares).
