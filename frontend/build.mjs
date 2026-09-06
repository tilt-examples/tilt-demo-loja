import { mkdirSync, writeFileSync } from "node:fs";
import { paginaProdutos } from "./src/carrinho.mjs";

mkdirSync("dist", { recursive: true });
const produtos = [
  { sku: "CAM-01", nome: "Camiseta", preco_centavos: 5990 },
  { sku: "CAN-02", nome: "Caneca", preco_centavos: 3450 },
  { sku: "BON-03", nome: "Bone", preco_centavos: 7900 },
  { sku: "MOC-04", nome: "Mochila", preco_centavos: 12900 },
];
writeFileSync("dist/index.html", paginaProdutos(produtos));
console.log("dist/index.html gerado");
