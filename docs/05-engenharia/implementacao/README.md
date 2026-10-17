# Implementar a Vértice por fases

Use estes guias para dar tarefas curtas a agentes de terminal. Cada fase contém unidades com
ID, dependências, papel responsável, entrega e verificação. As unidades organizam a execução
do [roadmap](../../02-produto/roadmap.md), sem substituir os PRDs ou os ADRs.

## Iniciar uma sessão

Indique o arquivo e o ID da unidade. Por exemplo:

```text
Execute F0.1 conforme docs/05-engenharia/implementacao/f0-fundacao.md.
```

Para outra fase, use o mesmo formato:

```text
Execute F1.2 conforme docs/05-engenharia/implementacao/f1-exercicios.md.
```

Para retomar sem saber o próximo ID:

```text
Retome a próxima unidade pronta conforme docs/05-engenharia/implementacao/README.md.
```

Consulte o [registro de execução](progresso.md) antes de escolher a próxima unidade. Ele indica
o que foi verificado, onde está a evidência e o que precisa ser retomado.

Esse pedido autoriza executar uma unidade pronta, depois de conferir suas dependências. Se o
pedido autorizar uma fase inteira, avance por unidades verificadas e respeite os papéis do AGENTS.md.
Uma unidade pode ocupar mais de uma sessão. O tamanho do arquivo não é o tamanho do PR.

## Escolher a fase

| Fase | Guia | Resultado ao concluir |
| --- | --- | --- |
| F0 | [Fundação](f0-fundacao.md) | Nome, navegação, site aprovado, build, CI e publicação estática |
| F1 | [Exercícios offline](f1-exercicios.md) | Lista resolvida e corrigida offline, autoria, backup, referências e evidência |
| F2 | [Estudo local ampliado](f2-estudo-local.md) | Revisão, Biblioteca, IA opcional e novas disciplinas |
| F3 | [Contas e nuvem](f3-contas-e-nuvem.md) | Conta opcional, sincronização e arquivos autorais privados |
| F4 | [Comunidade](f4-comunidade.md) | Grupos, publicação autoral e moderação |

## Ler somente o necessário

Toda execução começa pelas [regras para agentes](regras-para-agentes.md). Para trabalho visual,
leia também o [guia de fidelidade ao design](design-aprovado.md). Depois siga as leituras da
unidade escolhida. Os links são relativos a estes arquivos e funcionam em qualquer checkout.

O roadmap mantém o estado das entregas de produto. Estes guias mantêm a ordem de execução e os
critérios por unidade. Atualize ambos somente com evidência da entrega correspondente. Não
marque uma fase como pronta porque sua documentação existe.

## Conferir o ponto de partida

Em 2026-10-15, o Laboratório de Cálculo e VGA existe. A navegação por módulos da F0.4 foi
verificada no build estático, com tópicos em `/lab/...` e compatibilidade das URLs antigas.
O renome técnico local da F0.2 usa `vx-*` e `@vertice/*`; veja os checks e bloqueios no
[registro de execução](progresso.md). A F1.1 entregou os valores compartilhados e contratos
estruturais de `content` e `assessment`, com schemas e tipos TypeScript gerados; veja o
[relatório de contratos](f1-1-contratos.md). A correção F1.2 tem os
[nove tipos e a checagem de forma implementados](f1-2-correcao.md). Compilação de autoria,
`review`, `progress` e `library` continuam planejados. Reconfira o código antes de escolher uma unidade.

Os ADRs 0013, 0014 e 0015 estão aceitos. O ADR 0011 continua proposto; a F3 exige revisão antes
da implementação. O spike do arXiv está concluído e é uma entrada da F2.

A fonte navegável do protótipo exige recuperação. Na consulta ao GitHub em 2026-10-15,
`prototype/vertice-design` não apareceu entre os branches do remoto deste repositório.
Veja a tarefa de recuperação no [guia de design](design-aprovado.md#recuperar-a-fonte-do-protótipo).
Isso não bloqueia unidades de motor, contratos ou CI. Impede declarar a comparação visual com
o protótipo como concluída.
