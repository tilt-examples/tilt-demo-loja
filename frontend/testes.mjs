import assert from "node:assert/strict";
import { formataCentavos, paginaProdutos, somaCarrinho } from "./src/carrinho.mjs";

const casos = [
  ["formata centavos", () => assert.equal(formataCentavos(5990), "R$ 59,90")],
  ["formata zero", () => assert.equal(formataCentavos(0), "R$ 0,00")],
  ["formata milhar", () => assert.equal(formataCentavos(123456), "R$ 1.234,56")],
  ["soma vazia", () => assert.equal(somaCarrinho([]), 0)],
  ["soma itens", () => assert.equal(somaCarrinho([{ precoCentavos: 3450, qtd: 2 }, { precoCentavos: 7900, qtd: 1 }]), 14800)],
  ["pagina lista os skus", () => {
    const html = paginaProdutos([{ sku: "CAM-01", nome: "Camiseta", preco_centavos: 5990 }]);
    assert.match(html, /data-sku="CAM-01"/);
    assert.match(html, /R\$ 59,90/);
  }],
];
let falhas = 0;
for (const [nome, fn] of casos) {
  try { fn(); console.log(`ok   ${nome}`); } catch (e) { falhas++; console.log(`FALHA ${nome}: ${e.message}`); }
}
console.log(`${falhas} falha(s)`);
process.exit(falhas === 0 ? 0 : 1);
