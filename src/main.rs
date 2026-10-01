mod cliente;
mod produto;
mod categoria;
mod grafo;
mod recomendacao;

use cliente::Cliente;
use produto::Produto;
use categoria::Categoria;
use grafo::Grafo;
use recomendacao::recomendar_produtos;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;

fn ler_texto(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn ler_u32(mensagem: &str) -> u32 {
    loop {
        let entrada = ler_texto(mensagem);
        match entrada.parse::<u32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Digite um número válido."),
        }
    }
}

fn ler_f64(mensagem: &str) -> f64 {
    loop {
        let entrada = ler_texto(mensagem);
        match entrada.replace(',', ".").parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("Digite um preço válido."),
        }
    }
}

fn mostrar_produtos(produtos: &HashMap<u32, Produto>) {
    println!("\n=== PRODUTOS CADASTRADOS ===");
    if produtos.is_empty() {
        println!("Nenhum produto cadastrado.");
        return;
    }

    let mut lista: Vec<&Produto> = produtos.values().collect();
    lista.sort_by_key(|p| p.id);

    for produto in lista {
        println!(
            "ID: {} | {} | Categoria: {} | R$ {:.2}",
            produto.id, produto.nome, produto.categoria, produto.preco
        );
    }
}

fn mostrar_clientes(clientes: &HashMap<u32, Cliente>) {
    println!("\n=== CLIENTES ===");
    for cliente in clientes.values() {
        println!("ID: {} | {}", cliente.id, cliente.nome);
    }
}

fn cadastrar_produto(produtos: &mut HashMap<u32, Produto>) {
    println!("\n=== CADASTRO DE PRODUTO ===");
    let id = ler_u32("ID: ");

    if produtos.contains_key(&id) {
        println!("Já existe um produto com esse ID.");
        return;
    }

    let nome = ler_texto("Nome: ");
    let categoria = ler_texto("Categoria: ");
    let preco = ler_f64("Preço: ");

    let produto = Produto::novo(id, &nome, &categoria, preco);
    produtos.insert(id, produto);

    println!("Produto cadastrado com sucesso.");
}

fn consultar_produto(produtos: &HashMap<u32, Produto>) {
    println!("\n=== CONSULTA DE PRODUTO ===");
    let id = ler_u32("Informe o ID: ");

    match produtos.get(&id) {
        Some(produto) => produto.mostrar(),
        None => println!("Produto não encontrado."),
    }
}

fn cadastrar_cliente(clientes: &mut HashMap<u32, Cliente>) {
    println!("\n=== CADASTRO DE CLIENTE ===");
    let id = ler_u32("ID: ");

    if clientes.contains_key(&id) {
        println!("Já existe um cliente com esse ID.");
        return;
    }

    let nome = ler_texto("Nome: ");
    clientes.insert(id, Cliente::novo(id, &nome));
    println!("Cliente cadastrado com sucesso.");
}

fn registrar_compra(
    clientes: &mut HashMap<u32, Cliente>,
    produtos: &HashMap<u32, Produto>,
    grafo: &mut Grafo,
) {
    println!("\n=== REGISTRAR COMPRA ===");
    let cliente_id = ler_u32("ID do cliente: ");
    let produto_id = ler_u32("ID do produto: ");

    if !clientes.contains_key(&cliente_id) {
        println!("Cliente não encontrado.");
        return;
    }

    if !produtos.contains_key(&produto_id) {
        println!("Produto não encontrado.");
        return;
    }

    let cliente = clientes.get_mut(&cliente_id).unwrap();

    if cliente.comprou(produto_id) {
        println!("Esse cliente já comprou esse produto.");
        return;
    }

    cliente.adicionar_compra(produto_id);
    grafo.adicionar_aresta(cliente_id, produto_id, 1.0);

    println!("Compra registrada e conexão adicionada ao grafo.");
}

fn adicionar_similaridade(grafo: &mut Grafo, produtos: &HashMap<u32, Produto>) {
    println!("\n=== CONECTAR PRODUTOS ===");
    let produto_a = ler_u32("ID do primeiro produto: ");
    let produto_b = ler_u32("ID do segundo produto: ");

    if !produtos.contains_key(&produto_a) || !produtos.contains_key(&produto_b) {
        println!("Um ou ambos os produtos não existem.");
        return;
    }

    if produto_a == produto_b {
        println!("Não é possível conectar um produto a ele mesmo.");
        return;
    }

    let peso = ler_f64("Peso da similaridade (0 a 1): ");
    if !(0.0..=1.0).contains(&peso) {
        println!("O peso deve estar entre 0 e 1.");
        return;
    }

    grafo.adicionar_aresta(produto_a, produto_b, peso);
    grafo.adicionar_aresta(produto_b, produto_a, peso);

    println!("Similaridade adicionada.");
}

fn recomendar(clientes: &HashMap<u32, Cliente>, produtos: &HashMap<u32, Produto>, grafo: &Grafo) {
    println!("\n=== RECOMENDAÇÃO ===");
    let cliente_id = ler_u32("ID do cliente: ");

    if !clientes.contains_key(&cliente_id) {
        println!("Cliente não encontrado.");
        return;
    }

    let recomendacoes = recomendar_produtos(cliente_id, clientes, produtos, grafo, 5);

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
        return;
    }

    println!("Produtos recomendados:");
    for (posicao, produto) in recomendacoes.iter().enumerate() {
        println!(
            "{}. {} - {} - R$ {:.2}",
            posicao + 1,
            produto.id,
            produto.nome,
            produto.preco
        );
    }
}

fn mostrar_grafo(grafo: &Grafo) {
    println!("\n=== LISTA DE ADJACÊNCIA ===");
    grafo.mostrar();
}

fn executar_desempenho() {
    println!("\n=== TESTE DE DESEMPENHO ===");
    println!("Os testes usam grafos artificiais para comparar diferentes volumes.");

    let volumes = [100usize, 1_000, 10_000, 50_000];

    println!("{:<15} {:<20}", "Vértices", "Tempo de construção");
    println!("------------------------------------");

    for quantidade in volumes {
        let inicio = Instant::now();
        let mut grafo = Grafo::novo();

        for i in 1..=quantidade {
            grafo.adicionar_vertice(i as u32);
            if i > 1 {
                grafo.adicionar_aresta((i - 1) as u32, i as u32, 1.0);
            }
        }

        let tempo = inicio.elapsed();
        println!("{:<15} {:?}", quantidade, tempo);
    }
}

fn carregar_dados_exemplo(
    clientes: &mut HashMap<u32, Cliente>,
    produtos: &mut HashMap<u32, Produto>,
    categorias: &mut HashMap<u32, Categoria>,
    grafo: &mut Grafo,
) {
    let categorias_exemplo = vec![
        Categoria::novo(1, "Informatica"),
        Categoria::novo(2, "Celulares"),
        Categoria::novo(3, "Audio"),
        Categoria::novo(4, "Perifericos"),
    ];

    for categoria in categorias_exemplo {
        categorias.insert(categoria.id, categoria);
    }

    let produtos_exemplo = vec![
        Produto::novo(1, "Notebook Gamer", "Informatica", 4500.00),
        Produto::novo(2, "Mouse Gamer", "Perifericos", 150.00),
        Produto::novo(3, "Teclado Mecanico", "Perifericos", 280.00),
        Produto::novo(4, "Monitor 24", "Informatica", 900.00),
        Produto::novo(5, "Headset Gamer", "Audio", 220.00),
        Produto::novo(6, "Celular", "Celulares", 1800.00),
        Produto::novo(7, "Tablet", "Celulares", 1200.00),
        Produto::novo(8, "Webcam", "Perifericos", 190.00),
        Produto::novo(9, "Caixa de Som", "Audio", 250.00),
        Produto::novo(10, "Microfone", "Audio", 320.00),
    ];

    for produto in produtos_exemplo {
        produtos.insert(produto.id, produto);
    }

    let clientes_exemplo = vec![
        Cliente::novo(101, "Joao"),
        Cliente::novo(102, "Maria"),
        Cliente::novo(103, "Pedro"),
        Cliente::novo(104, "Lucas"),
        Cliente::novo(105, "Ana"),
    ];

    for cliente in clientes_exemplo {
        clientes.insert(cliente.id, cliente);
    }

    // Compras dos clientes.
    let compras = vec![
        (101, vec![1, 2]),
        (102, vec![1, 3]),
        (103, vec![6, 5]),
        (104, vec![1, 4]),
        (105, vec![7, 8]),
    ];

    for (cliente_id, produtos_comprados) in compras {
        for produto_id in produtos_comprados {
            clientes.get_mut(&cliente_id).unwrap().adicionar_compra(produto_id);
            grafo.adicionar_aresta(cliente_id, produto_id, 1.0);
        }
    }

    // Relações de similaridade entre produtos.
    let similares = vec![
        (1, 2, 0.90),
        (1, 3, 0.85),
        (1, 4, 0.80),
        (1, 5, 0.75),
        (2, 3, 0.90),
        (2, 5, 0.70),
        (2, 8, 0.75),
        (3, 5, 0.80),
        (4, 8, 0.85),
        (5, 10, 0.80),
        (5, 9, 0.65),
        (6, 5, 0.60),
        (6, 7, 0.80),
        (7, 8, 0.65),
        (9, 10, 0.85),
    ];

    for (a, b, peso) in similares {
        grafo.adicionar_aresta(a, b, peso);
        grafo.adicionar_aresta(b, a, peso);
    }
}

fn main() {
    let mut produtos: HashMap<u32, Produto> = HashMap::new();
    let mut clientes: HashMap<u32, Cliente> = HashMap::new();
    let mut categorias: HashMap<u32, Categoria> = HashMap::new();
    let mut grafo = Grafo::novo();

    carregar_dados_exemplo(
        &mut clientes,
        &mut produtos,
        &mut categorias,
        &mut grafo,
    );

    println!("==============================================");
    println!("          CONECTASTORE - MEGASTORE");
    println!(" Sistema de Recomendacao Baseado em Grafos");
    println!("==============================================");
    println!("Dados de exemplo carregados.");

    loop {
        println!("\n================ MENU ================");
        println!("1 - Cadastrar produto");
        println!("2 - Consultar produto");
        println!("3 - Listar produtos");
        println!("4 - Cadastrar cliente");
        println!("5 - Listar clientes");
        println!("6 - Registrar compra");
        println!("7 - Conectar produtos por similaridade");
        println!("8 - Recomendar produtos");
        println!("9 - Mostrar grafo");
        println!("10 - Teste de desempenho");
        println!("0 - Sair");
        println!("======================================");

        let opcao = ler_texto("Escolha uma opcao: ");

        match opcao.as_str() {
            "1" => cadastrar_produto(&mut produtos),
            "2" => consultar_produto(&produtos),
            "3" => mostrar_produtos(&produtos),
            "4" => cadastrar_cliente(&mut clientes),
            "5" => mostrar_clientes(&clientes),
            "6" => registrar_compra(&mut clientes, &produtos, &mut grafo),
            "7" => adicionar_similaridade(&mut grafo, &produtos),
            "8" => recomendar(&clientes, &produtos, &grafo),
            "9" => mostrar_grafo(&grafo),
            "10" => executar_desempenho(),
            "0" => {
                println!("Programa encerrado.");
                break;
            }
            _ => println!("Opcao invalida."),
        }
    }
}
