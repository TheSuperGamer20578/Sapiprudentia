FROM node:19 AS builder

WORKDIR /frontend

COPY package.json .
RUN npm install

COPY . .
RUN npm run build
RUN npm prune --production

FROM node:19

ENV NODE_ENV=production
WORKDIR /sapiprudentia

COPY --from=builder /frontend/build ./build
COPY --from=builder /frontend/package.json ./package.json
COPY --from=builder /frontend/node_modules ./node_modules

CMD ["node", "build"]
