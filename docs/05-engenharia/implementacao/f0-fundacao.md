# F0: preparar a fundação da Vértice

Comece pelas [regras para agentes](regras-para-agentes.md). Execute a unidade solicitada e pare
nos limites dela. O [roadmap](../../02-produto/roadmap.md#f0--fundação-vértice) mantém o estado
da fase. Esta fase prepara a plataforma e publica o Laboratório existente.

## Conferir as entradas

Leia o [ADR 0006](../../03-arquitetura/adr/0006-plataforma-vertice.md), a
[arquitetura](../../03-arquitetura/visao-geral.md) e o [guia de design](design-aprovado.md).
Preserve os comportamentos atuais de Cálculo e VGA. Não implemente o player de exercícios,
contas, servidor ou Biblioteca nesta fase.

## Executar na ordem

### F0.1: registrar a linha de base e localizar a referência visual

Papel: `qa-engineer`. Dependência: nenhuma.

Execute os checks atuais do motor e do front. Abra o Laboratório e confira limites, derivadas,
integrais, vetores, matrizes e sistemas. Registre falhas preexistentes e ambiente utilizado.
Confira a disponibilidade do protótipo conforme o guia de design. Se a fonte continuar ausente,
registre essa limitação separadamente dos resultados funcionais.

Conclua com uma linha de base reproduzível e a situação da fonte visual registrada. Recuperar
o branch em outra máquina é tarefa do mantenedor; não bloqueie a coleta dos demais resultados.

### F0.2: executar o renome técnico isolado

Papel: `backend-developer`, com entregas de front encaminhadas ao `frontend-developer`.
Dependência: F0.1 com falhas conhecidas registradas.

Siga o mapeamento do ADR 0006 nos manifests, imports, scripts, testes, bindings e configuração
Tauri. Atualize os comandos de exclusão do pacote desktop nos documentos operacionais. Preserve
os ADRs históricos. Não mude regras matemáticas nem misture atualização de dependências.
Confirme escopo de pacote e identidade pública com o mantenedor antes de publicação externa.
Renomear o repositório no GitHub é uma operação externa separada da alteração local.

Conclua com fmt, clippy, testes, WASM e checks do front verdes, além de build desktop no ambiente
disponível. Registre alvos desktop não testados. O diff deve permitir distinguir o renome das
correções estritamente necessárias aos manifests.

### F0.3: atualizar dependências com regressão controlada

Papéis: `backend-developer` e `frontend-developer`, em entregas separadas.
Dependência: F0.2.

Confira versões e migrações nas fontes oficiais no momento da execução. Atualize por grupo de
dependências compatíveis, mantendo lockfiles reproduzíveis. Não use números de versões de uma
pesquisa antiga como recomendação atual. Valide cada grupo antes de começar o próximo.

Conclua com os checks das camadas alteradas, build WASM, build estático e os cenários de
Laboratório da F0.1 sem regressões. Documente mudanças obrigatórias e vulnerabilidades restantes.

### F0.4: organizar navegação e identidade visual

Papel: `frontend-developer`. Dependência: F0.2 e F0.3.

Implemente shell e navegação por módulos. Mova o Laboratório para `/lab/...` e preserve as URLs
antigas por redirecionamento compatível com o build estático, incluindo parâmetros de estado.
Use o design aprovado. Mantenha módulos futuros identificados como planejados, sem links para
páginas inexistentes. Defina os links entre Site e Laboratório antes de ocupar a rota inicial.

Conclua navegando por links diretos, atualização de página, voltar e avançar, tema e teclado.
Teste as URLs antigas no build estático servido localmente, não apenas no servidor de desenvolvimento.

### F0.5: implementar o Site D

Papel: `frontend-developer`. Dependência: F0.4 e fonte visual recuperada para comparação final.

Siga [design-aprovado.md](design-aprovado.md), ADR 0013 e os componentes de produção. Implemente
os três botões de função do herói e o movimento nas próprias seções. Use o motor para as
demonstrações matemáticas disponíveis. Distinga funcionalidades entregues das planejadas.

Conclua com comparação visual registrada, layout responsivo, teclado, links para o Laboratório,
tema e redução de movimento verificados. O build final não contém a infraestrutura de protótipo.
O teste de cinco segundos com estudantes permanece pendente até haver participação humana real.

### F0.6: estabelecer CI e revisão de segurança da fundação

Papel de implementação: `devops-engineer`. Revisão: `security-researcher`.
Dependência: F0.2 e F0.3. Pode ocorrer enquanto F0.4 e F0.5 são implementadas.

Implemente os jobs da F0 descritos em [CI/CD](../../07-devops/ci-cd.md). Fixe Actions por SHA,
use permissões mínimas e isole segredos de forks. Publique ou confira `SECURITY.md` e revise
as ameaças T1–T3. E2E de exercícios e compilação de conteúdo entram na F1; o workflow não deve
chamar comandos que ainda não existem.

Conclua com execução real dos checks em PR, build WASM e estático, scanners e guard de conteúdo
privado. O relatório de segurança identifica achados e correções separadas. Registre falhas
preexistentes e serviços que exigem configuração do mantenedor.

### F0.7: preparar publicação e verificar a fase

Papel: `devops-engineer`; verificação por `qa-engineer`. Dependência: F0.4–F0.6.

Configure preview e produção estática conforme [ambientes e deploy](../../07-devops/ambientes-e-deploy.md).
Prepare cabeçalhos, URLs, build e rollback antes da publicação autorizada. Confira limites e
custos atuais nas fontes oficiais. Não declare um deploy feito só porque existe um workflow.

Conclua com URL de preview verificada, publicação de produção autorizada e testada, navegação
estática funcional e checks verdes. Verifique o Laboratório na web e registre o smoke desktop
efetuado. Releases Tauri multi-plataforma seguem a política de release; não invente assinatura
ou instaladores para alvos não executados.

## Encerrar a fase

O agente `qa-engineer` confere o checklist F0 do roadmap contra as evidências. Registre qualquer
publicação, verificação visual ou alvo desktop pendente. Só então indique [F1](f1-exercicios.md).
Uma pendência externa não autoriza declarar F0 concluída nem impede trabalho independente já
autorizado e compatível com seus contratos.
