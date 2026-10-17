# F2: ampliar o estudo local

Comece pelas [regras para agentes](regras-para-agentes.md). F2 acrescenta revisão, Biblioteca,
IA opcional e novas disciplinas à base offline. O [roadmap](../../02-produto/roadmap.md#f2--ia-revisão-e-novas-disciplinas)
mantém o estado da fase. Execute uma unidade por vez, com suas dependências verificadas.

## Conferir as entradas

F1 deve entregar conteúdo, correção, tentativas, visualizações e persistência. Leia o
[PRD da Biblioteca](../../02-produto/prd-biblioteca.md), os ADRs
[0014](../../03-arquitetura/adr/0014-biblioteca.md) e
[0015](../../03-arquitetura/adr/0015-evidencia-de-verificacao.md), o
[modelo de dados](../../03-arquitetura/modelo-de-dados.md), a
[estratégia de IA](../../08-ia/estrategia-ia.md) e os
[módulos por disciplina](../../02-produto/modulos.md).

Mantenha o app utilizável sem conta, sem servidor próprio e sem LLM. Arquivos de terceiros
continuam locais. Recursos de IA não decidem correção e não recebem material privado sem a
confirmação exigida. Biblioteca não inclui um gerenciador completo de referências nem um drive.

## Executar com dependências explícitas

### F2.1: revisar as novas fronteiras antes da implementação

Papel: `security-researcher`; contratos por `software-architect`. Dependência: F1 verificada.

Revise T7 e T4/T5 no [modelo de ameaças](../../06-seguranca/modelo-de-ameacas.md). Produza
achados com cenário, severidade, evidência ou indicação de hipótese e correção recomendada.
Confira as dependências do leitor em fontes oficiais atuais. Encaminhe correções aos
implementadores. Defina critérios de verificação antes de abrir PDF ou executar código do usuário.

O arquiteto confere as interfaces públicas e o teste de arquitetura exigido pelo ADR 0014.
O papel responsável pelos testes implementa esse teste antes do primeiro código da `library`.

Conclua com relatório de revisão, achados tratados pelos papéis adequados e teste que impede
imports entre internos dos contextos. A ausência de scanner ou de teste deve aparecer como
pendência, não como ausência de risco.

### F2.2: implementar revisão FSRS e cartões de nota

Papéis: `backend-developer`, depois `frontend-developer`. Dependência: F1.5 e F2.1.

Implemente `review` com os alvos do ADR 0014: parte de exercício e nota autoavaliada. Preserve a
unicidade do cartão por parte e os dados necessários à retomada. Use um agendador FSRS validado
com versão registrada, sem reinventar a fórmula. A criação de cartão de destaque será manual.

Conclua com fila por vencimento, avaliações e novo agendamento, retomada e teste determinístico
com relógio controlado. Verifique que o cartão de nota sobrevive à remoção de sua origem.

### F2.3: implementar catálogo e metadados da Biblioteca

Papéis: `backend-developer`, depois `frontend-developer`. Dependência: F1.1, F1.3 e F2.1.

Implemente `library` dentro das camadas existentes. Entregue materiais, coleções, licença por
edição, habilidades, errata, status e selos derivados. Acrescente autoria `*.mat.md`, validação
cruzada e os ports de armazenamento e índice. Preserve a separação entre índices público e privado.

Para fontes externas, guarde link e metadados. A consulta externa é uma ação explícita. O
[spike arXiv](../../09-pesquisa/spike-arxiv-cors.md) exige preenchimento manual na web até nova
medição favorável. No desktop, implemente consulta restrita por comando Tauri. Avalie Crossref
separadamente. Consulte os pontos de produto ainda abertos antes de expor novos formatos ou exportações.

Conclua criando e buscando materiais offline, conferindo licença por edição e recusando
arquivo de terceiros no catálogo público. Teste item corrigido, retirado e sem classificação.
Uma importação não pode preservar um selo de motor forjado.

### F2.4: importar arquivos locais e entregar o leitor

Papéis: `backend-developer` para políticas; `frontend-developer` para adapters e leitor.
Dependência: F2.1 e F2.3.

Implemente `library.check_file`, `AssetStore` por hash e `DocumentReader` com PDF.js em Worker.
Binários não passam pelo JSON do `dispatch`. Aplique detecção por conteúdo, limites, proteção
contra links ativos, isolamento e controles de recursos descritos no ADR 0014 e na T7.
EPUB e HTML dependem de decisão de produto; não os acrescente por extensão do leitor PDF.

Conclua com importação local, busca de texto, retomada de página, renderização limitada às
páginas necessárias e funcionamento offline. Teste MIME falso, arquivo acima dos limites,
falta de espaço e documento malformado. Confira remoção de metadados na exportação e os hashes
da cópia local e da cópia exportada, mantendo os destaques ligados ao arquivo correto.

### F2.5: entregar destaques, notas e cartão manual

Papéis: `backend-developer` e `frontend-developer`. Dependência: F2.2 e F2.4.

Implemente seletores e reancoragem do ADR 0014, nota sanitizada e criação manual de cartão FSRS.
O destaque mantém identidade própria. Troca de arquivo exige confirmação de reancoragem quando
o hash muda. Destaque órfão e cartão sem origem permanecem acessíveis.

Conclua com testes de hash igual, edição diferente, citação ambígua, trecho não encontrado,
exclusão de material e retomada do cartão. Confira o fluxo por teclado e a privacidade herdada.

### F2.6: compilar e executar roteiros de leitura

Papéis: `backend-developer`, `frontend-developer` e `content-author`, em entregas distintas.
Dependência: F2.3, F1.4 e F1.6. Leitura de arquivo depende também de F2.4.

Implemente `*.guide.md`, etapas estáveis, pré-requisitos, perguntas, checagens com `Part`,
exercícios e visualizações. Reutilize as APIs públicas de conteúdo e correção. Valide as
referências cruzadas no build, conforme ADR 0014. Conteúdo autoral segue licença e taxonomia.

Conclua um roteiro que liga ler, abrir laboratório e praticar. Teste referência ausente,
checagem incorreta e alteração de revisão. Leitura concluída não aumenta domínio por habilidade.
Selos de motor cobrem as checagens, nunca a veracidade de todo o texto.

### F2.7: implementar caderno de erros, plano e progresso

Papéis: `backend-developer`, depois `frontend-developer`. Dependência: F1.5 e F2.2.

Implemente `progress`, caderno de erros e plano com data de prova. Use tentativas reais e
habilidades; não conte leitura ou conclusão de roteiro como domínio. Atualize o Início A para
usar os vencimentos e metas agora disponíveis. Defina com produto qualquer fórmula ou política
de plano ainda não especificada antes de implementá-la.

Conclua com plano salvo, tarefas e erros rastreáveis às tentativas e retomada offline. Teste
mudança de data, ausência de dados e revisão de exercício sem reescrever tentativas antigas.

### F2.8: entregar importador e tutor com IA opcional

Papel: `ai-engineer`, com UI encaminhada ao `frontend-developer`. Dependência: F1 completa e F2.1.

Implemente o port `LlmProvider`, fakes e adapters local e externos previstos na estratégia de IA.
Separe o importador de listas do leitor da Biblioteca. Meça OCR e extração nas listas locais
autorizadas; valide saída pelo JSON Schema, confira respostas pelo motor e exija revisão humana.
No tutor, entregue dicas graduais fundamentadas nos passos do motor.

Conclua com conjunto público de avaliação autoral, métricas e limites de aprovação definidos
antes da avaliação. Teste prompt injection, fonte com `aiUse: forbidden`, recusa de envio e
indisponibilidade do provedor. O fluxo básico do app continua offline e sem LLM. Material
privado de avaliação não entra na CI pública nem é enviado automaticamente.

### F2.9: expor o motor por MCP

Papel: `ai-engineer`, com revisão de fronteira por `software-architect`.
Dependência: comandos estáveis da F1 e revisão F2.1.

Implemente `vertice-mcp` como driver do mesmo motor. Exponha cálculo e busca no banco público,
sem operações de escrita ou acesso implícito ao acervo privado.

Conclua com um cliente MCP chamando o motor e com erros de contrato e tentativas de acesso
fora do escopo testadas. Compare o resultado com o `dispatch` direto.

### F2.10: acrescentar as disciplinas em fatias completas

Papéis: `backend-developer`, `frontend-developer`, `content-author` e `qa-engineer`.
Dependência: F1 e registro de visualizações. Algoritmos e SQL executáveis dependem de F2.11.

Execute cada subunidade separadamente, conforme [módulos](../../02-produto/modulos.md):

| ID | Entrega por disciplina | Verificação específica |
| --- | --- | --- |
| F2.10a | Física I: cinemática, projéteis e forças | Relações entre posição, velocidade e aceleração; decomposição vetorial |
| F2.10b | Métodos numéricos: bisseção, Newton, Euler e RK4 | Convergência, parada, erro e hipótese de validade mostrados |
| F2.10c | Estruturas I: equilíbrio, apoios e esforços | Gabaritos autorais conferidos e convenções de sinais explícitas |
| F2.10d | Algoritmos: execução passo a passo e estruturas | Estados intermediários e resultado em casos vazios e extremos |
| F2.10e | SQL: schema, dados, consulta e resultado | Correção por conjunto de resultados e isolamento entre exercícios |

Cada subunidade entrega regra no núcleo quando aplicável, contrato, UI, conteúdo e teste E2E.
O contrato e o escopo específico vêm antes da implementação; não trate essa tabela como PRD
detalhado de disciplinas ainda sem histórias. Encaminhe a definição ao papel de produto.

### F2.11: executar código de estudante com isolamento

Papéis: `software-architect` para decisões; implementadores e revisão de `security-researcher`.
Dependência: F2.1 e tipos `code` e `sql` contratados.

Implemente JS, Python e SQL nas opções de isolamento da estratégia de IA. Limite tempo,
memória e capacidades. Teste cancelamento e execução infinita sem travar a UI. Persistência,
rede e acesso a dados do app não são capacidades implícitas do código do estudante.

Para Java, faça o spike previsto no roadmap e registre a decisão. Enquanto a execução não
estiver validada, exercícios permanecem autoavaliados, com rubrica claramente identificada.
Um judge remoto depende da F3 e não pode se tornar requisito da F2 offline.

### F2.12: verificar e delimitar a fase

Papel: `qa-engineer`. Dependência: unidades F2 entregues e achados críticos resolvidos.

Verifique o ciclo estudar, visualizar, praticar e revisar na web e no desktop. Confira backup
e restauração da Biblioteca, arquivos locais, destaques e cartões. Execute as avaliações de IA
e os cenários de segurança com fixtures públicas apropriadas. Registre plataformas ausentes.

Atualize cada item do roadmap individualmente. Não declare F2 completa com apenas Biblioteca
ou apenas tutor. Busca semântica, EDOs, multivariável e itens herdados entram somente com escopo
explicitamente definido, sem serem absorvidos por uma tarefa genérica de "terminar F2".

## Encerrar a fase

Confira os critérios do PRD da Biblioteca e as entregas explícitas do roadmap, incluindo novas
disciplinas. Itens ainda abertos em produto ou segurança mantêm a entrega correspondente pendente.
Antes de [F3](f3-contas-e-nuvem.md), revise as decisões propostas de servidor e sincronização.
