FROM docker.io/library/node:22-slim
WORKDIR /app
COPY package.json build.mjs testes.mjs ./
COPY src ./src
RUN node build.mjs
EXPOSE 80
CMD ["node", "-e", "require('http').createServer((q,r)=>{r.setHeader('content-type','text/html');r.end(require('fs').readFileSync('dist/index.html'))}).listen(80)"]
