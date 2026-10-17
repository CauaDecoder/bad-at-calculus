# F1.1 — Contratos de conteúdo e referências

Execução em 2026-10-15, no Windows x64. Papel de implementação: `backend-developer`.
Revisão de contrato coordenada com `software-architect`, autorizada pelo mantenedor após
o bloqueio inicial. A dependência F0.2 já estava registrada como concluída.

## Entrega

- [ADR 0016](../../03-arquitetura/adr/0016-contrato-de-conteudo-f1.md): `Part` pertence a
  `content`; `AnswerSpec` pertence a `assessment`. Conteúdo importa somente a fachada
  pública do corretor. ADRs históricos foram preservados.
- `crates/vx-domain/src/shared/`: habilidades, licenças, visibilidade, erros por campo e
  valores JSON técnicos para soluções e visualizações, sem dependências de serialização.
- `crates/vx-domain/src/assessment/`: nove especificações de resposta da F1, elementos
  escalares não recursivos e validação estrutural. Não há algoritmo de correção nesta unidade.
- `crates/vx-domain/src/content/`: exercícios, partes, listas, manifesto, pack, fonte,
  assets, soluções, visualizações e referências. Validação inclui identidade, revisão,
  duplicatas, referências cruzadas e fronteira de publicação.
- `crates/vx-api/src/{assessment,content}.rs`: DTOs com `serde`, `schemars` e `ts-rs`.
  Conversões explícitas ficam nos módulos `assessment_conversion` e `content_conversion`.
  `content_decode` preserva caminhos de erro, inclusive nos elementos das coleções de respostas.
- `contracts/*.schema.json` e `apps/web/src/lib/domain/generated/content.ts`: contratos
  gerados dos DTOs Rust. Não há schema ou união de respostas escritos separadamente em TypeScript.
- `crates/vx-api/tests/fixtures/{exercise,list,manifest}.json`: exemplos autorais mínimos,
  sem conteúdo de terceiros. São fixtures de contrato, não o catálogo público da F1.9.

As fachadas `vx_api::content::{decode_exercise, decode_list, decode_manifest, decode_pack}`
decodificam JSON e validam o domínio. A fronteira de publicação usa
`validate_for_publication()`: validar um documento privado para armazenamento local não
autoriza publicá-lo. Um manifesto público também não promove documentos privados incluídos.

O JSON Schema verifica a estrutura e as restrições expressáveis pelos contratos gerados.
As regras de publicação, existência de referências cruzadas, retangularidade da matriz e
demais invariantes semânticas continuam no domínio; aceitar um documento no schema não
substitui a validação completa. O TypeScript representa os DTOs serializados, com coleções
normalizadas pelos defaults do desserializador Rust.

## Geração reproduzível

```bash
cargo run -p vx-api --example generate_content_contracts
cargo run -p vx-api --example generate_content_contracts -- --check
```

Também disponíveis como `pnpm contracts:generate` e `pnpm contracts:check`. Os destinos são
ancorados no manifesto de `vx-api`. A geração escreve cinco schemas e um arquivo TypeScript.
`--check` apenas compara o conteúdo e falha se houver divergência ou arquivo ausente.
O teste de contratos compara os arquivos com a geração atual e verifica determinismo.
A integração na CI de conteúdo permanece na F1.9.

## Verificação

Linha de base: `cargo test --workspace --exclude vertice-desktop` passou com 140 testes;
`pnpm -C apps/web check` passou sem erros ou avisos. Nesta máquina, os comandos pnpm foram
invocados por `corepack pnpm`.

Verificação final:

| Comando | Resultado |
| --- | --- |
| `cargo fmt --all -- --check` | Aprovado |
| `cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings` | Aprovado |
| `cargo test --workspace --exclude vertice-desktop` | 186 testes aprovados: 121 no domínio, 17 de contrato e 48 nas demais suítes |
| `cargo run -p vx-api --example generate_content_contracts -- --check` | Aprovado |
| `pnpm -C apps/web check` | Sem erros ou avisos |
| `pnpm -C apps/web lint` | Aprovado, incluindo TypeScript gerado |
| `pnpm -C apps/web test` | 61 testes aprovados |
| `pnpm wasm:build` | Build WASM aprovado |
| `pnpm -C apps/web build` | Build estático aprovado |
| `git diff --check` | Aprovado |
| Links locais da documentação alterada | 165 destinos conferidos em 11 arquivos |

Os testes cobrem:

- Exemplos mínimos e round-trip das nove variantes entre DTO, domínio e JSON.
- Referências com `javascript:`, `data:`, `file:`, host ausente, título/licença vazios,
  identificação insuficiente e campos desconhecidos ou de tipos incorretos.
- DOI, arXiv ou ISBN como alternativa ao link; licença restritiva da referência sem
  relicenciar o exercício.
- Material privado e licenças incompatíveis impedidos de publicação, inclusive em pack público.
- Partes inexistentes, documentos ausentes, duplicatas e UUIDs equivalentes com letras
  maiúsculas/minúsculas, tanto na resolução quanto na detecção de duplicatas.
- Tolerâncias, intervalos, mínimo de 12 amostras, escolhas e associação estruturalmente válidas.
- `ordered: true` e linhas vazias rejeitados também pelo schema; strings soltas e coleções
  recursivas recusadas como elementos escalares.
- Caminhos como `parts[0].answer.values[0].value`, inclusive com prefixo de pack.
- Preservação de inteiros nos parâmetros técnicos, sem convertê-los silenciosamente em
  ponto flutuante; execução de `calculus.sample` com o request depois do round-trip.
- JSON técnico não finito rejeitado; `verify`, `evidence`, vínculos `material` e parâmetros
  de aleatorização não suportados recusados em vez de ignorados.
- Ausência de `serde`, `schemars` e `ts-rs` nos manifests das camadas internas.

As primeiras execuções encontraram divergências de schema, perda de caminhos em enums
internamente tagged e perda da representação inteira de números JSON. Foram corrigidas e
receberam testes antes dos resultados finais acima. Nenhuma regra de lint foi relaxada.

## Limites e retomada

Sem execução interativa em navegador ou desktop, nem build/instalador desktop nesta unidade.
Não foram executados testes WASM das novas fachadas de conteúdo: o build comprova compilação
nesse alvo, não paridade de comportamento. A validação funcional dos contratos rodou no host.
Os avisos de empacotamento sobre LICENSE no WASM e fallback estático não impediram os builds.

Não foram implementados `assessment.check`, compilação de autoria, catálogo, armazenamento,
ZIP, sanitização de renderização, evidência, selo, Biblioteca, FSRS ou UI. Não foram executados
aceites de listas privadas nem auditoria de segurança das novas dependências. Nada foi
publicado, commitado ou enviado ao remoto; alterações anteriores foram preservadas.

Próxima unidade: **F1.2**, dependente deste contrato. F1.3 continua dependente de F1.1 e F1.2.
