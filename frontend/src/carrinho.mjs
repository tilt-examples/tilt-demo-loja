// Dominio do frontend da Loja: formatacao e soma do carrinho. Sem dependencias -- o build e o
// teste rodam com o Node da imagem, e o que o pipeline prova e a coreografia, nao o framework.
export function formataCentavos(centavos) {
  const reais = Math.floor(centavos / 100);
  const cents = String(centavos % 100).padStart(2, "0");
  return `R$ ${reais.toLocaleString("pt-BR")},${cents}`;
}

export function somaCarrinho(itens) {
  return itens.reduce((acc, { precoCentavos, qtd }) => acc + precoCentavos * qtd, 0);
}

export function paginaProdutos(produtos) {
  const linhas = produtos
    .map((p) => `<li data-sku="${p.sku}">${p.nome} — ${formataCentavos(p.preco_centavos)}</li>`)
    .join("\n");
  return `<!doctype html><html lang="pt-BR"><body><h1>Loja</h1><ul>\n${linhas}\n</ul></body></html>`;
}
