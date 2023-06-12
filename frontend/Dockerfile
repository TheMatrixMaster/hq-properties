### Build Step
FROM node:16 as builder

RUN npm install -g pnpm

WORKDIR /usr/src/app
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

### Serve Step
FROM node:16-alpine
 
WORKDIR /app
COPY --from=builder /usr/src/app/build .
COPY --from=builder /usr/src/app/package.json .
COPY --from=builder /usr/src/app/node_modules ./node_modules
 
EXPOSE 3000
CMD ["node", "-r", "dotenv/config", "build"]
