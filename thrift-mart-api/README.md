# thrift-mart-api

The backend API for **Thrift Mart** — a crypto-powered marketplace on Stellar
(XLM/USDC) for selling pre-loved items at affordable prices.

## Tech stack

- [Express](https://expressjs.com) 5
- [TypeScript](https://www.typescriptlang.org) (NodeNext ESM)
- [tsx](https://tsx.is) for local development

## Getting started

```bash
npm install
cp .env.example .env
npm run dev
```

The server starts on `http://localhost:4000` (configurable via `PORT`).
Verify it's running:

```bash
curl http://localhost:4000/api/health
```

## Scripts

- `npm run dev` — start the dev server with hot reload (`tsx watch`)
- `npm run build` — type-check and compile to `dist/`
- `npm run start` — run the compiled build (`dist/server.js`)
- `npm run typecheck` — type-check without emitting output

## Project structure

```
src/
├── config/         # Environment configuration
├── middlewares/     # Express middleware (error handling, 404s, ...)
├── routes/          # Route definitions, mounted under /api
├── utils/           # Shared utilities (e.g. HttpError)
├── app.ts           # Express app assembly (middleware + routes)
└── server.ts         # Entry point — starts the HTTP server
```

Routes are grouped by resource under `src/routes/` and mounted onto the
shared `apiRouter` in `src/routes/index.ts`. Follow the pattern in
`health.routes.ts` when adding new endpoints, and throw `HttpError` (from
`src/utils/http-error.ts`) for expected client-facing errors — the central
`errorHandler` middleware will format the response.

## Environment variables

See `.env.example`:

| Variable   | Description                        | Default       |
| ---------- | ----------------------------------- | ------------- |
| `NODE_ENV` | `development` \| `production`       | `development` |
| `PORT`     | Port the HTTP server listens on     | `4000`        |

## Related

- [`thrift-mart-web`](../thrift-mart-web) — web frontend
- [`thrift-mart-contracts`](../thrift-mart-contracts) — Soroban smart contracts
