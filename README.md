# api.ajr.dev

Cloudflare worker to respond to requests to api.ajr.dev

### Deployment

Make sure `wrangler` CLI is authenticated and `account_id` in `wrangler.toml` is replaced with your Cloudflare account id.

Then run:

```
$ wrangler publish
```

### Usage
This template starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this project to use.

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ npm run dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npm run deploy
```
