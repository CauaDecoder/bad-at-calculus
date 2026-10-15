# OWASP na Vértice

Os três documentos que guiam a segurança do projeto e como cada item se aplica. Detalhes por
fronteira em [modelo-de-ameacas.md](modelo-de-ameacas.md).

## Metas por fase

| Fase | Meta |
| ---- | ---- |
| F0–F2 (estático, sem contas) | **ASVS 5.0 nível 1** nos capítulos aplicáveis (validação, codificação de saída, configuração, arquivos, dependências) |
| F3 (contas e servidor) | **ASVS 5.0 nível 2** completo |
| F4 (comunidade, conteúdo de usuários públicos) | Nível 2 + revisão de moderação/abuso; pentest externo voluntário (programa de divulgação) |

O ASVS 5.0 (maio/2025) tem cerca de 350 requisitos em 17 capítulos e 3 níveis; OAuth/OIDC e
tokens autocontidos ganharam capítulos próprios, e são relevantes para a F3.

## OWASP Top 10:2025

| # | Risco | Onde aparece na Vértice | Controles |
| - | ----- | ----------------------- | --------- |
| A01 | Broken Access Control (inclui SSRF) | F3: listas, tentativas, packs de outros usuários | Autorização por caso de uso, RLS, testes de IDOR, sem busca de URL arbitrária |
| A02 | Security Misconfiguration | Cabeçalhos do site estático, Tauri capabilities, Postgres, Coolify | CSP/HSTS/`X-Content-Type-Options`/`Referrer-Policy` versionados; IaC; *hardening* revisado |
| A03 | Software Supply Chain Failures | npm, crates, GitHub Actions, imagens Docker | Lockfiles, `cargo-deny`, OSV-Scanner, Renovate, Actions por SHA, SBOM, cosign, Scorecard |
| A04 | Cryptographic Failures | F3: sessões, segredos, backups | TLS em tudo (Cloudflare), segredos fora do repo, backups criptografados no R2 |
| A05 | Injection | XSS em enunciados (Markdown/KaTeX/SVG), SQL na F3, código em exercícios | Sanitização (DOMPurify), KaTeX `trust: false`, SQLx com consultas parametrizadas, sandbox de código |
| A06 | Insecure Design | Importação de packs, IA, correção | Modelo de ameaças por fase; regras de domínio (`proprietary` ≠ `public`); limites de importação |
| A07 | Authentication Failures | F3 | Passkeys/OIDC, sem senha própria, sessões rotacionadas, proteção contra *credential stuffing* na borda |
| A08 | Software or Data Integrity Failures | Packs importados, releases, updater do Tauri | Validação de schema no núcleo, releases e atualizações assinados, proveniência SLSA |
| A09 | Security Logging and Alerting Failures | F3 | `tracing` estruturado, auditoria, alertas, retenção definida |
| A10 | Mishandling of Exceptional Conditions | Todo o núcleo | Já é política: sem `unwrap`/`expect`/`panic!` fora de testes (clippy `deny`), erros tipados (`ApiError`), *fail closed* na correção ("indeterminado" ≠ "correto") |

## OWASP Top 10 for LLM Applications 2025

| # | Risco | Aplicação |
| - | ----- | --------- |
| LLM01 | Prompt Injection | PDFs e enunciados importados são dados não confiáveis |
| LLM02 | Sensitive Information Disclosure | Packs privados e chaves BYOK |
| LLM03 | Supply Chain | Modelos baixados (Ollama), pacotes de SDK |
| LLM04 | Data and Model Poisoning | Se no futuro houver fine-tuning com exercícios da comunidade |
| LLM05 | Improper Output Handling | Saída do LLM → JSON Schema → sanitização |
| LLM06 | Excessive Agency | `vertice-mcp` só com ferramentas de leitura e cálculo |
| LLM07 | System Prompt Leakage | Prompts sem segredos; tratar o system prompt como público |
| LLM08 | Vector and Embedding Weaknesses | Busca semântica (F2+): separar índices público/privado; nunca misturar dados de usuários |
| LLM09 | Misinformation | Dicas fundamentadas nos passos do motor; a correção nunca depende do LLM |
| LLM10 | Unbounded Consumption | Orçamentos, limites de tokens, cache |

## Ferramentas (todas open source e gratuitas)

| Categoria | Ferramenta |
| --------- | ---------- |
| SAST | CodeQL (GitHub, gratuito para repo público), Semgrep CE |
| Dependências | `cargo-deny`, `cargo-audit`, OSV-Scanner, `pnpm audit`, Renovate |
| Segredos | gitleaks |
| Containers (F3) | Trivy, Grype |
| DAST (F3) | OWASP ZAP no ambiente de preview |
| Postura do repo | OpenSSF Scorecard |
| SBOM e assinatura | syft, cosign (Sigstore), GitHub Artifact Attestations |
| Fuzzing | `cargo-fuzz` no parser e no leitor de packs |

## Referências
- https://owasp.org/Top10/2025/
- https://owasp.org/www-project-application-security-verification-standard/
- https://genai.owasp.org/llm-top-10/
