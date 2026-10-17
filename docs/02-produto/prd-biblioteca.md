# PRD — Biblioteca (material de estudo)

- **Status:** rascunho (F1 só `references[]`; módulo completo na F2; nuvem na F3; comunidade na F4)
- **Responsável:** product-manager (ver [AGENTS.md](../../AGENTS.md#papéis))
- **Relacionados:** [visão](../01-visao/visao-do-produto.md) · [módulos](modulos.md) ·
  [roadmap](roadmap.md) · [PRD de exercícios](prd-exercicios.md) · [taxonomia](taxonomia.md) ·
  [formato de exercícios](formato-de-exercicios.md) ·
  [ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md) ·
  [ADR 0014 — Biblioteca](../03-arquitetura/adr/0014-biblioteca.md) (aceito; decisões técnicas moram lá) ·
  [pesquisa OpenAI Math](../09-pesquisa/openai-math.md#modelo-de-catálogo-para-a-biblioteca)

## 1. Problema

O estudante não estuda só resolvendo exercícios. Antes de resolver, ele **lê**: o capítulo do livro
indicado pelo professor, a apostila, uma página da Wikipedia, um artigo de revisão, um vídeo. Hoje
esse material vive longe da prática.

Exemplo concreto, semana de prova de Cálculo A. A Ana errou o item 9 da Lista 04 (regra da cadeia).
Ela sabe que existe uma seção sobre isso num livro, mas:
- o PDF está numa pasta, o vídeo num favorito, o resumo num caderno, e a lista noutro app;
- ela abre o PDF, sublinha uma definição e, três dias depois, não acha o trecho nem lembra por que
  o marcou;
- o que ela destacou nunca volta a ela no momento certo (não vira revisão);
- ao terminar de ler, não sabe **quais exercícios** treinam aquilo.

Outro exemplo, estudante de ADS. O professor passa um artigo sobre complexidade de grafos. O texto
assume pré-requisitos (notação de conjuntos, indução) que ele não tem, e ele não sabe o que ler
antes. Um estudante de AU enfrenta o mesmo com a apostila de Estruturas: o texto fala em "momento
fletor" e ele precisa rever vetores primeiro.

Faltam três coisas:
1. **Um lugar só** para o material de estudo da disciplina, com a origem e a licença à vista.
2. **Ligação com a prática**: depois de ler, saber o que exercitar; ao errar, saber o que reler.
3. **Confiança**: saber de onde o material vem, se foi revisado, se foi corrigido ou retirado.

### Referências de mercado (o que aproveitar e o que evitar)

| Plataforma | O que faz bem | Aproveitamos | Evitamos |
| ---------- | ------------- | ------------ | -------- |
| **Zotero** | Leitor de PDF com destaques e notas; extrai anotações para notas com link de volta à página ([Zotero 6](https://www.zotero.org/blog/zotero-6/), [guia UCL](https://library-guides.ucl.ac.uk/zotero/extracting-annotations)) | Destaque com link de volta ao trecho; anotações pesquisáveis | Virar gerenciador de bibliografia (citações em Word, grupos de pesquisa) |
| **Readwise** | Reúne destaques e os devolve por revisão espaçada; converte em cartões ([Readwise](https://readwise.io/paperlessmovement)) | Destaque → cartão de revisão | Depender de nuvem e assinatura; a revisão aqui é o FSRS local |
| **Khan Academy** | Curso organizado em unidades e habilidades, com níveis de domínio ([ajuda](https://support.khanacademy.org/hc/bg/articles/5548760867853)) | Ligar material a habilidades e à prática | Conteúdo fechado e hospedado só por eles |
| **Semantic Reader** (Semantic Scholar) | Leitor de artigos com definições em *tooltips* e destaques para skimming ([produto](https://webflow.semanticscholar.org/product/semantic-reader)) | Inspiração para auxílios de leitura de paper em fases futuras | Auxílios gerados por IA sem selo e sem revisão |
| **OpenAI Math** (repositório) | Famílias de documentos, BibTeX, `history.md` com retiradas, versões antigas acessíveis | Modelo de **status e errata** ([pesquisa](../09-pesquisa/openai-math.md#modelo-de-catálogo-para-a-biblioteca)) | Conteúdo avançado demais para o público |

## 2. Princípios do módulo

1. **Estudar → ver → praticar → revisar.** A Biblioteca é o "estudar". Todo material leva a uma
   visualização, a um exercício ou a um cartão; material sem caminho para a prática é exceção.
2. **Habilidades como cola.** Todo item se liga a habilidades da [taxonomia](taxonomia.md)
   (`calculo.derivadas.regra-da-cadeia`). É isso que alimenta o ciclo: ler → praticar a habilidade
   → errar → reler.
3. **Procedência e licença sempre visíveis.** Nenhum item existe sem origem, autor (quando
   conhecido) e licença (ou "desconhecida / proprietária"). O que não podemos redistribuir, só
   linkamos ou guardamos privado no dispositivo do próprio usuário
   ([ADR 0012](../03-arquitetura/adr/0012-conteudo-de-terceiros-e-licencas.md)).
4. **Confiança é declarada, não presumida.** Selo e status por item. O selo **nunca** garante que
   o texto está correto; diz **quem responde por ele** e **o que foi checado**.
5. **Privado por padrão.** Arquivo do usuário, anotações e progresso ficam no dispositivo até ele
   escolher sincronizar (F3). Nada de conteúdo privado vai a um serviço externo sem confirmação
   por ação.
6. **A IA não corrige nem vira autoridade.** Resumos e rascunhos de roteiro por IA levam o selo
   "gerado por IA" e não recebem "revisado" sem pessoa. Perguntas de checagem corrigidas
   automaticamente passam pelo motor, nunca por LLM ([estratégia de IA](../08-ia/estrategia-ia.md)).

## 3. Personas

| Persona | O que precisa da Biblioteca |
| ------- | --------------------------- |
| **Estudante** (Ana, Cálculo A) | Reunir o material da disciplina, ler, destacar e saber o que praticar depois |
| **Estudante em semana de prova** | Voltar rápido ao trecho certo e rever os destaques no intervalo certo |
| **Estudante lendo um artigo** (ADS, Física) | Roteiro com pré-requisitos, para não ler às cegas |
| **Autor** (monitor, professor, colaborador) | Escrever notas de aula e roteiros de leitura, escolher a licença, ligar a exercícios |
| **Contribuidor do catálogo** | Propor uma fonte aberta, com licença conferida, por pull request |
| **Moderador** (F4) | Analisar denúncias e decidir sobre remoção |
| **Titular de direitos** (F4) | Denunciar um material seu e ter retorno |

## 4. Tipos de material

| Tipo | Exemplo | Como entra | Onde pode estar | Compartilhável na F4? |
| ---- | ------- | ---------- | --------------- | --------------------- |
| **Paper** | Artigo do arXiv, com DOI | Link + metadados | Catálogo (sem arquivo) | Sim, como link + metadados |
| **Livro aberto** | Livro com licença aberta compatível; Wikipedia (CC BY-SA) | Link e metadados; trecho adaptado vira nota autoral com atribuição ([ADR 0014](../03-arquitetura/adr/0014-biblioteca.md)) | Catálogo | Sim, como link; a nota adaptada segue a licença dela |
| **Nota de aula autoral** | Resumo de derivadas escrito por um monitor | Escrita no editor ou PR | `content/` (CC BY-SA) ou privada | Sim (autoria declarada) |
| **Arquivo do usuário** | PDF da apostila do professor | Upload local | **Só no dispositivo** (privado) | **Não** |
| **Vídeo** | Aula no YouTube | Link + metadados | Catálogo (sem arquivo) | Sim, como link + metadados |
| **Roteiro de leitura** | "Regra da cadeia: do conceito à prova" | Escrito no editor ou PR | `content/` (CC BY-SA) ou privado | Sim (autoral) |

**Roteiro de leitura** é o item de maior valor pedagógico: material (ou trecho) + pré-requisitos +
perguntas de checagem + exercícios + "Abrir no laboratório", em ordem. É o que transforma "uma
pilha de PDFs" em um caminho de estudo. Um roteiro pode apontar para material de terceiros por
link, mas o texto do roteiro é autoral.

### Selo de confiança e status

| Selo (pode haver mais de um) | Significa |
| ---------------------------- | --------- |
| **Autoral** | Escrito por quem o publica, que responde pela autoria |
| **Fonte aberta** | Vem de fonte com licença aberta, conferida e com atribuição |
| **Revisado por monitor** | Uma pessoa identificada revisou (data e nome registrados) |
| **Gerado por IA** | Texto produzido ou rascunhado por modelo; sempre visível |
| **Verificado pelo motor** | Vale **apenas** para perguntas de checagem e exercícios cuja resposta o motor conferiu; nunca para o texto explicativo |

| Status | Significa |
| ------ | --------- |
| **Ativo** | Em uso normal |
| **Corrigido** | Há errata publicada; a versão atual traz a correção |
| **Retirado** | Não deve mais ser usado (erro grave, direitos autorais, pedido do autor); continua visível como aviso, com motivo |

Inspirado no [OpenAI Math](../09-pesquisa/openai-math.md#modelo-de-catálogo-para-a-biblioteca):
retiradas e correções ficam registradas e versões antigas continuam acessíveis (exceto quando a
retirada for por direitos autorais; aí o arquivo sai, e o registro e o motivo ficam).

## 5. Objetivos e métricas

Sem telemetria por padrão (como no [PRD de exercícios](prd-exercicios.md)). As metas abaixo são
verificadas por testes automatizados, revisão manual ou teste com estudantes piloto.

| Fase | Objetivo | Métrica |
| ---- | -------- | ------- |
| F1 | Ligar a prática ao estudo | ≥ 60% dos exercícios públicos de Cálculo/VGA têm ≥ 1 referência ligada |
| F2 | Estudar sem sair do app | Estudante piloto adiciona um PDF próprio, lê, destaca e volta ao trecho em < 2 min na primeira tentativa (5 pilotos, ≥ 4 conseguem) |
| F2 | Privacidade dos arquivos próprios | 0 requisições de rede com conteúdo de arquivo privado, verificado por teste automatizado (E2E) |
| F2 | Não perder anotação | 0 destaques perdidos após fechar e reabrir o app (teste de persistência) |
| F2 | Ler → revisar | Destaque vira cartão em ≤ 3 cliques |
| F2 | Roteiros que funcionam | ≥ 5 roteiros autorais de Cálculo/VGA no lançamento, cada um com ≥ 1 exercício e ≥ 1 visualização ligados |
| F2 | Licença confiável | 100% das entradas do catálogo com licença, data de conferência ("verificado em") e URL da licença |
| F2 | Abertura rápida | Primeira página de um PDF de 300 páginas em < 2 s em notebook comum |
| F4 | Responder a denúncias | 100% das denúncias com decisão registrada dentro do prazo definido na política (prazo a definir com avaliação jurídica) |

## 6. Histórias e critérios de aceite

### F1 — Referências nos exercícios

**B1. Ver de onde estudar.** Como estudante, quero ver, num exercício, links para material que
explica o assunto, para saber o que ler quando travo.
- Dado um exercício com `references[]`, quando eu o abro, então vejo "Para estudar" com título,
  tipo, fonte e licença de cada referência.
- Quando clico numa referência de tipo link, então ela abre fora do app (navegador), sem o app
  baixar nem guardar o conteúdo.
- Dado que estou offline, quando abro o exercício, então vejo os metadados das referências e um
  aviso de que o link precisa de internet; o exercício continua funcionando.
- Dado um exercício sem referências, então a seção "Para estudar" não aparece.

**B2. Autor liga referência.** Como autor de exercício, quero declarar referências (livro,
capítulo/seção, URL, DOI) no exercício, para o estudante saber onde estudar.
- Dado um `*.ex.md` com referência sem licença ou sem origem, quando a CI valida, então falha com
  a linha e o campo.
- Dado uma referência que aponta para material de terceiros, então ela deve ser **link/metadados**;
  a CI falha se houver arquivo anexado (ADR 0012, item 7).

### F2 — Biblioteca local

**B3. Adicionar um arquivo próprio.** Como estudante, quero adicionar o PDF que meu professor me
passou, para estudar nele dentro da Vértice.
- Dado um PDF no meu computador, quando o adiciono, então ele aparece na Biblioteca marcado
  **privado** e com licença "proprietária/desconhecida", e o arquivo fica **só no dispositivo**.
- Quando exporto a biblioteca, então o arquivo só entra se eu marcar essa opção, e nunca em
  exportação "pública".
- Dado qualquer recurso de IA, quando ele usaria o conteúdo do arquivo, então peço confirmação por
  ação, mostrando o que seria enviado e para quem.
- Dado um arquivo que não é PDF válido ou excede o limite, então recebo mensagem clara e nada é
  adicionado. (Limite e tipos aceitos: ADR 0014.)

**B4. Ler e retomar.** Como estudante, quero abrir o material e voltar de onde parei, para não
perder tempo.
- Dado um PDF aberto na página 87, quando fecho e reabro o app, então ele abre na página 87.
- Quando busco um termo, então vejo as ocorrências no texto do documento aberto (quando o PDF tem
  camada de texto).
- Dado que estou offline, então o leitor funciona para arquivos já na Biblioteca.
- Dado um PDF escaneado sem texto, então a busca avisa que o documento não tem texto pesquisável.

**B5. Destacar e anotar.** Como estudante, quero destacar um trecho e escrever uma nota, para
achar depois e entender por que marquei.
- Dado um trecho selecionado, quando escolho uma cor e opcionalmente escrevo uma nota, então o
  destaque é salvo com a página e o trecho.
- Quando clico num destaque na lista, então o leitor vai direto à posição.
- Dado que fechei e reabri o app, então todos os destaques e notas continuam lá.
- Quando excluo um destaque que originou um cartão, então o app pergunta se o cartão também deve
  ser excluído; por padrão ele fica, independente do destaque.
- A nota aceita Markdown e LaTeX e é sanitizada (conteúdo do usuário não é confiável).

**B6. Destaque vira cartão de revisão.** Como estudante, quero transformar um destaque em cartão
FSRS, para lembrar dele no dia da prova.
- Dado um destaque, quando escolho "Criar cartão", então posso editar frente e verso (por padrão:
  frente = pergunta que eu escrevo; verso = trecho destacado) e o cartão entra na fila de revisão.
- Na F2 o cartão é criado **só manualmente**: o app não sugere pergunta por IA.
- Dado um cartão vindo de arquivo privado, então ele herda a visibilidade **privada**.
- Quando reviso o cartão, então há um atalho "Abrir no material" que leva à página de origem.
- Dado que o material de origem foi removido, então o cartão continua funcionando e indica "origem
  indisponível".

**B7. Adicionar fonte por link e metadados.** Como estudante ou autor, quero registrar um artigo,
livro ou vídeo por DOI, arXiv ou URL, para tê-lo na Biblioteca sem baixar arquivo.
- Dado um identificador (DOI, arXiv ou URL), quando o informo, então o item é criado do tipo link,
  com título, autores, ano, fonte e licença (ou "desconhecida").
- Dado que estou offline ou o preenchimento automático não encontra dados, então posso preencher à
  mão.
- Dado um item de link, então o app **não** guarda cópia do conteúdo remoto.
- Quando o preenchimento automático consulta um serviço externo, então o usuário é avisado de qual
  serviço recebe o identificador (ver pontos em aberto).

**B8. Ligar material a habilidades.** Como estudante, quero que cada material diga quais
habilidades cobre, para praticar o que acabei de ler.
- Dado um item, quando escolho habilidades da taxonomia, então ele aparece na página dessas
  habilidades e, nelas, vejo exercícios e visualizações relacionados.
- Dado uma habilidade que não existe na taxonomia, então o app não permite salvar o vínculo e
  oferece "propor habilidade" (PR de taxonomia).
- Dado um item **privado** sem habilidade, então ele aparece como "sem classificação" e não
  alimenta a recomendação de prática.
- Dado um item do catálogo público ou compartilhado (F4), quando não tem nenhuma habilidade, então
  não pode ser publicado.
- Quando clico em "Praticar esta habilidade" num material, então abro a lista de exercícios
  filtrada por aquela habilidade.

**B9. Seguir um roteiro de leitura.** Como estudante, quero seguir um roteiro passo a passo, para
estudar um tema em ordem, sem me perder em pré-requisitos.
- Dado o roteiro "Regra da cadeia", quando o abro, então vejo as etapas: pré-requisitos, material,
  perguntas de checagem, exercícios e visualização.
- Quando marco uma etapa como concluída, então o progresso é salvo e o roteiro reabre onde parei.
- Dado que errei a pergunta de checagem, então o roteiro indica o pré-requisito ou o trecho a
  rever, sem me bloquear.
- Dado uma pergunta de checagem com resposta corrigível, então a correção vem do motor; se for
  autoavaliada, eu marco o resultado.
- Quando clico em "Abrir no laboratório" numa etapa, então abro a visualização com os dados do
  roteiro.
- Dado que uma etapa aponta para material que não tenho (ex.: arquivo próprio), então vejo
  "material não disponível aqui" com a citação, e o roteiro segue funcionando.

**B10. Escrever um roteiro ou nota de aula.** Como autor, quero criar um roteiro com preview e
publicá-lo no banco aberto, para outros estudantes usarem.
- Dado um rascunho de roteiro, quando salvo, então vejo o preview igual ao que o estudante verá.
- Quando abro um PR com o roteiro em `content/`, então a CI valida: licença presente (CC BY-SA),
  `source` e autoria, habilidades existentes, exercícios e visualizações referenciados existem, e
  nenhum arquivo de terceiros.
- Dado que o roteiro cita material de terceiros, então ele entra só como link e metadados.
- Dado que declaro um item como "revisado por monitor", então a CI exige nome e data do revisor.

**B11. Catálogo de fontes abertas.** Como estudante, quero um catálogo curado de livros e notas
abertas por disciplina, para saber onde estudar sem procurar.
- Dado o catálogo, quando filtro por disciplina e habilidade, então vejo os itens com licença,
  "verificado em" e, quando aplicável, o aviso "somente link".
- Dado um item com licença NC (ex.: MIT OpenCourseWare), então ele aparece como **somente link**:
  o app não oferece copiar para `content/` nem incluir em packs públicos.
- Quando uma fonte tem edição com licença diferente da anterior (ex.: Active Calculus), então o
  item registra a licença **por edição**.
- Dado um link quebrado detectado pela verificação periódica, então o item recebe um aviso e entra
  na fila de manutenção.

**B12. Status, errata e retirada.** Como estudante, quero saber se um material foi corrigido ou
retirado, para não estudar errado.
- Dado um item **corrigido**, quando o abro, então vejo a errata (o que mudou, quando).
- Dado um item **retirado**, quando o abro, então vejo um aviso com o motivo (quando público) e, se
  houver, o substituto; os exercícios que o referenciam mostram o mesmo aviso.
- Dado um item que eu já tinha destacado e que foi retirado, então minhas anotações continuam
  minhas, com aviso.
- Dado um item retirado por direitos autorais, então o arquivo deixa de ser distribuído, mas o
  registro (título, motivo, data) permanece.

**B13. Buscar na Biblioteca.** Como estudante, quero buscar por título, autor, habilidade, tag e
nas minhas anotações, para achar rápido na semana de prova.
- Dado a busca "regra da cadeia", então vejo itens, roteiros e minhas anotações que casam, em
  < 200 ms para uma biblioteca de até 500 itens.
- Dado que estou offline, então a busca funciona (é local).

**B14. Exportar e fazer backup.** Como estudante, quero exportar minha biblioteca, anotações e
progresso, para não perder nada e trocar de máquina.
- Quando exporto, então recebo um arquivo com metadados, anotações, vínculos e progresso, e
  **opcionalmente** os arquivos próprios.
- Dado que importo esse backup noutra máquina, então a Biblioteca fica igual (testado por
  ida-e-volta).

**B15. Rascunho assistido por IA (opcional na F2, depois do núcleo).** Como autor, quero que a IA
rascunhe um roteiro ou resumo a partir de um material **aberto**, para ganhar tempo, mantendo a
revisão humana.
- Dado um rascunho gerado, então ele leva o selo "gerado por IA" e não pode receber "revisado"
  sem revisor identificado.
- Dado material privado, então o envio ao provedor exige confirmação por ação.
- Dado uma fonte com restrição ao uso por IA (ver riscos), então o app não a envia a modelos.
- Todo exercício ou pergunta com resposta no rascunho é conferido pelo motor antes da publicação.

### F3 — Nuvem privada

**B16. Meus materiais em todos os dispositivos.** Como estudante com conta, quero sincronizar
minha biblioteca privada, para estudar no celular o que adicionei no notebook.
- Dado que ativei a sincronização de um item, então só eu consigo acessá-lo; compartilhar exige
  outra ação explícita (e a F4).
- Dado que desativo a sincronização ou excluo a conta, então o item e as anotações são apagados da
  nuvem e eu continuo com a cópia local.
- Dado um arquivo de terceiros, quando sincronizo, então **só metadados, destaques, notas e
  progresso** sobem para a nuvem; o arquivo continua só no dispositivo.
- Dado um arquivo de terceiros que já existe em outro dispositivo meu, quando eu o adiciono lá,
  então o app reconhece o mesmo arquivo e reassocia os destaques e as notas sincronizados.
- Dado um arquivo **autoral** meu, quando ativo a sincronização, então o arquivo também sobe para a
  nuvem privada.

### F4 — Comunidade

**B17. Compartilhar conteúdo autoral num grupo.** Como autor, quero publicar minhas notas e
roteiros para meu grupo ou turma, para que colegas estudem com eles.
- Dado que publico um item, então preciso **declarar que sou o autor** e **escolher uma licença
  aberta** da lista permitida; sem isso, não publica.
- Dado um arquivo marcado privado ou de terceiros, quando tento compartilhar, então o app bloqueia
  e oferece compartilhar **link e metadados**.
- Dado um item compartilhado, então ele mostra autor, licença, data e selos.

**B18. Denunciar um material.** Como qualquer pessoa (inclusive titular de direitos, com ou sem
conta), quero denunciar um material que viola direitos, para ele ser analisado e removido se
necessário.
- Dado qualquer item compartilhado, então há um botão "Denunciar" visível, e a denúncia também pode
  ser feita sem login por um formulário público.
- Quando envio a denúncia, então informo: item, motivo, justificativa e meio de contato; recebo um
  protocolo.
- Quando a denúncia é recebida, então o item entra em **análise** e o autor é notificado.
- Dado que o autor discorda, então ele pode responder (contestação) e o caso segue para um
  moderador.
- Prazos, o que acontece com o item durante a análise (ocultar ou manter) e o texto das mensagens
  dependem da avaliação jurídica; este PRD só fixa que **o processo existe antes do upload
  compartilhado abrir**.

**B19. Moderar.** Como moderador, quero uma fila de denúncias com histórico, para decidir com
consistência.
- Dado uma denúncia, quando decido (manter, corrigir, retirar), então a decisão, o motivo e a data
  ficam registrados e visíveis ao autor e ao denunciante.
- Dado um item retirado, então o status passa a **retirado** e o motivo aparece conforme B12.
- Dado reincidência do mesmo autor, então a fila sinaliza o histórico.
- Dado uma decisão, então o autor pode recorrer uma vez.

## 7. Escopo por fase

| Fase | Entra | Não entra ainda |
| ---- | ----- | --------------- |
| **F1** | `references[]` nos exercícios (título, tipo, fonte, licença, link); validação na CI; seção "Para estudar" no player | Leitor, upload, catálogo, roteiros |
| **F2** | Biblioteca local: upload de PDF próprio **só no dispositivo** (como packs privados); leitor com busca e retomada; destaques e notas; destaque → cartão FSRS; vínculo com habilidades; roteiros autorais em `content/` (editor + PR); catálogo de fontes abertas **por link**; status, selos e errata; busca local; backup | Nuvem, contas, compartilhamento, moderação |
| **F3** | Sincronização (depende de contas): metadados, destaques, notas e progresso; arquivos **autorais** na nuvem **privada**; cota por usuário | Arquivo de terceiros na nuvem; compartilhar com terceiros |
| **F4** | Compartilhar **conteúdo autoral** em grupos e turmas (autoria + licença aberta); material de terceiros só como **link + metadados**; denúncia, remoção e moderação; reputação e comentários | Hospedar arquivo de terceiros em qualquer hipótese |

Rotas: base `/biblioteca` (ex.: `/biblioteca`, `/biblioteca/roteiros`, `/biblioteca/ler/...`);
detalhe de rotas é de produto/UX e pode mudar até a F2.

## 8. Fora de escopo (não-objetivos)

- **Drive genérico**: a Biblioteca guarda material **de estudo ligado a habilidades**, não qualquer
  arquivo.
- **Gerenciador de referências** (estilo Zotero): sem plugin de Word/LaTeX, sem formatação ABNT/APA
  de bibliografia de TCC, sem sincronizar com Zotero/Mendeley, sem coleções de pesquisa para
  publicar.
- **Hospedar ou redistribuir arquivos de terceiros**, em qualquer fase, para qualquer usuário.
- **Baixar automaticamente PDFs** de sites ou repositórios em nome do usuário; o app só abre links.
- **Material NC no banco público**: MIT OpenCourseWare e similares entram só como link.
- **LMS** (turmas com notas, entrega de trabalho) e **colaboração em tempo real** em anotações.
- **OCR de PDF escaneado** e **busca semântica** sobre o acervo (avaliar depois, ver
  [estratégia de IA](../08-ia/estrategia-ia.md)).
- **Hospedar vídeo ou áudio.** Vídeo é sempre link.
- **Verificação formal** (Lean etc.) de papers; o selo "verificado pelo motor" cobre só perguntas
  e exercícios.
- **Livro/curso completo** produzido por nós na F2; entram notas e roteiros curtos.
- **Aplicativo mobile nativo** (igual à visão).

## 9. Riscos

O principal é **direitos autorais**.

| # | Risco | Impacto | Mitigação (produto) |
| - | ----- | ------- | ------------------- |
| R1 | **Direitos autorais**: usuário envia/compartilha material de terceiros (Lei 9.610/98) | Alto: retirada de conteúdo, responsabilidade do mantenedor, perda de confiança | Fases: local → nuvem privada → compartilhar só autoral; terceiros só link+metadados; declaração de autoria e licença; denúncia e remoção **antes** de abrir o compartilhamento; avaliação jurídica |
| R2 | Licença registrada errada no catálogo | Alto: redistribuímos o que não podíamos | "Verificado em" + URL da licença por item e **por edição**; dois revisores no PR do catálogo; na dúvida, só link |
| R3 | Selo mal interpretado (ex.: "verificado" lido como "o texto está certo") | Médio | Texto fixo explicando o escopo de cada selo; "verificado pelo motor" só em perguntas e exercícios |
| R4 | Conteúdo gerado por IA com erro apresentado como confiável | Médio | Selo obrigatório; sem "revisado" sem pessoa identificada; motor confere respostas |
| R5 | Fontes com restrição de uso por IA | Médio | Registrar a restrição no item; recursos de IA não enviam essas fontes |
| R6 | Escopo vira "Zotero + Drive" | Médio | Não-objetivos acima; todo item precisa de habilidade |
| R7 | PDF malicioso ou anotações/Markdown com script | Alto | Tratar arquivo e texto do usuário como não confiáveis; acionar `security-researcher` antes da F2 |
| R8 | Links quebrados | Baixo | Verificação periódica; metadados guardados mesmo sem o link |
| R9 | Carga de moderação sobre mantenedores voluntários | Médio | Só abrir a F4 quando houver processo e pessoas; começar por grupos pequenos |
| R10 | Privacidade: a lista do que o estudante lê é sensível | Médio | Local por padrão; consulta externa de metadados só com aviso |

**Pré-requisitos para abrir upload compartilhado (F4)** — todos devem estar prontos:
1. Processo de **denúncia e remoção** (notice and takedown) definido e funcionando, alinhado ao
   Marco Civil da Internet (Lei 12.965/2014) e à Lei de Direitos Autorais (Lei 9.610/98).
2. **Avaliação jurídica**, feita antes de abrir a F4, sobre esse processo, os termos de uso e a posição do projeto
   como provedor. Este PRD não é parecer jurídico. Obs.: o STF alterou em 2025 o regime geral de
   responsabilidade de provedores do art. 19 ([Machado Meyer](https://www.machadomeyer.com.br/pt/inteligencia-juridica/publicacoes-ij/direito-digital/stf-muda-regime-de-responsabilizacao-das-plataformas));
   o efeito sobre direitos autorais não foi confirmado nesta pesquisa.
3. **Termos de uso** e declaração de autoria/licença no ato de publicar.
4. Fila de moderação e pessoas responsáveis.
5. Revisão de segurança do upload (`security-researcher`).

## 10. Casos de borda

- PDF próprio que, na verdade, é um livro de licença aberta: continua sendo "arquivo do usuário"
  (privado) a menos que o usuário o cadastre no catálogo com a licença conferida.
- O mesmo livro com edições de licenças diferentes (ex.: Active Calculus tem edições sob
  CC BY-NC-SA 3.0 e posteriores sob CC BY-SA 4.0 ([LibreTexts](https://math.libretexts.org/Bookshelves/Calculus/Active_Calculus_(Boelkins_et_al.)))).
- Fonte aberta com **imagens** de licença diferente do texto (ex.: Wikipedia): a licença é
  registrada por componente quando se copia; por link, não importa.
- Destaque num PDF cujo arquivo foi substituído por outra edição: o destaque guarda o trecho, e o
  app avisa "posição pode ter mudado".
- Roteiro que referencia um exercício privado do usuário A, aberto pelo usuário B (F4): o exercício
  aparece como indisponível.
- Item retirado que era pré-requisito de um roteiro: o roteiro mostra aviso e sugere o substituto.
- Autor apaga a conta (F3/F4): o conteúdo publicado sob licença aberta continua com atribuição;
  o conteúdo privado é apagado.
- Material em outro idioma: o item registra o idioma; roteiros em PT-BR podem apontar para fonte em
  inglês.

## 11. Requisitos não funcionais (de produto)

- Funciona offline para tudo que já está na Biblioteca local.
- Acessível: navegação por teclado no leitor, contraste AA, texto alternativo em figuras de
  roteiros autorais.
- Textos da interface em PT-BR em `lib/ui/copy/`.
- Segurança: arquivos e anotações do usuário são não confiáveis
  ([modelo de ameaças](../06-seguranca/modelo-de-ameacas.md)).

## 12. Pontos em aberto

**Decididos pelo mantenedor (2026-10-15)**
- Licença padrão para compartilhar conteúdo autoral na F4: **CC BY-SA 4.0** (igual a `content/`),
  com CC BY 4.0 e CC0 como opções.
- Na F3, arquivo de terceiros **não** vai para a nuvem: sincronizam só metadados, destaques, notas
  e progresso (B16).
- A **avaliação jurídica** é feita antes da F4; não bloqueia a F3.
- Na F2, cartão a partir de destaque é **só manual** (B6).

**Para o mantenedor (produto/jurídico)**
1. **Durante a análise de uma denúncia**, o item fica oculto ou visível? Prazos e política de
   reincidência. Depende da avaliação jurídica.
2. Quem faz a **avaliação jurídica**.
3. **Correção do ADR 0012** (a pesquisa já foi corrigida; o ADR 0012 não é reescrito, a correção
   fica registrada no ADR 0014): o texto cita OpenStax como CC BY, mas os três volumes de
   Cálculo da OpenStax são **CC BY-NC-SA 4.0**
   ([prefácio](https://openstax.org/books/calculus-volume-1/pages/preface)) e a página do Volume 1
   proíbe o uso em treinamento de modelos sem permissão. Outros títulos da OpenStax podem ter
   licença diferente; conferir livro a livro. Para Cálculo, a OpenStax entra como **somente link**.

**Resolvidos pelo `software-architect` nos ADRs 0014 e 0015, aceitos em 2026-10-15**

O [spike do arXiv](../09-pesquisa/spike-arxiv-cors.md) concluiu o teste de CORS: preenchimento
manual na web; consulta por comando Tauri prevista para o desktop.

4. Modelo de dados, formato do item/roteiro/catálogo, formato de `references[]`, armazenamento de
   arquivos e anotações, onde fica o contexto no código, limites de upload e tipos aceitos.
5. Leitor de PDF: biblioteca, sanitização e isolamento; busca de texto; destaque estável entre
   edições do arquivo.
6. Consulta de metadados por DOI/arXiv: qual serviço, quando consultar e como avisar o usuário
   (privacidade).
7. Como o selo "verificado pelo motor" é registrado e invalidado.
8. Versionamento de itens e retenção de versões antigas (inclusive quando retirada for por
    direitos autorais).
9. Granularidade do vínculo com habilidades (item inteiro, capítulo ou intervalo de páginas).

**Para o `product-manager`/UX**
10. Formatos além de PDF (EPUB, HTML salvo, Markdown): F2 ou depois?
11. Exportar citação (BibTeX) de um item: vale a pena, ou vira escopo de gerenciador de referências?
12. Busca no texto completo dos PDFs próprios (indexação local) entra na F2?
13. Quantos itens/roteiros mínimos por disciplina para lançar a F2?
14. Coleções ("famílias") de materiais por tema: entram na F2 ou depois do primeiro uso real?

**Para outros papéis**
15. `security-researcher`: ameaças de PDF/Markdown/SVG enviados por usuário e do upload
    compartilhado (F4).
16. `content-author`: semear o catálogo e os primeiros roteiros; conferir licença de cada fonte.
17. `ai-engineer`: política de quais fontes podem ir a modelos; avaliação de rascunho de roteiro.
