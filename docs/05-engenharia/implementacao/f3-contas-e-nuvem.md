# F3: acrescentar contas e nuvem opcional

Comece pelas [regras para agentes](regras-para-agentes.md). F3 adiciona serviços à plataforma
local existente. O [roadmap](../../02-produto/roadmap.md#f3--contas-e-nuvem) mantém o estado
da fase. Uma conta não passa a ser requisito para usar Laboratório, exercícios ou dados locais.

## Conferir as entradas

Leia o [ADR 0011 proposto](../../03-arquitetura/adr/0011-backend-axum-postgres.md), o
[ADR 0014](../../03-arquitetura/adr/0014-biblioteca.md), o
[modelo de dados](../../03-arquitetura/modelo-de-dados.md), o
[modelo de ameaças](../../06-seguranca/modelo-de-ameacas.md) e os documentos de
[ambientes](../../07-devops/ambientes-e-deploy.md) e
[infra](../../07-devops/infra-free-tier.md).

O servidor Axum e Postgres ainda exigem revisão da proposta. Sync e autenticação têm escolhas
em aberto. Não escolha um fornecedor por aparecer na pesquisa. Reconfirme limites gratuitos,
compatibilidade e custos nas fontes oficiais no momento da execução.

## Executar na ordem

### F3.1: aceitar os contratos de servidor, identidade e sync

Papel: `software-architect`, com requisitos de `product-manager` e revisão de `security-researcher`.
Dependência: contratos locais da F1 e dados da F2 estabilizados.

Revise ADR 0011 e registre a decisão. Especifique autenticação, autorização, sync, resolução
de conflitos, exclusão e migração. Defina os novos ADRs necessários. Inclua as regras da
Biblioteca e a política de listas compartilhadas por link, sem antecipar compartilhamento
de arquivos da Biblioteca em grupos da F4.

Conclua com decisões aceitas e cenários de isolamento por usuário, reconexão e conflito
verificáveis. O agente de backend não implementa uma opção de sync enquanto ela for proposta.

### F3.2: criar o servidor usando os mesmos casos de uso

Papel: `backend-developer`. Dependência: F3.1.

Implemente o driver `vx-server`, contrato HTTP, configuração e migrações Postgres. Reutilize
os casos de uso do núcleo. Mantenha regras matemáticas e de licença fora dos handlers. Exponha
OpenAPI e erros estruturados conforme o contrato aceito.

Conclua com paridade de correção para fixtures executadas em WASM e servidor, testes de
migrações e funcionamento dos adapters locais preservado. Documente o comando local real do
servidor e seus serviços de apoio, sem depender de credenciais de produção.

### F3.3: autenticar e isolar usuários

Papel: `backend-developer`; revisão de `security-researcher`. Dependência: F3.1 e F3.2.

Implemente a decisão de identidade e sessões, autorização nos casos de uso e RLS no Postgres.
Cubra contas, listas, tentativas, notas e referências privadas. Preserve acesso sem conta aos
recursos locais. Segredos ficam em configuração apropriada, sem aparecer no bundle ou nos logs.

Conclua com dois usuários reais de teste incapazes de acessar dados um do outro, inclusive
alterando IDs e chamando diretamente a API. Confira expiração, revogação e exclusão conforme
o contrato. A revisão aplica o objetivo ASVS 5.0 nível 2 previsto no projeto, com evidências.

### F3.4: sincronizar dados e listas por link

Papéis: `backend-developer` e `frontend-developer`. Dependência: F3.3 e ADR de sync aceito.

Implemente os adapters de sync sem substituir os repositórios usados offline. Preserve UUID,
revisão e tentativas históricas. Sincronize metadados, destaques, notas e progresso dos materiais
conforme consentimento e política definida. O arquivo de terceiros permanece no dispositivo.
Em outra máquina, permita re-associação local pelo hash.

Implemente listas por link somente com a política de visibilidade, autoria e autorização da
F3.1. Isso não habilita automaticamente comunidade ou upload compartilhado da Biblioteca.

Conclua com dois dispositivos: alterações offline concorrentes, reconexão, retomada de sync,
conflito, revogação de acesso e exclusão. Demonstre que metadados podem sincronizar sem upload
do arquivo de terceiros. Teste repetição de requisições sem duplicar tentativas ou cartões.

### F3.5: guardar arquivos autorais em nuvem privada

Papel: `backend-developer`, com UI por `frontend-developer` e revisão de `security-researcher`.
Dependência: F3.3 e regras locais da Biblioteca.

Implemente upload por item com opt-in, cota, URLs pré-assinadas, quarentena, varredura e liberação
conforme T7. O servidor detecta tipo, refaz hash e aplica `can_upload`. Só arquivos autorais
podem subir. Remova metadados da cópia de saída e preserve a ligação entre hashes local e remoto.

Conclua recusando arquivo de terceiros mesmo com requisição adulterada. Teste arquivo não
liberado, cota excedida, URL expirada, acesso de outro usuário e exclusão completa. Mostre que
armazenar arquivo privado autoral não o publica nem o compartilha com grupos.

### F3.6: operar infraestrutura, backup e recuperação

Papel: `devops-engineer`. Dependência: F3.2; exposição externa depende também de F3.3 e revisão.

Implemente IaC, imagens multi-arch, execução sem root, rede, observabilidade e backups conforme
os documentos de DevOps. Revise os limites atuais de cada serviço e registre o orçamento.
Prepare rollback e restauração antes do deploy autorizado. Use ambientes descartáveis para testes.

Conclua com restauração de backup executada, migração compatível e rollback demonstrado,
serviço acessível pela entrada prevista e segredos ausentes dos artefatos. Registre custos,
alertas e responsáveis. Não habilite alertas para terceiros sem autorização de comunicação.

### F3.7: avaliar o judge remoto, se necessário

Papéis: `software-architect`, `backend-developer`, `devops-engineer` e `security-researcher`.
Dependência: evidência da F2.11 e F3.3.

Só implemente se a avaliação de execução local mostrar necessidade aceita. Isole o judge da
API, das credenciais e dos dados de usuários. Defina quotas, timeout e rede antes de executar
código não confiável. A decisão de não implementar também deve citar a evidência que a sustenta.

Conclua com cenários de abuso e isolamento testados ou com decisão registrada de manter o judge
fora da entrega. O judge não transforma respostas autoavaliadas em corrigidas sem testes válidos.

### F3.8: verificar contas, nuvem e regressão offline

Papel: `qa-engineer`, com revisão de segurança das fronteiras expostas.
Dependência: F3.1–F3.6; situação da F3.7 registrada.

Execute a matriz conta ausente, usuário A, usuário B, dispositivo offline e dispositivo
reconectado. Confira backup, exclusão, listas por link e Biblioteca privada. Garanta que os
fluxos locais da F1 e da F2 continuam funcionando quando o servidor está indisponível.

## Encerrar a fase

Marque o roadmap com evidências do ambiente efetivamente publicado e testado. Segurança ou
restauração pendente impede declarar a nuvem pronta. A [F4](f4-comunidade.md) exige decisões
adicionais de comunidade, direitos, moderação e compartilhamento.
