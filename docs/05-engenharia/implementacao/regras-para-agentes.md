# Executar uma unidade de implementação

Leia o [AGENTS.md](../../../AGENTS.md) e este arquivo antes de executar qualquer unidade F0–F4.
Se o usuário restringir o trabalho, aplique a restrição à unidade escolhida.

## Preparar a execução

1. Leia o guia da fase e a unidade solicitada. Confira suas dependências no código e no roadmap.
   Consulte o [registro de execução](progresso.md) para localizar evidências e pendências anteriores.
2. Declare o papel do AGENTS.md que executa a unidade. Consulte a definição desse papel e suas
   leituras. Os arquivos em `.claude/agents/` são uma implementação dos mesmos papéis para Claude Code.
3. Examine `git status --short` e o diff relevante. Preserve alterações anteriores e arquivos
   não rastreados. O repositório pode conter trabalho do mantenedor e de outros agentes.
4. Execute os checks aplicáveis para obter uma linha de base. Separe falhas preexistentes das
   introduzidas pela mudança. Não apague testes ou relaxe regras para produzir resultado verde.
5. Implemente a menor entrega verificável da unidade. Se ela exceder um PR revisável, divida a
   execução em partes e registre quais critérios continuam pendentes.

Dependência ausente não autoriza implementar uma fase anterior inteira. Registre o bloqueio e
indique a unidade necessária. Divergência entre documentos deve ser resolvida antes de fixar um
contrato público. Não invente um comportamento de produto para preencher uma lacuna.

## Respeitar contratos e papéis

Use os PRDs para comportamento, os ADRs aceitos para decisões e o código para o estado real.
Estes guias definem a sequência. Um ADR proposto ou uma opção pesquisada não é decisão aceita.
Quando uma decisão nova for necessária, encaminhe ao `software-architect` e use o template de ADR.

Siga a receita de operação do motor no AGENTS.md. Preserve a separação entre domínio, aplicação,
adaptadores e UI. O renome técnico local da F0.2 usa `vx-*` e `@vertice/*`.
Os ADRs e relatórios históricos preservam os nomes antigos. Não misture o renome com uma funcionalidade.

Se uma unidade precisar de outro papel, entregue o contrato e a recomendação para esse papel,
conforme o AGENTS.md. Com coordenação explicitamente autorizada, distribua as entregas entre os
papéis correspondentes. Um revisor de segurança relata achados e não corrige código nesse papel.

## Aplicar as orientações visuais

Para qualquer mudança em tela, leia [design-aprovado.md](design-aprovado.md) antes de editar.
Início A, Exercício D e Site D são referências distintas. Telas sem protótipo seguem os tokens,
componentes e princípios aprovados. Não crie dados fictícios apresentados como histórico real.

Recursos de fases futuras aparecem como planejados ou ficam fora da navegação ativa. A presença
de um cartão no protótipo não antecipa FSRS, conta, nuvem ou IA para a F0 ou a F1.

## Validar a entrega

Use os comandos do workspace e a [estratégia de testes](../testes.md). O pacote desktop
excluído dos checks Rust é `vertice-desktop`. Confira `cargo metadata` quando houver dúvida.

Para mudanças Rust, execute:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings
cargo test --workspace --exclude vertice-desktop
```

Para mudanças no front, execute:

```bash
pnpm -C apps/web check
pnpm -C apps/web lint
pnpm -C apps/web test
```

Se mudar o motor consumido pelo navegador, reconstrua WASM antes da verificação na UI:

```bash
pnpm wasm:build
pnpm -C apps/web build
```

Os comandos `vertice content build` dos ADRs são entregas previstas da F1. Não presuma que já
existem. Depois de implementá-los, documente a invocação real no guia da F1 e nos scripts do projeto.

Toda unidade acrescenta os cenários específicos descritos em seu guia. Para uma entrega visual,
os checks de código não substituem a interação no navegador e a comparação com o design.
Para documentação, confira links locais, coerência dos IDs e `git diff --check`.

Registre um check indisponível com seu motivo. Uma etapa não executada não passa. CI pública
usa apenas fixtures autorais ou compatíveis. Aceite com pack privado roda localmente, sem
publicar enunciados, arquivos ou capturas desse material.

## Retomar e encerrar

Ao encerrar uma sessão, entregue um registro copiável para a próxima sessão:

```text
Unidade: F1.2
Papel: backend-developer
Estado: concluída | parcial | bloqueada
Alterações: caminhos e, se houver, commit ou PR
Verificação: comandos, resultados e cenários observados
Não verificado: itens e motivos
Pendências: critérios ainda ausentes
Próxima unidade: ID e dependências
```

Atualize o [registro de execução](progresso.md), apontando para a descrição do PR, relatório ou
handoff quando esses artefatos existirem. Só marque uma
entrega do roadmap depois de cumprir todos os seus critérios. A execução de uma unidade não
autoriza avançar automaticamente para outra fase ou publicar em produção.

Se vários agentes trabalharem ao mesmo tempo, atribua arquivos e unidades diferentes. Defina
um responsável por integração, lockfiles e arquivos gerados compartilhados. Reconfira o diff
antes de concluir. Não reverta trabalho alheio para resolver uma colisão.
