use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub compras: HashSet<u32>,
}

impl Cliente {
    // Cria um novo cliente sem compras.
    pub fn novo(id: u32, nome: &str) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            compras: HashSet::new(),
        }
    }

    // Adiciona um produto ao conjunto de compras do cliente.
    // O HashSet evita que a mesma compra seja adicionada duas vezes.
    pub fn adicionar_compra(&mut self, produto_id: u32) {
        self.compras.insert(produto_id);
    }

    // Verifica se o cliente já comprou determinado produto.
    pub fn comprou(&self, produto_id: u32) -> bool {
        self.compras.contains(&produto_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_registrar_compra_sem_duplicar() {
        let mut cliente = Cliente::novo(1, "Joao");

        cliente.adicionar_compra(10);
        cliente.adicionar_compra(10);

        assert_eq!(cliente.compras.len(), 1);
        assert!(cliente.comprou(10));
    }
}
