#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    // Cria um novo produto.
    pub fn novo(id: u32, nome: &str, categoria: &str, preco: f64) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }

    // Mostra os dados do produto no terminal.
    pub fn mostrar(&self) {
        println!("ID: {}", self.id);
        println!("Nome: {}", self.nome);
        println!("Categoria: {}", self.categoria);
        println!("Preço: R$ {:.2}", self.preco);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_criar_produto() {
        let produto = Produto::novo(1, "Mouse", "Perifericos", 100.0);

        assert_eq!(produto.id, 1);
        assert_eq!(produto.nome, "Mouse");
        assert_eq!(produto.categoria, "Perifericos");
        assert_eq!(produto.preco, 100.0);
    }
}
