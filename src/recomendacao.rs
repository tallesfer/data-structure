use crate::cliente::Cliente;
use crate::grafo::Grafo;
use crate::produto::Produto;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn recomendar_produtos(
    cliente_id: u32,
    clientes: &HashMap<u32, Cliente>,
    produtos: &HashMap<u32, Produto>,
    grafo: &Grafo,
    limite: usize,
) -> Vec<Produto> {
    let cliente = match clientes.get(&cliente_id) {
        Some(cliente) => cliente,
        None => return Vec::new(),
    };

    let mut fila = VecDeque::new();
    let mut visitados = HashSet::new();
    let mut recomendados = HashSet::new();

    // Começamos pelos produtos que o cliente já comprou.
    for produto_id in &cliente.compras {
        fila.push_back((*produto_id, 0));
        visitados.insert(*produto_id);
    }

    while let Some((atual, distancia)) = fila.pop_front() {
        // Limita a busca a três níveis para evitar percursos desnecessários.
        if distancia >= 3 {
            continue;
        }

        for (vizinho, peso) in grafo.vizinhos(atual) {
            if !visitados.contains(&vizinho) {
                visitados.insert(vizinho);
                fila.push_back((vizinho, distancia + 1));

                // Só produtos cadastrados podem virar recomendações.
                // O peso precisa representar uma relação positiva.
                if produtos.contains_key(&vizinho) && peso > 0.0 {
                    // Produtos já comprados pelo cliente não são recomendados.
                    if !cliente.compras.contains(&vizinho) {
                        recomendados.insert(vizinho);
                    }
                }
            }
        }
    }

    // Ordena primeiro pelos maiores pesos encontrados diretamente.
    // Como a recomendação é baseada no percurso, aqui mantemos uma
    // ordenação simples pelo ID para produzir uma saída estável.
    let mut ids: Vec<u32> = recomendados.into_iter().collect();
    ids.sort_unstable();

    ids.into_iter()
        .take(limite)
        .filter_map(|id| produtos.get(&id).cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cliente::Cliente;
    use crate::grafo::Grafo;
    use crate::produto::Produto;

    #[test]
    fn nao_deve_recomendar_produto_ja_comprado() {
        let mut clientes = HashMap::new();
        let mut produtos = HashMap::new();
        let mut grafo = Grafo::novo();

        let mut cliente = Cliente::novo(1, "Joao");
        cliente.adicionar_compra(10);

        clientes.insert(1, cliente);
        produtos.insert(10, Produto::novo(10, "Notebook", "Info", 1000.0));
        produtos.insert(11, Produto::novo(11, "Mouse", "Info", 100.0));

        grafo.adicionar_aresta(10, 11, 0.9);

        let resultado = recomendar_produtos(1, &clientes, &produtos, &grafo, 5);

        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].id, 11);
    }

    #[test]
    fn nao_deve_duplicar_recomendacoes() {
        let mut clientes = HashMap::new();
        let mut produtos = HashMap::new();
        let mut grafo = Grafo::novo();

        let mut cliente = Cliente::novo(1, "Joao");
        cliente.adicionar_compra(10);
        cliente.adicionar_compra(12);

        clientes.insert(1, cliente);

        produtos.insert(10, Produto::novo(10, "Notebook", "Info", 1000.0));
        produtos.insert(12, Produto::novo(12, "Teclado", "Info", 200.0));
        produtos.insert(11, Produto::novo(11, "Mouse", "Info", 100.0));

        grafo.adicionar_aresta(10, 11, 0.9);
        grafo.adicionar_aresta(12, 11, 0.9);

        let resultado = recomendar_produtos(1, &clientes, &produtos, &grafo, 5);

        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].id, 11);
    }
}
