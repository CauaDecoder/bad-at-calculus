// Run with Node.js and Playwright available; this spike adds no project dependency.
const http = require('node:http');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

const endpoint = 'https://export.arxiv.org/api/query?id_list=2401.01234&max_results=1';

async function main() {
  const server = http.createServer((request, response) => {
    if (request.url === '/allowed') {
      response.setHeader('Access-Control-Allow-Origin', '*');
    }
    response.setHeader('Content-Type', 'text/plain');
    response.end('local CORS control');
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    const failures = [];
    const consoleErrors = [];
    page.on('console', (message) => {
      if (message.type() === 'error') consoleErrors.push(message.text());
    });
    const session = await page.context().newCDPSession(page);
    await session.send('Network.enable');
    session.on('Network.loadingFailed', (event) => {
      failures.push({ error: event.errorText, cors: event.corsErrorStatus || null });
    });
    const port = server.address().port;
    await page.goto(`http://localhost:${port}/`);

    async function probe(url) {
      return page.evaluate(async (target) => {
        try {
          const response = await fetch(target, {
            mode: 'cors', credentials: 'omit', referrerPolicy: 'no-referrer',
            signal: AbortSignal.timeout(20000),
          });
          const body = await response.text();
          const xml = new DOMParser().parseFromString(body, 'application/xml');
          return {
            readable: true, status: response.status, responseType: response.type,
            hasAtomEntry: xml.getElementsByTagNameNS('http://www.w3.org/2005/Atom', 'entry').length > 0,
          };
        } catch (error) {
          return { readable: false, error: String(error) };
        }
      }, url);
    }

    const allowedControl = await probe(`http://127.0.0.1:${port}/allowed`);
    const deniedControl = await probe(`http://127.0.0.1:${port}/denied`);
    const arxiv = await probe(endpoint);
    // Allow CDP and console events from the final request to arrive.
    await page.waitForTimeout(100);
    console.log(JSON.stringify({
      checkedAt: new Date().toISOString(), browser: browser.version(),
      origin: new URL(page.url()).origin, endpoint,
      allowedControl, deniedControl, arxiv, failures, consoleErrors,
    }, null, 2));
  } finally {
    if (browser) await browser.close();
    await new Promise((resolve) => server.close(resolve));
  }
}

main().catch((error) => { console.error(error); process.exitCode = 1; });
