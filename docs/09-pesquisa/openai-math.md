# Pesquisa sobre o repositório OpenAI Math

Levantamento em 2026-10-15. Papel: `software-architect`, com pesquisa documental e recomendações.
Não altera arquitetura, roadmap, contratos ou dependências da Vértice.

## Conclusão

O repositório contribui mais como referência de verificação matemática e corpus de pesquisa
avançada. O aproveitamento imediato para os exercícios de Cálculo e VGA é pequeno. Recomenda-se
estudar a separação entre afirmação, evidência e verificação, além de selecionar exemplos para
avaliação de importação e leitura científica. Não há justificativa nesta pesquisa para adicionar
Lean ao aplicativo ou substituir o motor Rust.

Com a decisão de criar o módulo **Biblioteca** (material de estudo, não só exercícios), o
repositório ganha uma segunda utilidade: a forma como organiza, cita, versiona e retira documentos
é um bom modelo para o catálogo de material. Ver
[Modelo de catálogo para a Biblioteca](#modelo-de-catálogo-para-a-biblioteca).

Essa recomendação considera a [proposta da Vértice](../../README.md), sua
[estratégia de IA](../08-ia/estrategia-ia.md) e o
[ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md).

## O que a OpenAI publicou

O README apresenta uma coleção de manuscritos e artefatos de prova produzidos por um modelo
interno ainda não lançado. A coleção tem 719 manuscritos em 372 famílias. As famílias agrupam
trabalhos relacionados por tema e dependência. O próprio projeto informa que existem resultados
em diferentes estágios de verificação e que materiais não formalizados podem conter problemas.
O material publicado não disponibiliza esse modelo nem um tutor pronto para integração.
[Fonte: README](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/README.md).

A consulta identificou o HEAD `1581d4cbdd69a0faf34b2089614fe2a814316a6a`, de 2026-10-12,
às 21:55:24 UTC. O último commit modifica a política de conversas em PRs. Os links deste
relatório fixam essa revisão para permitir nova conferência.
[Fonte: commit](https://github.com/openai/math/commit/1581d4cbdd69a0faf34b2089614fe2a814316a6a).

| Material | Conteúdo observado | Utilidade possível para a Vértice |
| --- | --- | --- |
| `CONTENTS.md` e `overview.tex` | Catálogo de famílias, resumos e links para manuscritos | Referências e seleção por assunto |
| `preprints/` | PDFs, fontes e instruções de citação e construção por manuscrito | Leitura científica e avaliação de extração |
| `lean/` | Biblioteca de formalizações, catálogo YAML, notas de escopo e desafios Comparator | Referência para provas e contratos verificáveis |
| `reasoning_traces/` | Resumos abreviados de raciocínio de dez resultados | Estudo de estratégias de pesquisa, com revisão humana |
| Verificadores de manuscritos específicos | Há também código auxiliar, como os verificadores Python da família 189 | Referência para evidências reproduzíveis |

Fontes: [catálogo](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/CONTENTS.md),
[biblioteca Lean](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/README.md),
[catálogo de formalizações](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/formalization.yaml),
[resumos de raciocínio](https://github.com/openai/math/tree/1581d4cbdd69a0faf34b2089614fe2a814316a6a/reasoning_traces) e
[família 189](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/preprints/Cycle-clique-Ramsey-numbers-September-25-2026/README.md).

## O que a verificação garante e o que permanece em aberto

O histórico de 7 de outubro informa 300 de 719 resultados principais formalizados, aproximadamente
42%. Esse denominador não é o número de famílias. O mesmo histórico registra três manuscritos
retirados por um erro de sinal, além de reparos em outros trabalhos. Portanto, o catálogo é uma
fonte de afirmações e artefatos a examinar, não um gabarito integral já validado por esta pesquisa.
[Fonte: histórico](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/history.md).

O Comparator verifica a correspondência entre o enunciado formal de referência e a prova,
os axiomas permitidos e a aceitação pelo kernel, sob as condições de confiança documentadas.
A definição do problema e a correspondência com o texto matemático ainda precisam de revisão.
A documentação também alerta que desafios com definições abertas exigem verificação adicional.
[Fonte: Comparator](https://github.com/leanprover/comparator#introduction).

É necessário ler a nota de escopo de cada família. As instruções do OpenAI Math distinguem
cinco configurações que verificam apenas resultados auxiliares dos respectivos teoremas
principais. Já a nota da família 189 declara cobertura de todo o intervalo de parâmetros do
teorema correspondente. Nesta pesquisa, essas declarações foram lidas, não reproduzidas.
[Fontes: instruções](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/ComparatorChallenges/README.md) e
[escopo da família 189](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/docs/189.md).

Exemplo autoral aplicável ao corretor da Vértice: `x/x` e `1` coincidem quando `x != 0`.
Uma amostragem que não visita zero pode sugerir equivalência sem mostrar a diferença de domínio.
O critério correto depende do enunciado e do domínio declarado. Teste numérico, identidade
simbólica e prova formal oferecem evidências diferentes. Esse exemplo recomenda explicitar
hipóteses nos testes do corretor; não exige mudar o schema nesta etapa.

## Oportunidades concretas

| Prioridade sugerida | Uso | Experimento delimitado e critério de aceite |
| --- | --- | --- |
| F1, referência de qualidade | Separar resultado calculado, comparação numérica e revisão humana | Conferir casos autorais de domínio e singularidades; nenhum caso deve receber uma garantia maior que a evidência disponível |
| F2, importação assistida | Usar pares de PDF e fonte como corpus complementar | Selecionar páginas públicas e revisar manualmente fórmulas, hipóteses e referências extraídas; medir precisão por campo |
| F2+, leitura científica | Criar roteiros de leitura com pré-requisitos e perguntas | Escolher uma família; produzir explicação PT-BR revisada, citando versão e escopo; avaliar compreensão com exercícios autorais |
| Exploração futura | Provas formais em lógica ou matemática discreta | Avaliar um exercício pequeno e autoral fora do aplicativo; conferir enunciado e axiomas antes de discutir integração |

Os experimentos são recomendações, sem alteração das fases atuais. O importador da Vértice
produz exercícios, enquanto manuscritos contêm teoremas e demonstrações. Extrair texto de um
artigo não demonstra que o sistema consegue segmentar uma lista de exercícios. Esse corpus
complementaria os testes previstos na [estratégia de IA](../08-ia/estrategia-ia.md).

Um exemplo técnico útil é `verification/code/verify.py`, da família 189. O programa executa dois
verificadores, compara saídas esperadas e conserva versões, hashes, duração e resultados em
`run.json`. Confere 3.099 padrões em 42 pares de parâmetros e exige zero casos pendentes.
O README esclarece que os rastros entregues não são objetos de prova separados para um
validador mínimo independente. Vale adaptar o princípio de registrar evidências, sem copiar
essas regras combinatórias para o corretor de Cálculo.
[Fontes: código](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/preprints/Cycle-clique-Ramsey-numbers-September-25-2026/verification/code/verify.py) e
[instruções do manuscrito](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/preprints/Cycle-clique-Ramsey-numbers-September-25-2026/README.md).

## Modelo de catálogo para a Biblioteca

Adendo de 2026-10-15, depois da decisão do mantenedor de criar o módulo **Biblioteca** na F2. O
conteúdo do repositório é avançado demais para o público da Vértice, mas a organização dele
responde a perguntas que a Biblioteca terá de responder, principalmente quando houver material
enviado por usuários e comunidades.

| Prática no OpenAI Math | Aplicação na Biblioteca |
| --- | --- |
| **Famílias** agrupam resultado principal, artigos companheiros, consequências e provas alternativas | Coleções que agrupam material, resumo, notas e exercícios de um mesmo tema |
| Cada manuscrito traz BibTeX e data no identificador | Metadados de citação obrigatórios (autores, DOI, arXiv, URL, versão) |
| `history.md` registra retiradas e correções; edições anteriores continuam acessíveis | Status do material (ativo, corrigido, retirado) com errata e histórico de versões |
| `formalization.yaml` segue um schema externo e liga cada fonte à sua verificação | Catálogo em formato declarado, ligando material a evidências e exercícios |
| README avisa que o conteúdo foi gerado por modelo e pode conter erros | Selo de confiança por item: autoral, fonte aberta, revisado, gerado por IA, verificado pelo motor |
| Resumos de raciocínio mostram o caminho até o resultado | Roteiros de leitura que conduzem à ideia, não só ao enunciado final |

Fontes: [README](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/README.md),
[README de manuscrito](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/preprints/A-C1-Counterexample-to-the-Entropy-Conjecture-September-25-2026/README.md),
[histórico](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/history.md) e
[catálogo de formalizações](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/formalization.yaml).

Para semear a Biblioteca, fontes de nível de graduação são mais adequadas que este catálogo:
Wikipedia (CC BY-SA, mesma licença de `content/`), livros da OpenStax e artigos de revisão do
arXiv referenciados só por link e metadados. A licença da OpenStax varia por título: os três
volumes de Cálculo são CC BY-NC-SA 4.0
([prefácio](https://openstax.org/books/calculus-volume-1/pages/preface)), enquanto outros títulos
podem ser CC BY. Materiais com cláusula NC (Cálculo da OpenStax, MIT OpenCourseWare) são
incompatíveis com CC BY-SA e entram apenas como link. A licença de cada item é registrada
individualmente, e o compartilhamento em comunidades começa restrito a conteúdo autoral. As
decisões de produto e de arquitetura ficam no PRD e no ADR da Biblioteca.

## Matemática e ciência no escopo do produto

Para Cálculo e VGA introdutórios, os artigos não constituem uma sequência didática de limites,
derivadas, integrais e álgebra linear. O catálogo apresenta pesquisa avançada. Para ADS,
há temas de grafos e complexidade que podem inspirar roteiros posteriores. Para ciência,
há física matemática, como vidros de spin e o sistema relativístico Vlasov–Maxwell.
[Fontes: catálogo](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/CONTENTS.md) e
[visão geral](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/overview.tex).

A nota de Vlasov–Maxwell descreve um resultado de existência e unicidade sob hipóteses de
suavidade, suporte e energia. Isso pode sustentar leitura avançada sobre equações de modelos
físicos. Não é um simulador numérico nem validação experimental do modelo físico.
Esta última distinção é uma avaliação de adequação ao produto.
[Fonte: escopo da família 362](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/docs/362.md).

## Integração, licenças e limites da pesquisa

A biblioteca fixa Lean `v4.34.1`, usa Mathlib e outras dependências por commit, e aplica patches
de compatibilidade. O próprio projeto recomenda compilar pequenas partes. Não foi encontrada
nessas interfaces uma ponte pronta para os comandos Rust/WASM da Vértice. Recomenda-se manter
qualquer experimento Lean separado até existir uma necessidade pedagógica e um ADR.
[Fontes: toolchain](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/lean-toolchain),
[configuração](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/lakefile.lean) e
[instruções Lean](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/lean/README.md).

O repositório publica uma licença Apache-2.0. A redistribuição exige cumprir suas condições,
incluindo preservar avisos e identificar modificações. Uma eventual incorporação precisa conferir
também os arquivos e as dependências escolhidas. A licença da origem não vira CC BY-SA por
copiar um arquivo para `content/`. A Vértice deve preservar a procedência e revisar a forma de
distribuição antes de importar material para seu banco público.
[Fonte: licença](https://github.com/openai/math/blob/1581d4cbdd69a0faf34b2089614fe2a814316a6a/LICENSE).

A pesquisa examinou documentação, diretórios e uma amostra de código e contratos formais.
Não verificou os 719 manuscritos, não executou Lean nem os programas auxiliares, não baixou
PDFs e não mediu desempenho ou aprendizagem. As oportunidades acima permanecem hipóteses
de produto. O princípio vigente continua sendo o motor calcular e corrigir, com o LLM
explicando ou extraindo estrutura. Não se recomenda alimentar o tutor com toda a coleção sem
curadoria, nem usar suas afirmações como gabaritos automáticos.
