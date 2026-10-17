# Ícone provisório do desktop

Ícone simples e autoral, com fundo neutro e a letra V. Sua criação foi autorizada
pelo mantenedor para substituir o ícone ausente e desbloquear a validação do build
desktop na F0.2. Não representa a identidade visual final da Vértice.

Fonte editável: `icon.svg`. Para reproduzir os arquivos gerados, execute na raiz
do repositório:

```powershell
corepack pnpm -C apps/desktop exec tauri icon src-tauri/icons/icon.svg --output src-tauri/icons
```

O comando gera `icon.png` RGBA, `icon.ico` para Windows e os demais formatos e
tamanhos padrão do Tauri. Nesta entrega, somente `icon.png` e `icon.ico` são
versionados, pois são os arquivos declarados na configuração desktop atual.
Não altera a UI nem define uma identidade visual final.
