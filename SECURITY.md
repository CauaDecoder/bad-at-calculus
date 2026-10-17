# Política de segurança

## Versões suportadas

A Vértice ainda está em desenvolvimento (0.x). Correções de segurança são feitas apenas na
branch `main` e no release mais recente.

## Como reportar uma vulnerabilidade

**Não abra uma issue pública.** Use o
[reporte privado de vulnerabilidades do GitHub](https://docs.github.com/pt/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)
na aba **Security → Report a vulnerability** do repositório.

**Disponibilidade do canal:** na conferência de 2026-10-15, a API do GitHub retornou
`enabled: false` para o reporte privado deste repositório. O mantenedor precisa habilitar
**Settings → Code security → Private vulnerability reporting** antes de considerar o canal
operacional. Se o botão não aparecer, não publique a prova de conceito em issues, PRs ou
discussões. Use apenas um canal privado já estabelecido com o mantenedor para solicitar uma
forma segura de envio; não há e-mail de segurança verificado neste documento.

Inclua, se possível:
- componente afetado (motor Rust, app web, desktop, importação de packs, IA…);
- passos para reproduzir ou prova de conceito;
- impacto esperado.

## O que esperar

| Etapa | Prazo-alvo |
| ----- | ---------- |
| Confirmação de recebimento | 3 dias |
| Avaliação inicial e severidade | 7 dias |
| Correção ou plano de mitigação | 30 dias para severidade alta/crítica |

Projeto voluntário: os prazos são metas, não garantias. Quem reportar recebe crédito no advisory,
se quiser.

## Escopo

Dentro do escopo: este repositório, os releases publicados e o site oficial. Fora do escopo:
ataques de engenharia social, DoS volumétrico contra a infraestrutura e vulnerabilidades em
dependências já públicas sem impacto demonstrado na Vértice (reporte ao projeto de origem).

Modelo de ameaças e padrões seguidos: [docs/06-seguranca](docs/06-seguranca/modelo-de-ameacas.md).
