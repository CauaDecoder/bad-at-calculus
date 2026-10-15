@AGENTS.md

# Notas específicas do Claude Code

O conteúdo acima vem do `AGENTS.md`, que é a fonte única de verdade para qualquer ferramenta.
Edite as regras do projeto lá, não aqui. Este arquivo só acrescenta o que é exclusivo do Claude Code.

## Modo caveman por padrão

Ative a skill `caveman` (instalada em `~/.claude/skills/caveman`) na **primeira resposta de toda
sessão**, sem esperar `/caveman`, e mantenha até o usuário dizer "stop caveman" ou "normal mode".
Escopo e exceções: seção **Estilo de resposta** do AGENTS.md. Subagentes também respondem em
caveman; o que eles gravam em arquivos segue escrito normalmente. O `.caveman.json` na raiz fixa
`defaultMode: caveman` para hosts com o hook da skill.

## Subagentes (`.claude/agents/`)

Cada papel da seção **Papéis** do AGENTS.md existe como subagente. Delegue quando a tarefa for
claramente de um papel; para mudanças pequenas, faça direto.

| Quando | Subagente |
| ------ | --------- |
| Definir problema, histórias, critérios de aceite, prioridade | `product-manager` |
| Decisão de arquitetura, ADR, contrato, modelo de dados, revisão de fronteiras | `software-architect` |
| Código Rust (motor, correção, conteúdo, servidor) | `backend-developer` |
| Código Svelte/TS, UI, visualizações | `frontend-developer` |
| Testes, verificação, regressões | `qa-engineer` |
| Revisão de segurança (OWASP, ASVS, LLM Top 10), scanners | `security-researcher` |
| CI/CD, Docker, deploy, IaC, observabilidade | `devops-engineer` |
| Funcionalidades com LLM, evals, MCP | `ai-engineer` |
| Modelar exercícios, taxonomia, licenças | `content-author` |

Fluxo típico de uma funcionalidade: `product-manager` → `software-architect` (se houver decisão) →
`backend-developer` / `frontend-developer` → `qa-engineer` → `security-researcher` (se tocar uma
fronteira do modelo de ameaças).

## Skills (`.claude/skills/`)

- `nova-operacao-motor` — a receita de 7 passos para expor uma operação nova do motor.
- `novo-adr` — cria o próximo ADR numerado a partir do template e atualiza os índices.
- `modelar-exercicio` — transforma um enunciado (+ gabarito) num `*.ex.md` validado.

## Cuidados neste ambiente

- Windows: use o import `@AGENTS.md` (não symlink).
- `content-private/` contém material de terceiros: pode ser **lido** para modelar packs privados ou
  testes locais, mas nunca copiado para `content/`, `docs/` ou commits.
