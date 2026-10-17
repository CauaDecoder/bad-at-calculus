# F4: abrir a comunidade com moderação

Comece pelas [regras para agentes](regras-para-agentes.md). F4 permite grupos, turmas e
publicação de conteúdo autoral. O [roadmap](../../02-produto/roadmap.md#f4--comunidade) mantém
o estado da fase. A existência de contas na F3 não habilita compartilhamento automaticamente.

## Conferir as entradas

Exija F3 verificada, revisão de segurança e capacidade operacional para moderar. Leia o
[PRD da Biblioteca](../../02-produto/prd-biblioteca.md), o
[ADR 0014](../../03-arquitetura/adr/0014-biblioteca.md) e a
[T7](../../06-seguranca/modelo-de-ameacas.md#t7--upload-e-leitura-de-arquivos-f2f4).

Compartilhamento de arquivos é apenas autoral. Material de terceiros entra por link e metadados,
qualquer que seja sua licença. A declaração de autoria e a escolha de licença não substituem
denúncia, moderação e revisão. Não transforme esta fase em LMS de notas ou edição em tempo real.

## Executar na ordem

### F4.1: fechar produto, direitos e operação

Papel: `product-manager`, com decisões do mantenedor e avaliação jurídica externa.
Dependência: F3 e responsáveis pela operação identificados.

Defina processo de denúncia e remoção, termos, licença, prazos, política durante análise,
reincidência, retenção e responsáveis pela fila. Resolva os pontos abertos do PRD. Registre a
avaliação jurídica exigida antes de abrir compartilhamento, sem tratar texto gerado por agente
como parecer jurídico. Especifique critérios de reputação e comentários antes de implementá-los.

Conclua com políticas e responsáveis definidos, critérios de aceite e avaliação registrada.
Sem esses requisitos, outras partes podem ser preparadas localmente, mas a abertura fica pendente.

### F4.2: contratar grupos, permissões e publicação

Papel: `software-architect`; revisão de `security-researcher`. Dependência: F4.1.

Crie o ADR previsto para visibilidade de grupo. Defina membros, papéis, convites, acesso a
revisões, publicação, retirada e auditoria. Preserve as regras de arquivos da Biblioteca.
Especifique migração e autorização dos dados novos, com critérios para testar cada papel.

Conclua com contratos aceitos e matriz de permissões para membro, não membro, autor e moderador.
Um campo genérico `public: true` não substitui essas regras.

### F4.3: implementar grupos e publicação autoral

Papéis: `backend-developer` e `frontend-developer`. Dependência: F4.2.

Implemente grupos, turmas e listas do professor conforme os contratos. Publique packs e
materiais autorais com revisão, licença e atribuição. Use os mesmos compiladores, verificadores
e políticas do núcleo. Dados privados de tentativa e cartões não acompanham uma publicação.

Conclua com usuário entrando e saindo de um grupo, permissão revogada e publicação recusada
para conteúdo incompatível. Confira que um não membro não recupera o arquivo por URL direta.
Compartilhar não altera a licença nem a visibilidade de outros materiais privados do autor.

### F4.4: implementar denúncia, remoção e auditoria

Papéis: `backend-developer`, `frontend-developer` e `devops-engineer`.
Dependência: F4.1–F4.3. Abertura depende da revisão F4.7.

Implemente a fila operacional, restrições de acesso, registro de decisão e retirada em todas
as revisões afetadas. Aplique a trilha de auditoria e retenção definidas. Remova assets e caches
conforme a política. Mantenha a lápide e a continuidade de notas e cartões privados conforme
o ADR 0014. A denylist de hashes auxilia a moderação e não garante impedir republicação.

Conclua com uma denúncia percorrendo o processo completo em ambiente de teste, com responsável,
decisão, retirada e conferência do acesso após a remoção. Não abra upload compartilhado antes
de demonstrar esse fluxo.

### F4.5: acrescentar comentários e reputação

Papéis: `backend-developer` e `frontend-developer`. Dependência: critérios da F4.1 e moderação F4.4.

Implemente os comportamentos definidos por produto, com proteção contra abuso, edição e
remoção rastreáveis. Sanitização e autorização também se aplicam a comentários. Não confunda
reputação do autor com selo matemático ou com domínio do estudante.

Conclua com cenários de abuso e permissões testados, além da aplicação das políticas de moderação.

### F4.6: implementar interoperabilidade com LMS

Papéis: `software-architect` para contrato, depois `backend-developer` e `qa-engineer`.
Dependência: conteúdo estável e escopo de integração definido por produto.

Implemente exportação QTI 3.0 e integração LTI previstas no roadmap em entregas separadas.
Confira as especificações oficiais atuais, declare o subconjunto de tipos suportado e relate
perdas de informação. Não apresente exportação de arquivo como integração LTI concluída.

Conclua com validação do formato e uso em um LMS de teste, incluindo tipos não suportados,
permissões e os dados pessoais enviados na integração. Registre o ambiente e suas limitações.

### F4.7: verificar e abrir a comunidade

Papel: `qa-engineer`, com `security-researcher`, operação e mantenedor.
Dependência: unidades anteriores entregues e políticas vigentes.

Execute a matriz de permissões, publicação, denúncia, retirada e exportação. Revise isolamento
entre grupos, dados privados e arquivos. Confira capacidade de moderação e rollback da abertura.
Somente então prepare a ativação externa autorizada pelo mantenedor.

## Encerrar a fase

Atualize o roadmap com o que foi entregue, o que foi aberto ao público e o que continua restrito.
Registre riscos aceitos e responsáveis pela operação. Uma demonstração local de grupos não
equivale à comunidade publicada e operável.
