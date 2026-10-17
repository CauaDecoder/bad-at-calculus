---
name: novo-adr
description: Cria o próximo ADR (Architecture Decision Record) numerado da Vértice a partir do template e atualiza os índices. Use quando uma decisão de arquitetura, stack, contrato, modelo de dados, segurança ou infraestrutura for tomada ou proposta.
---

# Novo ADR

1. Descubra o próximo número:
   ```bash
   ls docs/03-arquitetura/adr/ | grep -E '^[0-9]{4}-' | sort | tail -1
   ```
2. Crie `docs/03-arquitetura/adr/NNNN-titulo-em-kebab-case.md` a partir de
   `docs/03-arquitetura/adr/template.md`:
   - **Status:** `proposto` (decisão ainda não implementada/validada) ou `aceito`.
   - **Data:** hoje (AAAA-MM-DD).
   - **Contexto:** o problema, as forças e restrições (free tier, local-first, licenças, equipe pequena).
   - **Decisão:** uma ou duas frases afirmativas e, se preciso, os detalhes.
   - **Alternativas consideradas:** pelo menos duas, com prós e contras honestos.
   - **Consequências:** ✅ o que melhora, ⚠️ custos e riscos, 🔭 próximos passos.
3. Se substitui um ADR anterior, **não o reescreva**. Mude só o status do antigo para
   `substituído por ADR NNNN`.
4. Atualize os índices:
   - lista "Decisões registradas" em `docs/03-arquitetura/visao-geral.md`;
   - contagem/descrição em `docs/README.md`, se mudar.
5. Se a decisão afeta regras para agentes (camadas, contratos, licenças), atualize o `AGENTS.md`.
6. Documentação em PT-BR; nomes de código em inglês.
