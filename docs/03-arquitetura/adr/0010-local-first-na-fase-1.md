# ADR 0010 — Local-first na Fase 1

- **Status:** aceito
- **Data:** 2026-10-14

## Contexto
Até a Fase 3 não haverá contas. Mesmo assim, o estudante precisa guardar exercícios próprios,
packs privados (material de terceiros que **não pode** sair da máquina), tentativas, cartões de
revisão e planos de estudo. O app roda na web (estático) e no desktop (Tauri). O ecossistema de
sync local-first (Zero 1.0, ElectricSQL + PGlite, PowerSync, Jazz…) ainda está amadurecendo, e
cada opção prende o servidor a uma tecnologia.

## Decisão
- **Banco público**: arquivos em `content/` compilados no build para JSON estático + índice de
  busca local (MiniSearch ou Pagefind). Sem servidor.
- **Dados do usuário**: atrás de **ports** no front (`ExerciseRepository`, `AttemptRepository`,
  `ReviewRepository`, `StudyPlanRepository`, `AssetStore`), com adapters:
  - web → **IndexedDB via Dexie** (maduro, pequeno, consultas reativas);
  - desktop → **SQLite** no lado Rust do Tauri (comando dedicado, capabilities mínimas).
- **Backup/portabilidade**: exportar e importar tudo como pack JSON; o desktop pode salvar em arquivo.
- **Sync**: fora de escopo até a F3. O schema já nasce com UUIDv7, `revision` e `created_at` em
  tudo, para facilitar sincronização e resolução de conflitos depois. Escolha da engine em ADR próprio.
- **Nenhuma telemetria por padrão.** Métricas de produto ficam locais; coleta só com opt-in
  (F3+).

## Alternativas consideradas
- **SQLite-WASM (OPFS) também na web** — mesmo SQL nos dois alvos, mas aumenta o bundle, e alguns
  VFS exigem cabeçalhos COOP/COEP. Reavaliar se as consultas de Progresso ficarem complexas.
- **PGlite** — Postgres no navegador (paridade com a F3), mas cerca de 3 MB a mais e maturidade
  ainda em evolução.
- **Adotar Zero/Electric já** — antecipa a infraestrutura de servidor que não existe na F1.

## Consequências
- ✅ Custo zero de infra, funciona offline e protege o material privado por construção.
- ✅ Trocar de adapter (ou adicionar `Http*` na F3) não toca UI nem regras.
- ⚠️ Dois adapters para manter (IndexedDB e SQLite) → testes de contrato compartilhados para os ports.
- ⚠️ Dados ficam presos ao navegador/dispositivo até haver sync, então o app deve lembrar o usuário
  de exportar backups.
