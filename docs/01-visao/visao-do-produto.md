# Visão do produto — Vértice

> **Vértice** é uma plataforma de estudos open source para cursos de exatas, computação e
> arquitetura: **estudar** com material de procedência clara, **ver** o conceito acontecendo,
> **praticar** com correção imediata e **revisar** no momento certo. Começou como o *Math View*,
> um laboratório visual de Cálculo e Vetores, que hoje é o módulo **Laboratório** da plataforma.

O nome vem do ponto onde as arestas se encontram, e esse é o ponto comum entre os três cursos que
motivam o projeto: o vértice de uma parábola (Cálculo, GA), o vértice de um grafo (Computação) e o
vértice de um projeto (Arquitetura). Ver [ADR 0006](../03-arquitetura/adr/0006-plataforma-vertice.md).

## O problema

Estudantes de exatas alternam entre ferramentas que não conversam: uma calculadora gráfica para
plotar, um CAS para derivar, PDFs de listas com o gabarito na última página, uma IDE para os
exercícios de programação, flashcards em outro app e anotações soltas sobre "os erros que eu
sempre cometo". Cada uma tem uma interface diferente, e muitas são pesadas, fechadas ou datadas.

Faltam quatro coisas, juntas e no mesmo lugar:

1. **Estudar com o material certo.** O livro, a apostila, o artigo e o vídeo da disciplina ficam
   espalhados, sem ligação com os exercícios, e muitas vezes sem ninguém saber a licença ou se o
   texto foi corrigido. Ver o [PRD da Biblioteca](../02-produto/prd-biblioteca.md).
2. **Ver a matemática acontecendo.** Arrastar um ponto e observar a tangente e a derivada se
   formando, decompor uma força num diagrama de corpo livre, ver uma consulta SQL percorrendo as
   tabelas.
3. **Praticar com retorno imediato.** Digitar `y = 5x - 16` e saber na hora se está certo, mesmo
   escrito como `5x - y = 16`, com uma dica sobre o erro específico ("faltou a regra da cadeia").
4. **Lembrar no dia da prova.** Rever exatamente os exercícios errados e os trechos que destaquei,
   no intervalo que a ciência da memória recomenda, num plano de estudos com prazo.

## A proposta

| Pilar | O que significa |
| ----- | --------------- |
| **Estudar com procedência** | A **Biblioteca** reúne papers, livros abertos, notas de aula, vídeos e arquivos do próprio estudante, cada item com origem, **licença**, selo de confiança e status (ativo, corrigido, retirado). Todo material se liga a habilidades, e o **roteiro de leitura** leva do texto à prática. Material de terceiros fica privado no dispositivo ou entra só como link. |
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
| Estudante em semana de prova | Plano de estudos com prazo, revisão dos erros e dos trechos destacados, simulados |
| Estudante lendo um livro ou artigo | Reunir o material, destacar, e saber o que ler antes e o que praticar depois (roteiro de leitura) |
| Monitor / professor | Montar uma lista com correção automática a partir de um PDF ou planilha; escrever notas de aula e roteiros de leitura |
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
8. **Procedência à vista.** Todo material mostra origem, licença, selo de confiança e status. O
   selo diz quem responde pelo texto e o que foi checado; nunca garante que o texto está certo.

## Não-objetivos (por enquanto)

- Ser um CAS completo como Mathematica/Maple, ou um LMS completo como Moodle.
- Contas, comunidade e colaboração em tempo real **antes da Fase 3** do [roadmap](../02-produto/roadmap.md).
- Redistribuir listas, apostilas ou slides de professores sem licença (ver [ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
- Hospedar ou redistribuir livros, papers e arquivos de terceiros. Material de terceiros fica
  privado no dispositivo do próprio estudante ou entra como link e metadados; só conteúdo
  **autoral** poderá ser compartilhado na comunidade, e só depois de existir um processo de
  denúncia e remoção ([PRD da Biblioteca](../02-produto/prd-biblioteca.md)).
- Ser um drive genérico ou um gerenciador de referências (estilo Zotero): a Biblioteca guarda
  material de estudo **ligado a habilidades**, não qualquer arquivo nem bibliografia de TCC.
- Aplicativo mobile nativo (a versão web deve funcionar bem em tablets e celulares).
