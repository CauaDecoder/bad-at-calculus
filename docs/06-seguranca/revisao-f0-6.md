# Revisão de segurança da fundação — F0.6

Data: 2026-10-15. Papel: `security-researcher`. Escopo: fronteiras T1–T3, configuração
local de CI e estado consultável do repositório. Revisão documental e técnica local; não é
certificação OWASP/ASVS nem teste do ambiente publicado. Nenhuma alteração em código de
produção foi realizada no papel de revisão. Correções de DevOps estão separadas abaixo.

## Achados

### SEC-F0-01 — Alta: a branch principal não exige checks ou revisão

- **Referência:** T2/T3; OWASP A03:2025 (cadeia de suprimentos) e A08:2025 (integridade);
  ASVS 5.0, configuração e proteção da entrega.
- **Local:** configuração externa de `CauaDecoder/bad-at-calculus`, branch `main`.
- **Evidência:** `gh api repos/CauaDecoder/bad-at-calculus/branches/main` retornou
  `protected: false`. A API de Actions retornou `enabled: true`, `allowed_actions: all` e
  `sha_pinning_required: false`.
- **Cenário:** uma conta com escrita pode publicar mudanças sem os checks ou a revisão
  pretendidos. Não foi tentado bypass nem push. A ausência do controle foi confirmada pela API;
  comprometimento de conta é cenário de risco, não incidente observado.
- **Recomendação:** mantenedor deve configurar ruleset da `main`: checks obrigatórios de CI
  e segurança, uma aprovação humana, bloqueio de force push e restrição dos bypasses. Conferir
  regras de Code Scanning para alta/crítica e fixação de Actions por SHA. Não usar nomes de
  checks presumidos antes de sua primeira execução.
- **Estado:** aberto; workflows versionados não substituem proteção externa.

### SEC-F0-02 — Média: o canal de divulgação privada está desativado

- **Referência:** T2/T3; OWASP A03/A09:2025; processo de resposta a vulnerabilidades.
- **Local:** `SECURITY.md:10` e configuração externa de reporte privado.
- **Evidência:** `gh api repos/CauaDecoder/bad-at-calculus/private-vulnerability-reporting`
  retornou `enabled: false`. Não existe e-mail de segurança verificado no documento.
- **Cenário:** um pesquisador segue a política e não consegue enviar o relatório pelo canal
  indicado. Isso pode atrasar a correção ou induzir divulgação pública prematura.
- **Recomendação:** habilitar Private vulnerability reporting e testar a presença do botão.
  Até lá, não publicar provas de conceito em issues e não inventar endereço alternativo.
- **Estado:** aberto. A correção documental registra a indisponibilidade, mas não ativa o canal.

### SEC-F0-03 — Média: expressão profundamente aninhada causa trap no WASM

- **Referência:** T1; OWASP A06/A10:2025; ASVS 5.0, validação e tratamento de erros/recursos.
- **Locais:** `crates/vx-infra/src/parser/mod.rs:18`,
  `crates/vx-infra/src/parser/pratt.rs:39`,
  `apps/web/src/lib/adapters/engine/wasm-transport.ts:14` e
  `apps/web/src/lib/ui/components/ExpressionInput.svelte:25`.
- **Evidência:** parser recebe texto sem teto de bytes/profundidade; chamadas de grupos e
  potências são recursivas. Transporte chama `dispatch` na mesma thread. Input não define
  `maxlength`. Reprodutor abaixo, no binding real gerado com Rust 1.99.0/wasm-pack 0.15.0:
  profundidades 10 e 1.000 retornaram `{ latex: 'x' }`; profundidade 10.000 retornou
  `RuntimeError: memory access out of bounds`.
- **Cenário:** colar expressão patológica, ou futuramente importá-la, pode interromper o motor
  local. Não foi demonstrado vetor remoto, execução de código, vazamento ou indisponibilidade
  de servidor. O comportamento em navegador e a recuperação do módulo não foram verificados.
- **Recomendação:** encaminhar ao `backend-developer` limites de comprimento, profundidade,
  tamanho da AST e orçamento de trabalho com erros tipados e testes de fronteira. Encaminhar
  ao `frontend-developer` execução em Worker cancelável e recuperação controlada. Não corrigir
  apenas com `maxlength`, pois o contrato do motor também recebe entradas diretamente.
- **Estado:** preexistente, reproduzido; sem correção nesta unidade de DevOps.

Reprodução local limitada, sem arquivos de terceiros ou serviço externo:

```js
// node --input-type=module, na raiz; antes: pnpm wasm:build
import { readFileSync } from 'node:fs';
import init, { dispatch } from './apps/web/src/lib/wasm/pkg/vx_wasm.js';
await init({
  module_or_path: readFileSync('./apps/web/src/lib/wasm/pkg/vx_wasm_bg.wasm'),
});
for (const depth of [10, 1000, 10000]) {
  try {
    console.log(depth, dispatch('expression.latex', {
      expression: '('.repeat(depth) + 'x' + ')'.repeat(depth),
    }));
  } catch (error) {
    console.log(depth, String(error));
  }
}
```

### SEC-F0-04 — Média: cabeçalhos web ainda não estão preparados para publicação

- **Referência:** T1/T3; OWASP A02:2025; ASVS 5.0, configuração do cliente web.
- **Local:** `apps/web/svelte.config.js:7`; não existe `_headers` no checkout.
- **Evidência:** configuração não define CSP do SvelteKit; busca por `**/_headers` não retornou
  arquivo. O desktop possui CSP própria em `apps/desktop/src-tauri/tauri.conf.json:22`.
- **Cenário:** publicar os arquivos sem configuração adicional deixa o navegador sem as
  restrições CSP e os demais cabeçalhos definidos no plano de deploy.
- **Recomendação:** executar F0.7 antes da publicação; preparar CSP compatível com WASM,
  KaTeX e estilos, testar cabeçalhos HTTP reais, HSTS, nosniff, política de referência e permissões.
  Não inserir política arbitrária nesta unidade sem verificar o app publicado.
- **Estado:** ausência local confirmada; cabeçalhos do host e exploração de XSS não verificados.

### SEC-F0-05 — Média: advisory de unsoundness na cadeia Linux do desktop

- **Referência:** T2/T3; OWASP A03/A10:2025; ASVS 5.0, dependências.
- **Local:** `Cargo.lock:1069`, `glib@0.18.5`.
- **Evidência:** OSV 2.7.0 identificou RUSTSEC-2024-0429 / GHSA-wrw7-89jp-8q8g, CVSS 6.9.
  RustSec descreve escrita por referência imutável e possível dereferência nula em
  `VariantStrIter`, corrigida em `glib >=0.20.0`. `cargo tree --locked --target all -i glib`
  confirmou a cadeia GTK/WebKit/Tauri do desktop Linux; não é dependência do núcleo WASM.
- **Cenário:** uso das funções afetadas pelo app/dependências pode causar crash/UB. Presença
  do pacote foi confirmada; exploração e alcance das funções no app não foram demonstrados.
- **Recomendação:** encaminhar ao `backend-developer` atualização compatível da cadeia
  Tauri/GTK e teste no Linux, ou análise documentada de alcance com aceite explícito do
  mantenedor. Não forçar override de `glib` incompatível com a major requerida.
- **Estado:** preexistente; detectado pela primeira rodada de scanners Rust da fundação.
- **Atualização 2026-10-17:** risco aceito pelo mantenedor até 2027-04-17. Tauri 2.12.4 (última
  estável) ainda exige gtk-rs 0.18; `glib >=0.20` só chega com Tauri 3, em alpha. Busca no código
  das dependências do lockfile não encontrou chamada a `VariantStrIter`/`str_iter` fora do próprio
  `glib`, então a API afetada não é alcançável pelo app. Exceção com motivo e prazo em
  `osv-scanner.toml`; cargo-deny 0.20.2 não casa este advisory. Revisar ao migrar para Tauri 3.

### SEC-F0-06 — Informativos/baixo: manutenção e dependência de build

- **Referência:** T2/T3; OWASP A03:2025; ASVS 5.0, dependências.
- **Locais:** `Cargo.lock:2216` (`paste@1.0.15`), `Cargo.lock:2397`
  (`proc-macro-error@1.0.4`), `Cargo.lock:4488` (`yoke-derive@0.8.3`) e `pnpm-lock.yaml:517`
  (`cookie@0.6.0`).
- **Evidência:** RustSec marca `paste` (RUSTSEC-2024-0436) e `proc-macro-error`
  (RUSTSEC-2024-0370) como não mantidos, não como CVEs explorados. Cargo-audit também informa
  `yoke-derive` yanked. `paste` chega via nalgebra/simba ao motor; `proc-macro-error`, via
  macros GTK ao desktop. Audit npm completo e OSV identificaram GHSA-pxg6-pf52-xh8x em
  `cookie`, já conhecido da F0.3; audit npm classifica como baixo.
- **Cenário:** manutenção interrompida aumenta risco de cadeia; o advisory de `cookie`
  envolve atributos de cookie inválidos se controlados por entrada não confiável. A aplicação
  estática não demonstrou esse fluxo em produção. Dependência de build continua parte da cadeia.
- **Recomendação:** backend deve avaliar substituições/upstream, versão yanked e lockfile;
  frontend deve planejar migração compatível do Kit ou correção transitiva testada. Registrar
  alcance e aceite formal onde não houver versão corrigida; não ocultar avisos nos scanners.
- **Estado:** aberto; nenhuma alteração de dependência nesta unidade.
- **Atualização 2026-10-17:** `cookie` corrigido por override pnpm
  (`@sveltejs/kit>cookie` → `0.7.2`), porque o Kit 2.70.3, último 2.x, ainda declara `^0.6.0`;
  remover o override ao migrar para o Kit 3. `paste` e `proc-macro-error` não têm versão
  corrigida: risco aceito pelo mantenedor até 2027-04-17, com motivo em `deny.toml` e
  `osv-scanner.toml`. `yoke-derive` yanked continua apenas como aviso.

## Controles revisados e correções de DevOps

- F0.6 acrescenta `ci.yml` e `security.yml`, sem `pull_request_target`, segredos de repositório,
  ambientes, deploy ou execução em self-hosted runners. Permissões partem de `{}` e são
  declaradas por job. Checkout não persiste credenciais.
- Apenas CodeQL e Scorecard declaram escrita de eventos de segurança. OIDC existe apenas no
  job semanal Scorecard. Esse job não executa scripts do checkout e segue a lista de Actions
  permitida pelo projeto Scorecard. Nenhum PAT foi incluído.
- CodeQL depende da aprovação do guard privado antes de analisar/enviar resultados. O guard
  é um controle de CI, não uma garantia de que conteúdo nunca tenha sido publicado no Git.
- Actions externas usam SHAs de commits conferidos na API oficial. Downloads de binários
  exigem SHA-256 literal. Instalações Cargo usam versão e `--locked`; pnpm usa lockfile congelado.
- `deny.toml` não contém exceções a advisories. Verifica fontes e licenças inclusive de alvos
  desktop. Nenhuma dependência foi atualizada para mascarar resultado de scanner.
- Guard rejeita PDFs e caminhos privados, sem ler seu conteúdo ou imprimir seus nomes.
  Confere árvore e commits intermediários; inclui renome, maiúsculas, espaços, erros de Git e
  adição seguida de remoção. Esse controle detecta publicação indevida, mas não desfaz vazamento.
- `SECURITY.md` foi conferido e recebeu aviso de canal indisponível; é correção documental,
  não resolução da configuração externa.
- Renovate ficou restrito às Actions, sem automerge. Instalação do App e revisão humana
  continuam necessárias; versões de ferramentas baixadas manualmente não são atualizadas por ele.

## T1: controles futuros e saída HTML atual

Importação de packs, Markdown do usuário e PDFs ainda não existe. Zip slip, zip bomb,
sanitização de SVG/Markdown e regras `private`/`public` não foram aprovados por ausência de
implementação. Permanecem gates da F1, não controles entregues pela fundação.

`MathFormula.svelte:13` passa LaTeX ao KaTeX e injeta apenas o HTML retornado pelo renderer;
`trust` não é ativado. A regra Semgrep de HTML cru tem exceção apenas para esse componente,
com justificativa explícita. Isso não autoriza HTML de usuário nem dispensa revisão se o fluxo
mudar. Não foi demonstrado XSS no renderer atual. `maxSize` explícito e orçamento do renderer
continuam pontos de revisão para a entrada ampliada da F1.

## Dependências e scanners

Os resultados executados e ferramentas indisponíveis estão no
[relatório de execução](../05-engenharia/implementacao/f0-6-ci-seguranca.md). O relatório distingue
falha operacional do scanner de achado real. Saída verde de SAST não certifica ausência de falhas.

Cargo-deny também bloqueou cinco dependências MPL-2.0 fora da allowlist inicial: `cssparser`,
`cssparser-macros`, `dtoa-short`, `option-ext` e `selectors`. Trata-se de pendência da política
de licenças, não prova de vulnerabilidade ou de incompatibilidade jurídica. O mantenedor deve
revisar as obrigações antes de aprovar exceções específicas. Não foram inseridas exceções apenas
para tornar o check verde. Cargo-deny relatou ainda `unresolved-workspace-dependency` no Windows;
esse diagnóstico operacional não foi classificado como vulnerabilidade do produto.

**Atualização 2026-10-17:** o mantenedor aprovou MPL-2.0 apenas para essas cinco crates, por
exceção nominal em `deny.toml`; MPL-2.0 não entrou na allowlist global, então uma crate MPL nova
volta a bloquear o check. Todas vêm da cadeia desktop do Tauri e são usadas sem modificação. O
diagnóstico `unresolved-workspace-dependency` não se repetiu no cargo-deny 0.20.2 local nem no
runner Linux.

LLM não participa do fluxo atual; OWASP LLM Top 10:2025 é não aplicável à implementação da F0.
T4/T5 e consentimento para material privado continuam exigências de F2, sem aprovação antecipada.

## Limites da revisão

Não houve execução de PR/fork no GitHub, Scorecard agendado, CodeQL local, teste de deploy,
assinatura/proveniência de release, pentest de servidor ou teste de importação futura. Não
foram modificadas configurações remotas. Os achados não representam aceite de risco pelo
mantenedor. A unidade permanece parcial até reunir os critérios de execução e triagem.
