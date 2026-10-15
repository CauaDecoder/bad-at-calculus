# ADR 0011 — Backend futuro: Rust Axum + Postgres

- **Status:** proposto (implementação na Fase 3; revisar antes de começar)
- **Data:** 2026-10-14

## Contexto
Contas, sincronização, listas compartilhadas e comunidade (F3–F4) exigem um servidor. Restrições:
- reaproveitar o núcleo Rust (mesma correção e validação de packs do cliente);
- custo zero ou quase zero (Oracle Always Free ARM + Cloudflare free);
- servir de laboratório de **DevOps e segurança** para o mantenedor.

Fatos relevantes da pesquisa (2026):
- a cota Always Free ARM da Oracle teria caído de 4 OCPU/24 GB para **2 OCPU/12 GB** (fontes
  divergem), e instâncias ociosas podem ser recuperadas;
- Cloudflare D1 free tem 500 MB por banco;
- Coolify v4 está estável e tem guia para Oracle ARM;
- Better Auth absorveu o Auth.js e o Lucia foi descontinuado, mas ambos são TypeScript.

## Decisão (proposta)
- **`vx-server`**: Axum + Tokio + SQLx + `tracing`, chamando os **mesmos casos de uso** do núcleo
  ([ADR 0007](0007-contextos-como-modulos.md)). API HTTP JSON com OpenAPI gerado (`utoipa`).
- **Postgres** (com `pgvector` para busca semântica), Row-Level Security por `owner_id`.
- **Assets** e backups no **Cloudflare R2**; site estático no Cloudflare; **Cloudflare Tunnel** até
  o servidor (sem portas abertas), WAF e rate limit na borda.
- **Oracle ARM** gerenciado por **Coolify** (ou k3s como exercício de estudo), com tudo descrito em
  **OpenTofu**. Tudo precisa caber em **2 OCPU/12 GB**.
- **Autenticação OIDC com passkeys**: IdP self-hosted (Zitadel, Authentik ou Keycloak) **ou**
  sessões próprias com `webauthn-rs`. A escolha fica para um ADR na F3; o `vx-server` só valida
  tokens/sessões e nunca guarda senha.

## Alternativas consideradas
- **Serverless Cloudflare** (SvelteKit em Workers + D1 + Better Auth) — menos operação, mas limites
  do free tier, servidor em TS (sem reaproveitar a correção Rust, exceto via WASM no Worker) e menos
  aprendizado de infraestrutura.
- **Híbrido** (borda em Workers, Rust só para o pesado) — viável, mas dobra o número de deploys
  logo no início. Pode surgir naturalmente depois (ex.: Worker para rate limit/cache).
- **BaaS (Supabase/PocketBase)** — rápido, mas a regra de correção ficaria fora do servidor.

## Consequências
- ✅ Correção idêntica cliente/servidor; uma linguagem no núcleo; trilha completa de DevOps.
- ⚠️ Operar Postgres, backups, atualizações e monitoramento é responsabilidade do projeto.
- ⚠️ Risco de mudança/recuperação de cota da Oracle → IaC e backups no R2 permitem recriar o
  ambiente em outro provedor em horas.
