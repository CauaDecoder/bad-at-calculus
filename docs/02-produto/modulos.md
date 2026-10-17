# Módulos

A Vértice é organizada em **módulos de produto** (áreas da navegação) e em **disciplinas** (o
conteúdo). O Laboratório, os Exercícios e a Biblioteca são transversais a todas as disciplinas: uma
disciplina nova ganha visualizações no Laboratório, exercícios no banco e material (notas,
roteiros, fontes abertas) na Biblioteca. O ciclo é **estudar → ver → praticar → revisar**, e a
ligação entre os módulos é a [taxonomia](taxonomia.md) de habilidades.

Legenda: ✅ pronto · 🟡 parcial · 🔜 planejado (fase do [roadmap](roadmap.md) entre parênteses)

## Módulos de produto

| Módulo | Rota base | O que faz | Status |
| ------ | --------- | --------- | ------ |
| **Laboratório** | `/lab` | Visualizações interativas por disciplina (o antigo Math View) | ✅ Cálculo e VGA; rotas e redirecionamentos estáticos em `/lab` (F0.4) |
| **Exercícios** | `/exercicios`, `/listas` | Banco, listas, resolver com correção automática, dicas, solução em passos, "Abrir no laboratório" | 🔜 (F1) — [PRD](prd-exercicios.md) |
| **Biblioteca** | `/biblioteca` | Material de estudo: papers e vídeos (link + metadados), livros abertos, notas de aula, arquivos próprios (privados, só no dispositivo), **roteiros de leitura**; leitor de PDF com destaques que viram cartões de revisão; selo de confiança, licença e status (ativo, corrigido, retirado) por item; tudo ligado a habilidades | 🔜 referências nos exercícios (F1) · biblioteca local (F2) · nuvem privada (F3) · compartilhar conteúdo autoral (F4) — [PRD](prd-biblioteca.md) |
| **Autoria** | `/autoria` | Criar/editar exercício com preview; importar em lote (pack, CSV, gabarito separado); importar PDF com IA | 🔜 editor e pack (F1) · PDF com IA (F2) |
| **Revisão** | `/revisao` | Repetição espaçada (FSRS) dos itens errados ou marcados, e dos **destaques da Biblioteca** que viram cartão; **caderno de erros** | 🔜 (F2) |
| **Plano de estudos** | `/plano` | Plano com data de prova, checklist por dia e simulados (inspirado no fluxo real de estudo de POO) | 🔜 (F2) |
| **Progresso** | `/progresso` | Domínio por habilidade, histórico, sequência de estudos | 🔜 local (F2) |
| **Conta e comunidade** | `/conta`, `/grupos` | Login, sync, listas compartilhadas, turmas, publicação de packs e de material **autoral** (com denúncia e moderação) | 🔜 (F3–F4) |

Taxonomia de disciplinas, tópicos e habilidades: [taxonomia.md](taxonomia.md).

## Laboratório — o que já existe

Cada tópico é uma fatia vertical: regras em `vx-domain/src/<assunto>`, casos de uso em
`vx-application/src/use_cases`, comando em `vx-api`, estado em `apps/web/src/lib/application` e
telas em `apps/web/src/lib/ui/<assunto>`.

### Cálculo

| Tópico | Rota atual | O que tem |
| ------ | ---------- | --------- |
| ✅ Limites e continuidade | `/lab/calculo/limites` | Limites laterais e no infinito, tabela de aproximação, bolinhas abertas/fechadas, três condições de continuidade, tipo de descontinuidade |
| ✅ Derivadas | `/lab/calculo/derivadas` | Tangente arrastável, `f'(x₀)`, traçado de `f'`, derivada simbólica com cada regra, `f''`, pontos críticos |
| ✅ Integrais | `/lab/calculo/integrais` | Área com sinal, somas de Riemann, primitiva e TFC |
| 🔜 Funções | — | Domínio, composição, inversa (Lista 01) |
| 🔜 Técnicas de integração | — | Substituição e partes com passos |

Todas as páginas aceitam funções por partes: `{x < 1: x^2; x >= 1: 2x}`.

### Vetores e Geometria Analítica

| Tópico | Rota atual | O que tem |
| ------ | ---------- | --------- |
| ✅ Operações com vetores | `/lab/vetores/operacoes` | Vista 3D; escalar, vetorial, normas, ângulo, projeção, produto misto |
| ✅ Retas e planos | `/lab/vetores/retas-e-planos` | Plano por 3 pontos, reta paramétrica, distâncias |
| ✅ Matrizes | `/lab/vetores/matrizes` | Determinante, inversa, posto, escalonamento, autovalores |
| ✅ Sistemas lineares | `/lab/vetores/sistemas` | Gauss–Jordan passo a passo, SPD/SPI/SI |
| 🔜 Cônicas e quádricas | — | Identificação, elementos e gráfico |
| 🔜 Transformações lineares | — | Mudança de base, efeito no plano |

Desde a F0.4, as URLs antigas `/calculo/...` e `/vetores/...` redirecionam no build estático,
preservando parâmetros e fragmentos ([ADR 0006](../03-arquitetura/adr/0006-plataforma-vertice.md)).
O [relatório da navegação](../05-engenharia/implementacao/f0-4-navegacao.md) registra os testes
e a falha 3D preexistente, que permanece fora do escopo dessa mudança.

## Disciplinas planejadas (Laboratório + Exercícios)

### Exatas
- **Física I** — cinemática `s(t)`, `v(t) = s'(t)`, `a(t) = v'(t)` sincronizados; projéteis; vetores força. (F2)
- **Métodos numéricos** — bisseção e Newton-Raphson animados; Euler e RK4; erro em log-log. (F2)
- **EDOs** — campo de direções e soluções. (F2+)
- **Cálculo multivariável** — superfícies 3D com WebGL (Threlte). (futuro)

### Computação (ADS)
- **Programação Orientada a Objetos (Java)** — exercícios de código com testes, UML gerado a partir
  das classes, caderno de erros. Java no navegador ainda é um ponto aberto (ver
  [estratégia de IA e execução de código](../08-ia/estrategia-ia.md#execução-de-código)). (F2–F3)
- **Algoritmos e estruturas de dados** — execução passo a passo de ordenação, busca, pilha/fila,
  árvores; gráfico de crescimento O(n). (F2)
- **Banco de dados** — SQL executado no navegador (sqlite-wasm/PGlite) com diagrama ER e resultado
  comparado ao esperado. (F2)
- **Redes** — calculadora e visualização de sub-redes. (futuro)

### Arquitetura e Urbanismo
- **Estruturas I** — forças aplicadas, decomposição e resultante (reaproveita o motor de vetores),
  equilíbrio, reações de apoio, treliças, diagramas de esforço cortante e momento fletor; sistemas
  estruturais. (F2, com a colaboradora de AU)
- **Geometria descritiva** — épura de Monge sincronizada com a vista 3D; verdadeira grandeza. (F2+)
- **Conforto ambiental** — carta solar e insolação por latitude e data. (futuro)
- **Desenho e escalas** — conversões de escala, áreas e perímetros. (futuro)

## Transversal
- ✅ Editor de expressões com LaTeX ao vivo e ajuda "Como digitar".
- ✅ Exemplos clicáveis em cada página (vários tirados das listas de Cálculo A).
- 🔜 Command palette (`Ctrl/⌘ + K`), exportação PNG/SVG, workspaces (F0–F1).
- 🔜 **Registro de visualizações**: toda visualização tem `id`, parâmetros tipados e URL com estado,
  para poder ser embutida num exercício (F1, [ADR 0008](../03-arquitetura/adr/0008-formato-canonico-de-exercicio.md)).
