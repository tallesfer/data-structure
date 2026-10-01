use megastore::cliente::Cliente;
use megastore::grafo::Grafo;
use megastore::produto::Produto;
use megastore::recomendacao::recomendar_produtos;
use std::collections::HashMap;

#[test]
fn fluxo_completo_de_recomendacao() {
    let mut clientes = HashMap::new();
    let mut produtos = HashMap::new();
    let mut grafo = Grafo::novo();

    let mut cliente = Cliente::novo(1, "Maria");
    cliente.adicionar_compra(100);

    clientes.insert(1, cliente);

    produtos.insert(100, Produto::novo(100, "Notebook", "Informatica", 4000.0));
    produtos.insert(101, Produto::novo(101, "Mouse", "Perifericos", 150.0));
    produtos.insert(102, Produto::novo(102, "Teclado", "Perifericos", 250.0));

    grafo.adicionar_aresta(100, 101, 0.9);
    grafo.adicionar_aresta(100, 102, 0.8);

    let recomendacoes = recomendar_produtos(1, &clientes, &produtos, &grafo, 5);

    assert_eq!(recomendacoes.len(), 2);
    assert!(recomendacoes.iter().any(|p| p.id == 101));
    assert!(recomendacoes.iter().any(|p| p.id == 102));
}
