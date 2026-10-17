# ADR 0012 — Conteúdo de terceiros e licenças

- **Status:** aceito
- **Data:** 2026-10-14

## Contexto
O projeto usa como referência real listas de exercícios (Cálculo A — UFRGS), slides e listas de
POO (Unisinos) e apostilas de Estruturas (AU). Esse material tem direitos autorais dos professores
e das instituições, e o repositório é público/open source. Os PDFs de Cálculo foram versionados no
commit inicial (`docs/exercises/`).

## Decisão
1. **Material de terceiros fica fora do git**, em `content-private/` (no `.gitignore`), e entra no
   app como **pack privado** (`visibility: private`, `license: proprietary`, `citation` com a fonte).
2. **Banco público** (`content/`) aceita só:
   - conteúdo **autoral**, licenciado em **CC BY-SA 4.0** (`content/LICENSE`);
   - conteúdo de fontes abertas compatíveis (ex.: OpenStax CC BY, OATutor CC BY), com atribuição
     no campo `source`.
3. **O código continua MIT OR Apache-2.0.** Conteúdo e código têm licenças separadas.
4. Exercícios **inspirados** numa lista de terceiros podem ir para o banco público se forem
   reescritos (enunciado e números próprios). Citar a inspiração é opcional.
5. O domínio impede publicar exercício `proprietary` como `public` (regra testada no contexto `content`).
6. Recursos de IA nunca enviam conteúdo `private` a um provedor externo sem confirmação explícita
   por ação ([estratégia de IA](../../08-ia/estrategia-ia.md)).
7. A CI roda uma checagem que falha se aparecer PDF ou arquivo de `content-private/` no diff.

## Histórico do git (resolvido em 2026-10-14)
Os 5 PDFs de Cálculo estavam no commit inicial (`45975e6`) do repositório público. Esse commit foi
refeito sem `docs/exercises/` (novo `65fa622`), e o `main` do GitHub foi substituído por *force
push*, com o consentimento do mantenedor. O repositório não tinha forks nem estrelas. Um backup do
histórico antigo ficou só na máquina do mantenedor, em `content-private/_backups/` (fora do git).
Commits antigos podem continuar acessíveis por SHA no cache do GitHub até a coleta de lixo deles.
Para expurgar de imediato, é preciso abrir um pedido no GitHub Support (remoção de dados sensíveis).

## Alternativas consideradas
- **Publicar com atribuição** — só com permissão escrita; não é o padrão.
- **Repositório privado separado para conteúdo** — viável no futuro para turmas; hoje a pasta local basta.

## Consequências
- ✅ O projeto pode ser público sem risco de violar direitos autorais.
- ✅ A Lista 04 continua sendo o critério de aceite da F1, via pack privado local.
- ⚠️ Testes que dependem do pack privado não rodam na CI pública. Os testes de domínio usam só as
  **funções** matemáticas (não protegidas) e gabaritos recalculados.
