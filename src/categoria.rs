#[derive(Debug, Clone)]
pub struct Categoria {
    pub id: u32,
    pub nome: String,
}

impl Categoria {
    // Cria uma nova categoria.
    pub fn novo(id: u32, nome: &str) -> Self {
        Self {
            id,
            nome: nome.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deve_criar_categoria() {
        let categoria = Categoria::novo(1, "Informatica");

        assert_eq!(categoria.id, 1);
        assert_eq!(categoria.nome, "Informatica");
    }
}
