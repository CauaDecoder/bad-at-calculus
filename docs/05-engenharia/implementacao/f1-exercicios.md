# F1: entregar exercícios offline

Comece pelas [regras para agentes](regras-para-agentes.md). A entrega é resolver listas com
correção imediata, criar conteúdo e retomá-lo offline. O [roadmap](../../02-produto/roadmap.md#f1--exercícios-offline-mvp)
mantém o estado da fase.

## Conferir as entradas

Leia o [PRD de exercícios](../../02-produto/prd-exercicios.md), o
[formato canônico](../../02-produto/formato-de-exercicios.md) e os ADRs
[0008](../../03-arquitetura/adr/0008-formato-canonico-de-exercicio.md),
[0009](../../03-arquitetura/adr/0009-correcao-no-motor-rust.md),
[0010](../../03-arquitetura/adr/0010-local-first-na-fase-1.md),
[0014](../../03-arquitetura/adr/0014-biblioteca.md) e
[0015](../../03-arquitetura/adr/0015-evidencia-de-verificacao.md).

F0 deve fornecer nomes, shell e build estáveis. A Biblioteca nesta fase é somente `references[]`
e a seção "Para estudar". Leitor, upload de PDF, OCR, FSRS, tutor, contas e sincronização ficam
fora da F1. O passo a passo vem do motor e de dicas autorais.

## Executar na ordem

### F1.1: implementar o contrato de conteúdo e referências

Papel: `backend-developer`; revisão de contrato por `software-architect`. Dependência: F0.2.

Implemente os tipos de exercício, partes, listas, packs e os tipos compartilhados exigidos pela
F1. Acrescente `references[]` conforme §11 do formato, com erros por campo para links proibidos
e metadados insuficientes. Preserve regras de licença e visibilidade. Não implemente entidades
completas da Biblioteca nem antecipe tipos compartilhados sem consumidor nesta fase.

Gere JSON Schema e tipos TypeScript a partir dos tipos Rust nas camadas permitidas. Resolva
qualquer divergência de propriedade de `Part` e `AnswerSpec` com o arquiteto antes de fixar a
API pública. O conteúdo deve consumir a API pública do corretor, sem importar seus internos.

Conclua com exemplos mínimos válidos, testes de campos inválidos, conteúdo privado impedido
de publicação e geração reproduzível dos contratos. Documente os comandos reais de geração.

O [ADR 0016](../../03-arquitetura/adr/0016-contrato-de-conteudo-f1.md) resolve a propriedade
dos tipos: `content::Part` consome `assessment::AnswerSpec` pela fachada pública Rust. Os DTOs
com `serde`, `schemars` e `ts-rs` ficam em `vx-api`.

Geração dos contratos da F1.1 (não compila autoria nem verifica gabaritos):

```bash
cargo run -p vx-api --example generate_content_contracts
cargo run -p vx-api --example generate_content_contracts -- --check
```

Equivalentes no workspace: `pnpm contracts:generate` e `pnpm contracts:check`. O primeiro
comando atualiza `contracts/*.schema.json` e
`apps/web/src/lib/domain/generated/content.ts`; o segundo falha quando o conteúdo gerado
difere dos arquivos versionados ou quando algum arquivo está ausente. Os destinos são
relativos ao manifesto de `vx-api`, independentemente do diretório de execução. A integração
desse check com a CI de conteúdo pertence à F1.9.

Fixtures autorais mínimas ficam em `crates/vx-api/tests/fixtures/`: `exercise.json`, `list.json`
e `manifest.json`. Não são o banco público nem substituem o aceite com pack privado da F1.10.
Validação estrutural não interpreta expressões e não confirma respostas matemáticas; os
algoritmos pertencem à F1.2. `verify` e `evidence` são rejeitados até a F1.4; `material` e
`params` de exercício só aceitam ausência ou `null` neste recorte.

### F1.2: implementar a correção de respostas

Papel: `backend-developer`. Dependência: F1.1.

Implemente os tipos da F1 do ADR 0009 por grupos verificáveis: numérico e expressão; equação e
coleções; escolhas, associação e autoavaliação. Cada grupo inclui domínio, caso de uso, DTO,
comando `assessment.check` e contrato JSON. Registre o comando também no front.

Conclua com formas equivalentes aceitas, respostas incorretas rejeitadas, sintaxe separada de
erro matemático, amostragem determinística e domínio restrito. Teste `x/x` versus `1`,
`sqrt(x^2)` versus `x`, pontos inválidos e amostragem insuficiente. Autoavaliação não vira
correção automática. Verifique contrato e paridade do motor nos alvos executados.

O [relatório da F1.2](f1-2-correcao.md) registra o primeiro grupo e a retomada com os nove
tipos, `form`, `assessment.check`, contratos gerados e fixtures compartilhadas de paridade.
O contrato textual, os limites da amostragem e a checagem do IPC Tauri com `MockRuntime`
estão documentados no relatório; não representam aceite do player ou da F1 inteira.

### F1.3: compilar autoria e construir o catálogo

Papel: `backend-developer`. Dependência: F1.1 e F1.2.

Implemente `*.ex.md` para JSON canônico, taxonomia, listas, índice e erros por campo. Inclua o
driver de build previsto nos ADRs, sem criar um contexto de domínio novo para um comando de CLI.
Valide licença, revisão, habilidades e referências. Conferir a resposta esperada contra si
mesma valida formato; isso não gera selo de motor.

Conclua compilando fixtures autorais válidas e rejeitando estrutura, licença e referências
inválidas. A geração repetida deve produzir o mesmo canônico. Registre como invocar o build
real no projeto antes de usá-lo na CI.

### F1.4: verificar gabaritos e gerar evidência

Papel: `backend-developer`. Dependência: F1.2 e F1.3.

Implemente `VerifyExpectedAnswers`, o port `EngineCommandRunner`, `verify`, `evidence` e a
regra de selo do ADR 0015. O runner fica na camada externa e executa somente os comandos
permitidos para verificação. Rejeite evidência de motor escrita na autoria. Recalcule evidência
importada, vincule-a ao hash atual e preserve escopo, versão e parâmetros de amostragem.

Antes de fixar a serialização, detalhe com o arquiteto a canonicalização do hash, a tolerância
numérica registrada e quais campos variáveis, como data e duração, não determinam divergência
do lock. O ADR exige reprodutibilidade; não trate duração variável como regressão matemática.

Conclua com gabarito correto confirmado, errado marcado como `mismatch`, caso não decidível
como `indeterminate` e evidência antiga invalidada. Teste resultado JSON Pointer inválido,
comando recusado, selo forjado e evidência só humana. Gere `content/evidence.lock.json` com
regeneração e comparação demonstradas. Não apresente amostragem como prova formal.

### F1.5: armazenar e transportar os dados locais

Papéis: `backend-developer` e `frontend-developer`, com contratos separados.
Dependência: F1.1 e F1.3. A importação de evidência exige F1.4.

Implemente os ports de repositórios, IndexedDB na web e SQLite no desktop. Importe e exporte
pack, JSON, Markdown e gabarito separado; CSV cobre objetivas. Preserve identidade e revisão,
trate conflitos e faça backup restaurável. Aplique os limites de ZIP e sanitização da T1.

Conclua com round-trip de pack, retomada após reiniciar o app, tratamento de duplicatas,
caminhos maliciosos recusados e material privado ausente de exportações públicas. Teste os
dois adapters com o mesmo contrato. Sem ambiente desktop, registre a verificação pendente.

### F1.6: registrar visualizações reutilizáveis

Papel: `frontend-developer`. Dependência: F1.1 e F0.4.

Implemente o registro `viz.id`, versão, parâmetros tipados e URL com estado. Reutilize o
Laboratório existente. Valide os parâmetros de cada visualização antes de navegar.

Conclua abrindo função, ponto e demais parâmetros do exercício em uma nova sessão do
Laboratório. Teste parâmetros inválidos, IDs desconhecidos e links diretos no build estático.

### F1.7: entregar o player D e o Início A

Papel: `frontend-developer`. Dependência: F1.2, F1.4–F1.6 e contrato compilado da F1.3.

Siga o [guia de design](design-aprovado.md). Implemente exercícios e listas, progresso por parte,
entrada matemática, dicas, solução e erro comum. Acrescente "Para estudar" com referências e
selos qualificados pela evidência. Use sanitização e copy em PT-BR. Faça o spike de entrada
MathLive versus `ExpressionInput` antes de decidir substituir o componente existente.

No Início A, mostre continuação e tarefas baseadas nos dados locais. Não antecipe FSRS, plano
até a prova ou indicadores de domínio. Encaminhe decisões de pontuação e liberação de solução
aos pontos abertos do PRD antes de implementá-las.

Conclua o fluxo por teclado, sintaxe que não conta tentativa, acerto e erro, barra contínua,
retomada, link ao laboratório e comparação visual com A e D. Comprove uso offline após o
primeiro carregamento e recarga sem rede. Nomeie e implemente o mecanismo necessário a esse
comportamento; build estático sozinho não comprova funcionamento offline.

### F1.8: entregar a autoria com preview

Papel: `frontend-developer`. Dependência: F1.3–F1.5 e componentes da F1.7.

Implemente criação e edição com partes, metadados, referências, gabarito e preview pelo mesmo
compilador. Mostre erros por campo e o pedido de verificação. Não permita forjar selo digitando
`evidence`. Preserve o vínculo de tentativas com a revisão vista pelo estudante.

Conclua criando um exercício autoral de duas partes, verificando, exportando e reimportando.
Confira as metas de tempo do PRD com medição real, sem as declarar atendidas por estimativa.

### F1.9: semear conteúdo e integrar os checks de conteúdo

Papéis: `content-author` para conteúdo; `devops-engineer` para CI. Dependência: F1.3 e F1.4.

Crie os primeiros exercícios autorais de Cálculo e VGA, com respostas conferidas e licenças.
Implemente `content.yml` com schema, taxonomia, licença, referências e comparação do lock.
Schema e tipos gerados desatualizados também devem falhar. Fixtures públicas não contêm PDFs
ou texto de listas privadas.

Conclua com execução na CI que aprova conteúdo válido e reprova gabarito divergente, lock
desatualizado e mudança de veredito sem mudança de conteúdo.

### F1.10: verificar o MVP completo

Papel: `qa-engineer`; revisão de T1 por `security-researcher`. Dependência: F1.1–F1.9.

Execute os fluxos públicos E2E de resolver, pedir dica, abrir laboratório, criar, importar,
retomar offline e restaurar backup. Meça os limites de latência e importação do PRD.
Localmente, valide a Lista 04 inteira em pack privado contra o gabarito, sem publicar o material.

Conclua com os checks exigidos verdes e relatório do que rodou na web e no desktop. Se o pack
privado estiver ausente, registre que o aceite específico da Lista 04 não foi executado.
Fixtures autorais não substituem silenciosamente esse aceite.

## Encerrar a fase

Marque F1 pronta somente depois dos critérios acima e do aceite do roadmap. Registre eventuais
decisões de produto ainda abertas. A próxima fase é [F2](f2-estudo-local.md); ela usa os mesmos
contratos, tentativas e dados locais já entregues.
