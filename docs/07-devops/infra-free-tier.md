# Infra em free tier

Levantamento de 2026-10. **Limites mudam**: confira a página oficial antes de depender de um número.
Regra do projeto: tudo deve caber nos limites gratuitos com folga de 50% e poder ser recriado em
outro provedor a partir do IaC.

| Serviço | Uso na Vértice | Limites gratuitos relevantes | Fase |
| ------- | -------------- | ---------------------------- | ---- |
| **GitHub** | Código, Actions, Releases, GHCR, CodeQL, Pages (alternativa) | Actions ilimitado para repo público; CodeQL gratuito em repo público | F0 |
| **Cloudflare Pages / Workers static assets** | Site estático + previews | Builds e banda generosos; Workers ~100 mil req/dia no free | F0 |
| **Cloudflare R2** | Assets e backups | ~10 GB, 1 M operações classe A e 10 M classe B por mês, **sem egress** | F3 |
| **Cloudflare Tunnel, WAF básico, DNS** | Entrada do servidor sem portas abertas | Gratuitos | F3 |
| **Cloudflare D1** | Não planejado (Postgres no Oracle), mas é alternativa | 500 MB por banco, 10 bancos, 5 GB por conta | — |
| **Oracle Cloud Always Free** | `vx-server`, Postgres, IdP, Coolify | ARM Ampere A1: **2 OCPU / 12 GB** segundo relatos de ago/2026 (antes 4/24; fontes divergem); 200 GB de bloco; 2 VMs AMD micro | F3 |
| **Grafana Cloud free** | Métricas, logs, traces | Cotas mensais limitadas (verificar); alternativa: Prometheus + Loki + Grafana self-host | F3 |
| **Ollama** | LLM local para o importador e dicas | Grátis; limitado pelo hardware do usuário | F2 |

## Orçamento de recursos no Oracle (alvo 2 OCPU / 12 GB)

| Componente | RAM alvo |
| ---------- | -------- |
| Coolify + Traefik | ~1 GB |
| `vx-server` (Axum) | 128–256 MB |
| Postgres + pgvector | 2–3 GB |
| IdP (Zitadel/Authentik) | 0,5–1,5 GB |
| OTel Collector | 128 MB |
| Folga (builds, picos) | ≥ 5 GB |

Riscos conhecidos:
- **Recuperação de instâncias ociosas** pela Oracle → healthcheck periódico e alertas.
- **Capacidade ARM esgotada** em regiões populares → escolher a região na criação e manter o IaC
  pronto para outra.
- **Mudança de cota** → backups no R2 + OpenTofu.

## Fontes
- https://terminalbytes.com/oracle-cloud-free-tier-changes-2026
- https://developers.cloudflare.com/d1/platform/limits
- https://coolify.io/docs/knowledge-base/server/oracle-cloud
