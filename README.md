# api.ajr.dev

Cloudflare worker to respond to requests to api.ajr.dev

### Local dev

Make sure you follow the rust wasm guide and get the `wrangler` npm package installed:
- https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/
- https://developers.cloudflare.com/workers/wrangler/install-and-update/

Run the following command to start a local server:

```sh
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ npm run dev
```

At time of writing, the following are broken for me:
- Hot reloading on save (at least on M1 mac)
- R2 usage locally

### Deployment

Wrangler CLI will handle authenticating you with your cloudflare account. You just need to run:

```sh
# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npm run deploy
```
