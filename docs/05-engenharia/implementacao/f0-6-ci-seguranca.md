# F0.6 — CI e revisão de segurança da fundação

Data: 2026-10-15. Estado: **parcial**. Implementação: `devops-engineer`. Revisão documental
e técnica: `security-researcher`, em [relatório separado](../../06-seguranca/revisao-f0-6.md).
F0.2 e F0.3 têm evidência registrada e atendem às dependências locais desta unidade.

## Entrega local

- `.github/workflows/ci.yml`: fmt, Clippy, testes Rust sem desktop, checks/lint/testes web,
  geração real do WASM, build estático, actionlint e testes do guard.
- `.github/workflows/security.yml`: matriz cargo-audit/cargo-deny/OSV/pnpm, Gitleaks, Semgrep CE,
  CodeQL para TypeScript/JavaScript e Rust, guard privado e Scorecard semanal.
- `.github/actions/setup-web/action.yml`: pnpm conforme `packageManager`, Node 24.16.0,
  cache do store e instalação congelada. O binding é gerado antes dos checks web; não há stub.
- `deny.toml`: fontes e licenças permitidas, advisories sem supressão, todos os alvos do lockfile.
- `.semgrep.yml`: regras locais para código dinâmico JS/TS, shell Rust e HTML cru Svelte.
  Exceção de sink KaTeX fundamentada no relatório de revisão, não em entrada arbitrária de usuário.
- `scripts/ci/private-content-guard{,.test}.mjs`: proteção de árvore e histórico, incluindo
  commits intermediários. Só consulta nomes Git; não lê arquivos privados.
- `renovate.json`: atualização de Actions fixadas por digest, sem automerge.
- `SECURITY.md` conferido; canal privado indisponível explicitado.
- Documentação de CI, ameaças e progresso atualizada sem marcar os gates remotos como entregues.

Nenhuma mudança em motor, UI, dependências ou lockfiles nesta unidade. Alterações anteriores e
arquivos não rastreados do mantenedor foram preservados. Não houve commit, push, PR, deploy,
instalação de GitHub App ou mudança de configuração remota. O checkout contém unidades
anteriores ainda não commitadas; não seria seguro publicar seu conteúdo inteiro por inferência.

## Ambiente e linha de base

Windows x64; Rust 1.99.0, Node 24.16.0, pnpm 9.12.0 via `corepack`, wasm-pack 0.15.0.
GitHub CLI autenticada; consultas ao remoto foram apenas de leitura. O repositório é público,
branch padrão `main`. O terminal não possui shim `pnpm`; os comandos locais usam `corepack pnpm`.
Em Actions, `pnpm/action-setup` disponibiliza o comando `pnpm`.

Antes das alterações, passaram `cargo fmt --all -- --check`, Clippy com `--locked` e
`-D warnings`, 140 testes Rust, check sem erros/avisos, lint e 61 testes web em 13 arquivos.
Esses checks continuam válidos para código de produção, que não foi alterado nesta unidade.

## Verificações executadas

| Verificação | Resultado local |
| --- | --- |
| `cargo fmt --all -- --check` | Aprovado |
| `cargo clippy --workspace --exclude vertice-desktop --all-targets --locked -- -D warnings` | Aprovado |
| `cargo test --workspace --exclude vertice-desktop --locked` | 140 testes aprovados |
| `corepack pnpm -C apps/web check` | Zero erros e avisos |
| `corepack pnpm -C apps/web lint` | Aprovado |
| `corepack pnpm -C apps/web test` | 61 testes aprovados em 13 arquivos |
| `corepack pnpm wasm:build` | WASM real gerado; aviso de LICENSE ausente no pacote gerado |
| `corepack pnpm wasm:build -- --locked` | Repetição aprovada com lockfile obrigatório, usada pela CI |
| `corepack pnpm -C apps/web build` | Aprovado; adapter-static escreveu `build/` |
| `node --test scripts/ci/private-content-guard.test.mjs` | 9 aprovados; 1 ignorado por restrição de nomes do Windows |
| Guard sobre árvore de `HEAD` e histórico completo | Aprovado; não inclui as alterações ainda não commitadas |
| Semgrep CE 1.181.0, configuração final | Zero achados em 171 alvos, três regras; saída 0 |
| Semgrep em três fixtures sintéticas fora do repo | Três achados e saída 1, como esperado: TS `eval`, shell Rust e HTML cru Svelte |
| `corepack pnpm audit --prod --json` | Zero achados; saída 0 |
| `corepack pnpm audit --json` | Um achado baixo, `cookie@0.6.0`, GHSA-pxg6-pf52-xh8x; saída 1 |
| `actionlint` 1.7.12 | Workflows aprovados; ShellCheck não disponível neste Windows |
| Gitleaks 8.30.1, histórico `--all`, `--redact` | Quatro commits; nenhum segredo encontrado |
| Gitleaks em cópia explícita dos arquivos novos/configurados da unidade | Nenhum segredo encontrado; sem varrer diretórios privados locais |
| `cargo audit` 0.22.2 | Saída 0; quatro avisos permitidos pelo padrão: glib, paste, proc-macro-error e yoke-derive |
| OSV 2.7.0, `scan source --recursive .` | Saída 1; quatro pacotes afetados: glib, paste, proc-macro-error e cookie |
| `cargo deny --locked check` 0.20.2 | Reprovado: advisories e licenças; diagnóstico de resolução de workspace no Windows |
| `git diff --check` | Aprovado; avisos de conversão LF/CRLF do checkout |

Conferência final aprovou novamente actionlint e `git diff --check`; 51 links locais dos
documentos afetados apontam para arquivos existentes. A repetição final dos testes do guard
manteve 9 aprovações e 1 ignorado. O cargo-deny final produziu cinco rejeições de licença,
dois erros de manutenção, dois diagnósticos operacionais de herança de workspace, além dos
avisos de duplicatas, wildcard, licenças não encontradas e versão yanked; saída 5 no modo JSON.

O teste ignorado usa nome de arquivo com quebra de linha, inválido no Windows. O mesmo teste
executará em Linux no job `workflow-checks`; aprovação em Linux ainda não foi comprovada.
As fixtures do guard são autorais e temporárias, sem conteúdo de terceiros. A execução completa
do histórico não lê nem publica `content-private/` local ignorado pelo Git.

O achado `cookie` pertence às dependências de build do SvelteKit e já estava registrado na F0.3.
`--prod` não o inclui porque Kit está em `devDependencies`. Isso não é prova de ausência na
cadeia de build: OSV confere também o lockfile completo. Não houve override ou atualização
de dependência nesta unidade; correção é entrega separada do `frontend-developer`.

### Scanners e validação dos workflows

CodeQL e Scorecard não foram executados localmente. Os testes manuais Semgrep usam fixtures
em `C:/Users/anyyt/AppData/Local/Temp/opencode/f0-6-sast-fixture/`; ferramentas auxiliares ficam
sob o diretório temporário aprovado, fora do Git.

OSV conferiu `Cargo.lock` (436 pacotes) e `pnpm-lock.yaml` (220 pacotes). Reportou um médio
(`glib`, CVSS 6.9), zero altos/críticos e três sem severidade própria no relatório. Os dois
advisories de crates não mantidas são informativos; `cookie` é baixo segundo npm. Ausência
de severidade OSV não significa ausência de risco. O alcance do código afetado precisa de triagem.

Cargo-audit carregou 1.329 advisories, não relatou vulnerabilidade bloqueante e produziu quatro
avisos: `glib` unsound, `paste` e `proc-macro-error` não mantidos, `yoke-derive` yanked.
Cargo-deny bloqueia os dois advisories de manutenção e cinco crates MPL-2.0 não autorizadas pela
allowlist inicial (`cssparser`, `cssparser-macros`, `dtoa-short`, `option-ext`, `selectors`).
Fontes passaram. Duplicatas, wildcards das dependências locais por `path` e versão yanked são
avisos. A primeira configuração tentou negar todos os wildcards, produzindo ruído sobre as
crates locais da arquitetura; a configuração final mantém esse diagnóstico informativo,
sem alterar manifests. Advisories, fontes e licenças não foram relaxados.

O binário Windows do cargo-deny também emitiu `unresolved-workspace-dependency` para herança
de `vx-api` no desktop e WASM. Isso é limitação operacional a reproduzir no runner Linux, não
prova de falha no contrato Cargo: `cargo metadata`, builds e testes reconhecem as dependências.
Não declarar o check aprovado enquanto os achados e o diagnóstico persistirem.

Downloads auxiliares Windows foram conferidos com SHA-256 dos assets oficiais. Uma tentativa
via `Invoke-WebRequest` produziu arquivo com checksum divergente e foi rejeitada sem executar
o binário. Repetição com `curl.exe --fail --location --max-time 120` em diretório separado
conferiu os quatro binários (Gitleaks, OSV, cargo-deny e actionlint). A CI usa `curl` com checksum
literal para os binários baixados e instalação Cargo com versão/lockfile para ferramentas Rust.
Essa falha de transporte não foi tratada como achado de vulnerabilidade nem contornada sem hash.

## Segurança: correções versus achados

As correções desta entrega são isolamento dos workflows, pinagem, permissões, lockfiles no
pipeline, scanners configurados, guard e aviso no `SECURITY.md`. Não resolvem automaticamente
os riscos de configuração remota ou do produto. A revisão separada documenta:

- **Alta:** `main` sem proteção, confirmada pela API.
- **Média:** reporte privado desativado, confirmado pela API.
- **Média:** parser sem limite de profundidade; trap no WASM reproduzido com 10.000 grupos.
- **Média:** `_headers`/CSP web ausentes no checkout; conferir cabeçalhos e deploy na F0.7.
- **Média:** `glib@0.18.5` na cadeia Linux do desktop, advisory de unsoundness.
- **Informativos/baixo:** crates não mantidas, versão yanked e `cookie` de build; triagem separada.

Importação de packs, leitor PDF, LLM, servidor e assinatura de release não foram antecipados.
A falha 3D `effect_update_depth_exceeded` registrada anteriormente não foi corrigida nem
testada de novo; o mesmo vale para recuperação do protótipo. Nenhuma aprovação visual ou
smoke desktop foi inferida dos builds desta unidade.

## Pendências para concluir F0.6

1. Mantenedor autorizar e organizar a publicação das unidades locais em branch/PR sem incluir
   material privado; a execução desta unidade não publicou o trabalho anterior.
2. Executar `CI` e `Security` em PR real. Registrar URL, commit, jobs, logs e resultados.
   Repetir em fork para conferir restrição de token e ausência de segredos. Não basta validar YAML.
3. Tratar os resultados de cargo-audit, cargo-deny e OSV, inclusive os das dependências desktop
   e de build. Não desativar scanners ou suprimir advisories para declarar verde.
4. Conferir CodeQL e seu upload SARIF nos eventos suportados; executar o Scorecard semanal
   depois do merge. Confirmar recurso Code Scanning habilitado, sem duplicar default setup.
5. Habilitar reporte privado e ruleset da `main`; instalar Renovate. Automatizar triagem por
   issue de achados médios em contexto confiável, sem dar escrita a código de PR. Até lá,
   triagem é manual e essa parte do plano de CI continua pendente.
6. Conferir orçamento US$ 0 e limite de cache (10 GB) na conta. Runners Ubuntu padrão em repo
   público não consomem minutos pagos; os workflows não criam artefatos de bundle ou deploy.

F0.6 não está concluída sem execução real dos checks em PR. F0.7 continua dependente desta
e das unidades visuais; este relatório não autoriza publicação nem avanço automático.

## Fontes consultadas

- [Cobrança GitHub Actions](https://docs.github.com/en/billing/concepts/product-billing/github-actions):
  runners padrão gratuitos para repositórios públicos; cache separado de 10 GB/repositório.
- [CodeQL v4.39.0](https://github.com/github/codeql-action/releases/tag/v4.39.0) e manifests
  da Action fixada por commit; linguagem Rust e modo sem build previstos pelo scanner atual.
- [Scorecard v2.5.0](https://github.com/ossf/scorecard-action/tree/fa7117db83c52d787ae5a89f350999701505d78e):
  OIDC e restrições de steps para publicação de resultados.
- Releases oficiais e Git refs consultados pela API do GitHub para Actions e ferramentas.
  Tags anotadas foram resolvidas até o commit; não foi usado SHA de objeto tag como Action.
