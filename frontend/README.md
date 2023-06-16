# hq-properties webapp

Web application for hqproperties.ca written in sveltekit

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), create a private key and certificate using `mkcert`

```bash
mkcert -install -key-file local.key.pem -cert-file local.cert.pem localhost foo.local
```

Then move the created files to `nginx/dev/local.cert.pem` & `nginx/dev/local.key.pem`.
On Mac OS, add the keys to the system keychains in Keychain Access.

Finally, start a development server using docker compose or pnpm dev

```bash
docker compose -f docker-compose.dev.yml up

# or
pnpm dev
```

## Building

To create a production version of your app:

```bash
pnpm build
```

You can preview the production build with `pnpm preview`.
