# Spike de CORS da API do arXiv

Data: 2026-10-15. Papel: `software-architect`.
Decisão relacionada: [ADR 0014, §8](../03-arquitetura/adr/0014-biblioteca.md#8-citação-e-consulta-de-metadados).

## Resultado

**NOT VERIFIED.** A hipótese de que o frontend estático consegue ler os metadados da API do
arXiv por `fetch` entre origens não se confirmou. A API respondeu HTTP 200 no cliente HTTP,
mas o Chromium rejeitou a leitura por ausência de `Access-Control-Allow-Origin`.

Na F2, o preenchimento de metadados arXiv na web será manual. No desktop, a arquitetura mantém
a consulta por comando Tauri com hosts permitidos; este spike não implementa nem testa esse
comando. Na F3, a consulta pelo servidor continua prevista. Crossref não foi testado.

## Hipótese e critério

Hipótese: uma página de origem local, com a segurança padrão do navegador ativa, consegue
executar um GET HTTPS para a API pública e ler uma entrada Atom sem proxy.

O critério de sucesso exige HTTP 200 e uma entrada Atom legível em JavaScript. Apenas abrir a
URL em uma aba, obter resposta por `curl` ou receber resposta opaca em `no-cors` não atende ao
critério. O spike usa `mode: 'cors'`, sem credenciais e sem referenciador.

Endpoint público de teste, com um único identificador:

```text
https://export.arxiv.org/api/query?id_list=2401.01234&max_results=1
```

O [manual oficial](https://info.arxiv.org/help/api/user-manual.html) descreve a consulta por
identificador e a resposta Atom. O exemplo de navegação direta do manual não estabelece que
um site de outra origem possa ler a resposta via JavaScript.

## Evidência observada

Execução no Chromium `151.0.7922.34`, em 2026-10-15 às 13:32:46 UTC, 10:32:46 em São Paulo.
A página foi servida em `http://localhost:54948`, sem CSP restritiva nem desativação de CORS.
Os controles usam o mesmo servidor via `127.0.0.1`, uma origem diferente.

| Caso | Resultado esperado | Resultado observado |
| --- | --- | --- |
| Controle com `Access-Control-Allow-Origin: *` | JavaScript lê HTTP 200 | HTTP 200, `response.type = cors`, corpo legível |
| Controle sem cabeçalho CORS | Navegador bloqueia | `TypeError: Failed to fetch`; `MissingAllowOriginHeader` |
| GET para o arXiv | HTTP 200 e entrada Atom legível | `TypeError: Failed to fetch`; `MissingAllowOriginHeader` |

Mensagem do navegador para o arXiv:

```text
No 'Access-Control-Allow-Origin' header is present on the requested resource.
```

Uma consulta HTTP separada, com `Origin: http://localhost:5173`, recebeu `HTTP/1.1 200 OK` e
`content-type: application/atom+xml; charset=utf-8`, sem `Access-Control-Allow-Origin`.
Isso distingue a disponibilidade do endpoint da permissão de leitura entre origens. O host
ficou inacessível por DNS no sandbox restrito; as medições externas foram feitas após aprovação
da ferramenta para acesso à rede. O bloqueio CORS foi observado em uma requisição que alcançou
o serviço, não inferido dessa falha de DNS.

A [saída JSON da execução](spikes/arxiv-cors-result.json) conserva os resultados dos controles,
os erros CDP e as mensagens do navegador. Nenhum documento privado foi enviado.

## Reprodução

Use Node.js, Playwright e Chromium já instalados no ambiente. O spike não adiciona dependências
ao aplicativo. Se Playwright não estiver no caminho de módulos, defina `PLAYWRIGHT_MODULE` com
o caminho absoluto da instalação disponível.

Na raiz do repositório:

```powershell
node docs/09-pesquisa/spikes/arxiv-cors.cjs
```

O [script](spikes/arxiv-cors.cjs) abre uma página local, testa os controles e faz uma consulta ao
arXiv. Usa porta efêmera e encerra o navegador e o servidor ao terminar.

Para inspecionar os cabeçalhos sem o navegador:

```powershell
curl.exe --silent --show-error --location --max-time 25 --dump-header - --output NUL --header 'Origin: http://localhost:5173' 'https://export.arxiv.org/api/query?id_list=2401.01234&max_results=1'
```

Respeite intervalo mínimo de três segundos entre consultas ao arXiv e uma conexão por vez,
conforme os [termos oficiais da API](https://info.arxiv.org/help/api/tou.html#rate-limits).

## Limites e implementação futura

O resultado vale para o endpoint, origem e navegador medidos nesta data. A política do serviço
pode mudar. Refaça o teste antes de oferecer preenchimento automático arXiv na web.

O futuro adapter desktop deverá montar a URL a partir do identificador validado e restringir
HTTPS, host, caminho e redirecionamentos ao serviço permitido. A consulta continua dependendo
de ação explícita do usuário, com o serviço nomeado e somente o identificador enviado, conforme
o ADR 0014. Erros de rede mantêm o preenchimento manual disponível.

A revisão de segurança T7 permanece para antes da F2. `references[]`, evidência, tipos Rust,
leitor de documentos e comando Tauri não foram implementados nesta sessão, conforme o escopo
escolhido pelo mantenedor.
