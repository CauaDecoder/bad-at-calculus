# F1.2 — Correção de respostas

- **Papel:** `backend-developer`.
- **Estado:** concluída no escopo de motor/contratos da F1.2, com limites de verificação abaixo.
- **Dependência:** F1.1 entregue, conforme o [registro de execução](progresso.md).

## Escopo do primeiro grupo (histórico)

As seções iniciais registram o recorte da primeira sessão. A retomada ao final deste documento
amplia esse contrato para os nove tipos e `form`; os limites novos ali descritos prevalecem.

O domínio compara números e expressões sem serialização ou parser concreto. O caso de uso
`CheckAnswer` recebe a especificação pública do `assessment`, a identidade do exercício e a
resposta. O port `ExpressionParser` permite testes com fakes; `vx-api` injeta `PrattParser`.
O comando `assessment.check` está registrado no `dispatch` e em `commands.ts`, sem UI nova.

Arquivos principais:

- `crates/vx-domain/src/assessment/comparison.rs` e `comparison_tests.rs`;
- `crates/vx-domain/src/expression/{mod,piecewise}.rs`: avaliação com múltiplas variáveis,
  preservando os métodos anteriores;
- `crates/vx-application/src/use_cases/assessment.rs` e `assessment/tests.rs`;
- `crates/vx-api/src/assessment_check.rs`, `contracts.rs` e `lib.rs`;
- `crates/vx-api/tests/assessment_check.rs` e `fixtures/assessment-cases.json`;
- `crates/vx-api/examples/assessment_fixture_reports.rs` e
  `scripts/check-assessment-parity.mjs`;
- schemas `contracts/assessment-check-{request,response}.schema.json`, tipos gerados em
  `apps/web/src/lib/domain/generated/content.ts` e registro do comando no front.

Não há nova dependência, alteração de lockfile, persistência, player, pontuação ou evidência.
Os tipos canônicos e validadores estruturais da F1.1 continuam aceitando os nove tipos.
O contrato executável anuncia somente as duas variantes implementadas. Outros tipos não
recebem um veredito automático por acidente.

## Contrato executável

Exemplo de request para `assessment.check`:

```json
{
  "exerciseId": "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90",
  "answer": {
    "type": "expression",
    "expected": "x^2-1",
    "variables": ["x"],
    "domain": { "x": [-5, 5] }
  },
  "response": "(x-1)(x+1)"
}
```

`answer` reutiliza os campos canônicos de `numeric` ou `expression`. `response` é texto, não
uma segunda especificação. O chamador fornece uma identidade estável; em produção, usa o ID
do exercício. Ela não é usada para consultar armazenamento. O caso de uso também permite IDs
de fixtures, com 1 a 128 bytes, não vazios. UUIDs com letras hexadecimais em caixas diferentes
produzem a mesma semente.

A resposta contém `status`, `sampling`, `feedback` e `inputError`. Os três últimos campos são
`null` quando não se aplicam:

- `correct`: os valores coincidem dentro da tolerância; em expressão, somente no domínio
  avaliado, com o mínimo de amostras válidas;
- `incorrect`: existe divergência matemática ou a resposta não é finita onde a esperada é;
- `indeterminate`: o orçamento termina sem o mínimo de pontos válidos;
- `invalid_input`: sintaxe inválida, variável não declarada ou limite de entrada excedido.
  `inputError.field` é `response`; `position` usa índice de caracteres quando disponível.
  Esse estado não deve contar como tentativa. Não há contador de tentativas nesta entrega.

Erro na especificação ou em `feedback[].matches` é erro de autoria: `ApiError` com
`code: "invalid-request"` e caminho do campo, não um erro atribuído ao estudante. Erros de
transporte continuam usando `bad-request`. Equações e prefixos de definição não são aceitos
como respostas escalares; o corretor não deixa o parser descartar silenciosamente `y =`.

Números aceitam frações, raízes, constantes e vírgula decimal pelo parser existente. `unit` é
opcional: a resposta pode omitir a unidade ou incluir exatamente o rótulo cadastrado, separado
por espaço (`3 m/s`). Não há conversão, inferência nem álgebra de unidades.

O feedback usa a mesma comparação da resposta correta, com a mesma semente e domínio. A resposta
correta tem precedência; só uma resposta incorreta recebe a primeira mensagem equivalente
da lista. Mensagens permanecem Markdown não confiável; esta unidade não as renderiza.

## Amostragem e limites

- Tolerância padrão: absoluto `1e-12`, relativo `1e-9`; campos informados sobrepõem os padrões.
- Intervalo padrão por variável: `[-10, 10]`. Intervalos explícitos substituem o padrão
  somente nas variáveis informadas.
- Semente: FNV-1a de 32 bits dos bytes da identidade em minúsculas ASCII. Gerador: LCG de
  32 bits, multiplicador `1664525`, incremento `1013904223`, com aritmética wrapping.
  Variáveis seguem ordem lexicográfica. Não se usa relógio nem RNG de plataforma.
- Padrão de 12 pontos pseudoaleatórios válidos; requests de 12 a 256 pontos. O orçamento
  é dez vezes o número solicitado. Pontos não finitos da esperada são descartados, nunca
  pontos onde somente a resposta do estudante falha.
- Antes dos pontos pseudoaleatórios há cinco sondas determinísticas: zero, quando pertence
  ao intervalo, ou centro; extremos; primeiro e terceiro quartis. As coordenadas de cada
  sonda são escolhidas simultaneamente para todas as variáveis. As sondas podem reprovar,
  mas não contam no mínimo de amostras válidas.
- `sampling` registra `seed`, intervalos efetivos em `domain`, `requested`, `valid` e
  `discarded`. Os contadores cobrem somente as tentativas pseudoaleatórias; uma divergência
  nas sondas pode retornar `incorrect` com ambos em zero.
- A AST não é simplificada antes de avaliar. Assim, resposta `x/x` contra esperada `1`
  falha na sonda zero; esperada `x/x` contra resposta `1` só confirma concordância onde a
  esperada é finita. Ambas coincidem se o intervalo explícito exclui zero.
- `sqrt(x^2)` contra `x` falha no domínio com negativos e passa no intervalo `[0, 5]`.
  Esperada sem pontos válidos, válida apenas em ponto isolado ou com amostragem insuficiente
  retorna `indeterminate`, não `correct` por comparação vazia.
- Limites operacionais antes do parser: 4096 bytes e 128 caracteres não brancos por expressão;
  até oito variáveis e 16 entradas de feedback por request. Eles limitam esta operação, sem
  mudar a validade estrutural do conteúdo da F1.1. Limites excedidos são erros explícitos.
  A proteção não corrige as outras operações do parser registradas na F0.6.

Amostragem **não é prova de identidade** nem prova de igualdade dos domínios. As sondas capturam
casos comuns, mas não descobrem todos os polos ou diferenças em regiões não visitadas. O resultado
não produz selo, `verify`, `evidence` ou lock da F1.4.

## Reprodução

```bash
cargo run -p vx-api --example generate_content_contracts
cargo run -p vx-api --example generate_content_contracts -- --check
cargo fmt --all -- --check
cargo clippy --workspace --exclude vertice-desktop --all-targets -- -D warnings
cargo test --workspace --exclude vertice-desktop
corepack pnpm wasm:build
node scripts/check-assessment-parity.mjs
corepack pnpm -C apps/web check
corepack pnpm -C apps/web lint
corepack pnpm -C apps/web test
corepack pnpm -C apps/web build
```

O script de paridade executa o exemplo nativo e o binding WASM web real no Node, compara as
respostas completas das mesmas fixtures autorais e exercita limites de entrada no WASM.
Não é um teste interativo de navegador ou Tauri.

## Verificação desta sessão

Em 2026-10-15, no Windows x64:

- Linha de base: 186 testes Rust aprovados antes da implementação.
- Suite ampliada: 210 testes Rust aprovados, incluindo nove testes novos de domínio, sete
  de aplicação com fakes e oito de contrato/integração com parser real.
- `fmt --check`, Clippy com `-D warnings`, geração e check de contratos aprovados. O primeiro
  Clippy apontou `unwrap` nos testes novos; os testes passaram a propagar erros, sem afrouxar
  as regras do workspace.
- WASM web real reconstruído; 27 fixtures autorais produziram respostas completas iguais no
  `dispatch` nativo e no binding WASM executado no Node. Entradas recursivas nos limites
  aceitos e entradas grandes recusadas não causaram trap nessa operação.
- `check` web sem erros ou avisos; lint, 61 testes web e build estático aprovados.
- `git diff --check` aprovado; os avisos de conversão LF/CRLF do checkout não são falhas do check.
  A consulta adicional com `-c core.autocrlf=false` reprovou ao tratar CRLF preexistente como
  whitespace; a configuração não foi persistida e a execução padrão passou novamente.
- Os 68 links locais dos cinco documentos de implementação/roadmap envolvidos foram conferidos.

Não foram executados navegador interativo, transporte Tauri, build desktop, Linux/macOS,
CI remota ou aceite com pack privado. A meta de latência do PRD não foi medida; o build
estático não comprova offline após recarga. Não houve acesso a `content-private/`, commit,
push, PR ou publicação. O trabalho anterior do checkout foi preservado.

## Pendências ao encerrar o primeiro grupo (histórico)

1. Implementar o grupo equação e coleções, incluindo casamento um a um dos conjuntos.
2. Implementar escolhas, associação e autoavaliação sem convertê-la em correção automática.
   Pontuação parcial continua dependendo da decisão de produto do PRD.
3. Implementar checagem estrutural de `form`. Nesta entrega, qualquer valor não nulo é
   recusado com erro em `answer.form`, nunca ignorado.
4. Ampliar o contrato executável, os testes e a paridade para os demais grupos. O adapter/port
   e o player do front ainda não consomem esta operação; o comando e tipos estão disponíveis.
5. Verificar o transporte desktop quando houver execução Tauri; não inferir paridade desktop
   de um teste nativo do `dispatch`.

F1.3 não deve ser tratada como desbloqueada pela entrega apenas deste grupo.

## Retomada — demais grupos e checagem de forma

O `CheckAnswer` e o comando `assessment.check` agora executam os nove tipos da F1. `form`
não é mais recusado. O discriminante e os campos de `GradableAnswer` foram ampliados de forma
aditiva; os schemas e o TypeScript foram regenerados. O request continua contendo `exerciseId`,
`answer` e `response` textual; requests escalares da primeira entrega permanecem válidos.

### Sintaxe das respostas

Usa-se ponto e vírgula entre elementos para não confundir o separador com vírgula decimal.
Parênteses, colchetes, chaves e aspas internos são respeitados pelo separador. Exemplos:

| Tipo | `response` |
| --- | --- |
| `numeric` | `1/2` ou `0,5` |
| `expression` | `(x-1)*(x+1)` |
| `equation` | `5*x-y-16=0` |
| `set` | `{-1;0;1}`, inclusive `{}` |
| `tuple` | `(0,5;-2;3)` |
| `matrix` | `[[1;0];[0;1]]` |
| `choice` | `[b]` ou `[a;c]`, inclusive `[]` como seleção vazia |
| `matching` | `{a:4;b:2;c:1;d:3}`, inclusive `{}` como associação incompleta |
| `self_graded` | Texto livre ou vazio; não é submetido ao parser matemático |

IDs que contêm separadores, delimitadores ou aspas devem usar aspas duplas: `["a;b"]` e
`{"a:b":"1;2"}`. Em IDs entre aspas, `\"`, `\\`, `\n`, `\r` e `\t` representam os
caracteres correspondentes. Unicode literal é preservado; escapes desconhecidos são recusados.
Os exemplos da tabela são conteúdo da string `response`, não objetos JSON soltos.

Equações exigem exatamente uma igualdade externa. Cada lado passa pelo mesmo parser e validação
de variáveis; erros do lado direito preservam a posição na string completa. Igualdades em condições
de funções por partes não são confundidas com a igualdade externa.

Em coleções, erros de entrada identificam `response.values[i]` ou `response.rows[i][j]`; a posição,
quando disponível, continua relativa à string `response` completa. Matriz vazia, linha vazia e
linhas irregulares são entrada inválida. Dimensões diferentes do gabarito, mas matematicamente
bem formadas, são resposta incorreta. Uma parte não recebe um veredito incorreto antes de suas
entradas serem verificadas: erro de sintaxe continua distinto de erro matemático.

### Regras implementadas

- **Equação:** compara os resíduos sem simplificar a AST. A razão entre resíduo submetido e
  esperado precisa ser constante, finita e diferente de zero. A razão é normalizada em torno de
  um, para que fatores muito pequenos não escondam variação sob a tolerância absoluta. Pontos
  onde o resíduo esperado é zero não contam no mínimo de 12 pontos, mas uma divergência nesse
  ponto pode reprovar. Identidade com resíduo esperado sempre zero retorna `indeterminate`.
  A regra do ADR 0009 não equivale a resolver e comparar conjuntos de soluções: multiplicadores
  não constantes são recusados mesmo quando preservam algumas soluções.
- **Conjunto:** constrói as correspondências escalares e procura uma bijeção por caminhos
  aumentantes. Não reutiliza elementos e não depende de uma escolha gulosa sob tolerâncias
  sobrepostas. Retorna `indeterminate` se uma bijeção só é possível usando comparações
  indeterminadas; retorna `incorrect` se nem essas comparações completam a correspondência.
- **Tupla/matriz:** compara componentes em ordem, com o tipo, variáveis, tolerância, domínio e
  forma do respectivo `ScalarAnswerSpec`. Uma divergência conhecida prevalece sobre componentes
  indeterminados; sem divergência, qualquer componente indeterminado impede `correct`.
  O campo agregado `sampling` fica `null`; não representa todos os domínios internos como se
  fossem um único domínio. A evidência por escopo continua na F1.4.
- **Escolhas:** compara conjuntos de IDs. IDs desconhecidos, IDs repetidos e múltiplas seleções
  numa escolha única são entrada inválida, não uma tentativa matemática incorreta.
- **Associação:** compara todos os pares, sem considerar a ordem textual. IDs desconhecidos e
  chaves repetidas são entrada inválida; pares ausentes ou errados são `incorrect`.
  `partialCredit` continua sendo metadado: não há nota, fórmula ou veredito parcial inventado.
- **Autoavaliação:** retorna o novo status `self_graded`, sempre sem feedback automático,
  amostragem ou erro de sintaxe matemática. Nem o texto `acertei` nem coincidência com uma
  mensagem de feedback produz `correct`. A futura UI recolhe a avaliação humana.

### Formas estruturais

As três formas são checadas na AST submetida, somente depois de confirmar equivalência:

- `factored`: recusa soma/subtração externa e função por partes externa; aceita termo único,
  produto ou potência. Negações externas são transparentes. Quocientes exigem numerador e
  denominador fatorados segundo essa regra. Não há promessa de fatoração completa em fatores
  irredutíveis nem descoberta de fatores por CAS.
- `expanded`: recusa produto com operando aditivo e potência inteira maior que um de base
  aditiva, percorrendo também subexpressões. Expoente não constante de base aditiva é
  `indeterminate`, pois a regra estrutural não consegue decidir a distribuição.
- `no_negative_exponents`: recusa qualquer expoente constante negativo, inclusive `-2/2`;
  aceita divisão sem expoentes negativos. Expoente cujo sinal não é decidível como constante
  retorna `indeterminate`. Não há inferência de sinal a partir do intervalo de amostragem.

Uma violação estrutural conhecida prevalece sobre outra subexpressão indeterminada, inclusive
em ramos de função por partes. As formas não reescrevem a resposta para fazê-la passar.

### Feedback e limites adicionais

O feedback das novas variantes usa a mesma sintaxe e o mesmo corretor da variante. Resposta
correta continua tendo precedência. Para coleções, cada elemento de `feedback[].matches` usa
a configuração escalar da posição correspondente no gabarito; por isso o feedback precisa
ter a mesma quantidade de elementos e, em matrizes, a mesma forma. Diferença de forma é erro
de autoria, sem inferir novos tipos escalares. A resposta do estudante a um conjunto pode
continuar em qualquer ordem. Feedback dos elementos escalares é validado, mas este comando
retorna a mensagem de feedback da coleção, não uma lista de mensagens por componente.

Além dos limites escalares anteriores: até 32 elementos por conjunto/tupla/linha; até 32
linhas e 64 elementos totais por matriz; até 32 opções por escolha ou lado da associação;
IDs de opção com até 512 bytes; entrada composta com até 65536 bytes e 128 níveis de
delimitadores. Limites de autoria geram `invalid-request` por campo; limites de resposta
geram `invalid_input`. Esses limites operacionais não alteram a validação estrutural da F1.1.

### Arquivos acrescentados na retomada

- `crates/vx-domain/src/assessment/{collections,forms,grading_tests}.rs` e extensão do amostrador;
- `crates/vx-application/src/use_cases/assessment/{input,grading}.rs`, com extensão do caso de uso;
- contratos executáveis, testes de API e as fixtures compartilhadas, agora com 71 cenários;
- `apps/desktop/src-tauri/src/commands_tests.rs` e feature `tauri/test` somente em dependência
  de desenvolvimento, sem nova versão ou crate externa. O teste passa pelo IPC do Tauri com
  `MockRuntime`, registra o comando real `engine` e compara respostas com `dispatch` nativo.

O player, seu estado e o port/adapter de consumo da nova operação pertencem à F1.7. Não são
pendências de correção da F1.2, nem foram implementados nesta retomada. Compilador e evidência
continuam nas F1.3/F1.4. Não houve avanço automático para essas unidades.

### Verificação da retomada

Em 2026-10-15, Windows x64:

- Linha de base: 210 testes Rust aprovados antes da retomada.
- `cargo test --workspace --exclude vertice-desktop`: **225 testes aprovados**. O incremento
  inclui oito testes puros de domínio, quatro de aplicação e três de integração/contrato.
- `cargo test -p vertice-desktop --locked`: **um teste aprovado**, executando os 71 cenários
  pelo IPC de `engine` no `MockRuntime` do Tauri e comparando as respostas com o nativo.
- `cargo fmt --all -- --check`, Clippy do motor e Clippy desktop com `--all-targets -- -D warnings`
  aprovados. O primeiro Clippy rejeitou folds de `Option` na checagem de formas; foram trocados
  por laços que preservam a precedência de uma violação conhecida sobre outra parte desconhecida.
  Não houve supressão ou relaxamento das regras.
- Geração reproduzível e `--check` de schemas/TypeScript aprovados. Testes validam requests e
  reports das fixtures contra os schemas gerados, além de caminhos de erro e posições.
- WASM web reconstruído. `node scripts/check-assessment-parity.mjs` aprovou **71 fixtures**, com
  respostas completas iguais no nativo e no WASM real, mais os testes de limites sem trap.
  Uma execução com timeout de 120 segundos não concluiu enquanto o Cargo concorria com o
  Clippy desktop; a repetição em background terminou com saída zero.
- `cargo build -p vertice-desktop --locked` aprovado no perfil debug Windows x64. A feature
  `tauri/test` é somente de desenvolvimento; o build de produção não depende do mock.
- `check` web sem erros/avisos; lint, **61 testes web**, build estático e `git diff --check`
  aprovados. Avisos preexistentes do WASM sobre arquivos de licença e do adapter estático
  sobre `index.html` não foram removidos para produzir resultado verde.

**Não verificado:** WebView real ou janela Tauri interativa, navegador interativo, release ou
instaladores desktop, Linux/macOS, CI remota, pack privado, latência do PRD e recarga offline.
O teste de IPC usa o framework real com runtime simulado; não é E2E do sistema operacional.
Esses limites não foram apresentados como testes aprovados nem substituem o aceite F1.10.

**Encerramento:** pendências de implementação da F1.2 atendidas; próxima unidade elegível F1.3,
dependente da F1.1 e F1.2 entregues. Nenhuma publicação, commit, push ou PR nesta execução.
