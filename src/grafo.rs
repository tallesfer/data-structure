use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Aresta {
    pub destino: u32,
    pub peso: f64,
}

#[derive(Debug, Clone)]
pub struct Grafo {
    // Lista de adjacência:
    // cada vértice possui uma lista com seus vizinhos.
    pub adjacencia: HashMap<u32, Vec<Aresta>>,
}

impl Grafo {
    // Cria um grafo vazio.
    pub fn novo() -> Self {
        Self {
            adjacencia: HashMap::new(),
        }
    }

    // Adiciona um vértice ao grafo.
    pub fn adicionar_vertice(&mut self, id: u32) {
        self.adjacencia.entry(id).or_default();
    }

    // Adiciona uma aresta direcionada entre dois vértices.
    pub fn adicionar_aresta(&mut self, origem: u32, destino: u32, peso: f64) {
        self.adicionar_vertice(origem);
        self.adicionar_vertice(destino);

        self.adjacencia
            .get_mut(&origem)
            .unwrap()
            .push(Aresta { destino, peso });
    }

    // Retorna os vizinhos de um vértice.
    pub fn vizinhos(&self, id: u32) -> Vec<(u32, f64)> {
        match self.adjacencia.get(&id) {
            Some(arestas) => arestas
                .iter()
                .map(|aresta| (aresta.destino, aresta.peso))
                .collect(),
            None => Vec::new(),
        }
    }

    // BFS percorre o grafo por níveis a partir de um vértice.
    pub fn bfs(&self, inicio: u32) -> Vec<u32> {
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();
        let mut ordem = Vec::new();

        visitados.insert(inicio);
        fila.push_back(inicio);

        while let Some(atual) = fila.pop_front() {
            ordem.push(atual);

            if let Some(vizinhos) = self.adjacencia.get(&atual) {
                for aresta in vizinhos {
                    if !visitados.contains(&aresta.destino) {
                        visitados.insert(aresta.destino);
                        fila.push_back(aresta.destino);
                    }
                }
            }
        }

        ordem
    }

    // Mostra a lista de adjacência no terminal.
    pub fn mostrar(&self) {
        let mut vertices: Vec<u32> = self.adjacencia.keys().copied().collect();
        vertices.sort_unstable();

        for vertice in vertices {
            print!("{} -> ", vertice);

            if let Some(arestas) = self.adjacencia.get(&vertice) {
                for aresta in arestas {
                    print!("[{} | peso {:.2}] ", aresta.destino, aresta.peso);
                }
            }

            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_adicionar_aresta() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_aresta(1, 2, 0.8);

        let vizinhos = grafo.vizinhos(1);

        assert_eq!(vizinhos.len(), 1);
        assert_eq!(vizinhos[0].0, 2);
        assert_eq!(vizinhos[0].1, 0.8);
    }

    #[test]
    fn bfs_deve_percorrer_grafo() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_aresta(1, 2, 1.0);
        grafo.adicionar_aresta(1, 3, 1.0);
        grafo.adicionar_aresta(2, 4, 1.0);

        let resultado = grafo.bfs(1);

        assert_eq!(resultado[0], 1);
        assert!(resultado.contains(&2));
        assert!(resultado.contains(&3));
        assert!(resultado.contains(&4));
    }
}
