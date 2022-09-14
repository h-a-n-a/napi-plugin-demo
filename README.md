## Structure

```
- node: napi plugin runner
- plugin-1: napi-plugin1
- plugin-2: napi-plugin2
```

## How to run

```bash
npx pnpm@6 install
npx pnpm@6 run build:node
node example/index.js
```