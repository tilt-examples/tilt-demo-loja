//! API da Loja de exemplo (S-0260). `--auto-teste` roda os testes de dominio e sai com codigo:
//! e o que a etapa `teste-backend` do pipeline executa dentro da imagem construida.
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Produto {
    sku: String,
    nome: String,
    preco_centavos: u32,
}

fn catalogo() -> Vec<Produto> {
    vec![
        Produto { sku: "CAM-01".into(), nome: "Camiseta".into(), preco_centavos: 5990 },
        Produto { sku: "CAN-02".into(), nome: "Caneca".into(), preco_centavos: 3450 },
        Produto { sku: "BON-03".into(), nome: "Bone".into(), preco_centavos: 7900 },
        Produto { sku: "MOC-04".into(), nome: "Mochila".into(), preco_centavos: 12900 },
    ]
}

/// Total de um carrinho, em centavos. Inteiro de proposito: dinheiro nao e float.
fn total(itens: &[(String, u32)]) -> u32 {
    let cat = catalogo();
    itens
        .iter()
        .map(|(sku, qtd)| cat.iter().find(|p| &p.sku == sku).map(|p| p.preco_centavos * qtd).unwrap_or(0))
        .sum()
}

async fn health() -> &'static str {
    "ok"
}

async fn produtos() -> Json<Vec<Produto>> {
    Json(catalogo())
}

fn auto_teste() -> i32 {
    let mut falhas = 0;
    let casos: Vec<(&str, Vec<(String, u32)>, u32)> = vec![
        ("carrinho vazio", vec![], 0),
        ("uma camiseta", vec![("CAM-01".into(), 1)], 5990),
        ("duas canecas e um bone", vec![("CAN-02".into(), 2), ("BON-03".into(), 1)], 14800),
        ("sku inexistente vale zero", vec![("XXX-99".into(), 3)], 0),
        ("uma mochila", vec![("MOC-04".into(), 1)], 12900),
    ];
    for (nome, itens, esperado) in casos {
        let obtido = total(&itens);
        if obtido == esperado {
            println!("ok   {nome}: {obtido}");
        } else {
            println!("FALHA {nome}: esperado {esperado}, obtido {obtido}");
            falhas += 1;
        }
    }
    if catalogo().len() != 4 {
        println!("FALHA catalogo: esperava 4 produtos");
        falhas += 1;
    }
    falhas
}

#[tokio::main]
async fn main() {
    if std::env::args().any(|a| a == "--auto-teste") {
        let falhas = auto_teste();
        println!("{} falha(s)", falhas);
        std::process::exit(if falhas == 0 { 0 } else { 1 });
    }
    let app = Router::new().route("/health", get(health)).route("/produtos", get(produtos));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("porta 8080");
    println!("loja-api em :8080");
    axum::serve(listener, app).await.expect("servidor");
}
