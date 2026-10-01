# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

## 1. Objetivo

O ConectaStore é um sistema desenvolvido em Rust para demonstrar como grafos e estruturas de dados complementares podem ser utilizados para gerar recomendações de produtos.

O cenário considera uma empresa de comércio eletrônico com grande quantidade de produtos, clientes, categorias e histórico de compras.

A proposta é representar as relações entre os elementos por meio de um grafo e utilizar busca em largura (BFS) para encontrar produtos relacionados aos itens já comprados por um cliente.

## 2. Problema

Sistemas que recomendam apenas produtos mais vendidos ou produtos da mesma categoria podem produzir recomendações genéricas.

O ConectaStore utiliza as conexões do grafo para explorar relações de compra e similaridade entre produtos.

## 3. Modelagem

### Vértices

O sistema trabalha com:

- Clientes;
- Produtos;
- Categorias.

### Arestas

As arestas representam relações:

- Cliente -> Produto: compra;
- Produto -> Produto: similaridade.

As relações de produtos possuem peso entre 0 e 1.

### Tipo de grafo

A implementação utiliza lista de adjacência e permite representar relações direcionadas. As similaridades entre produtos são cadastradas nos dois sentidos, formando uma relação bidirecional.

## 4. Estruturas de dados

### HashMap

Utilizado para armazenar produtos, clientes e categorias pelo identificador. A busca média por chave possui complexidade O(1).

### HashSet

Utilizado para controlar produtos comprados e impedir recomendações duplicadas.

### Vec

Utilizado para armazenar listas de conexões e resultados.

### VecDeque

Utilizado como fila durante a execução do BFS.

### Lista de adjacência

Utilizada para representar o grafo. Essa representação evita reservar uma posição para todas as possíveis combinações de vértices, sendo adequada para grafos esparsos.

## 5. Algoritmo de recomendação

O algoritmo inicia pelos produtos já comprados pelo cliente.

Depois utiliza BFS para percorrer produtos relacionados em até três níveis.

Durante o percurso:

1. Os vértices já visitados são controlados por HashSet.
2. Produtos já comprados pelo cliente são ignorados.
3. Apenas produtos existentes no catálogo podem ser recomendados.
4. O HashSet impede duplicações.
5. O resultado é limitado à quantidade solicitada.

## 6. Complexidade

### BFS

Com lista de adjacência:

O(V + E)

Onde:

- V = número de vértices;
- E = número de arestas.

### Lista de adjacência

Espaço:

O(V + E)

### HashMap

A busca média por chave é O(1).

### HashSet

Inserção e consulta possuem custo médio O(1).

## 7. Desempenho

O programa possui uma opção de teste de desempenho no menu.

Ela cria grafos artificiais com:

- 100 vértices;
- 1.000 vértices;
- 10.000 vértices;
- 50.000 vértices.

Os tempos devem ser registrados durante a execução no computador utilizado na apresentação.

## 8. Como executar

É necessário ter Rust e Cargo instalados.

Executar:

```bash
cargo run
```

## 9. Como executar os testes

```bash
cargo test
```

## 10. Funcionalidades

- Cadastro de produtos;
- Consulta de produtos;
- Listagem de produtos;
- Cadastro de clientes;
- Registro de compras;
- Construção do grafo;
- Lista de adjacência;
- Conexão de produtos por similaridade;
- BFS;
- Recomendação de produtos;
- Prevenção de recomendações duplicadas;
- Testes unitários;
- Teste de integração;
- Medição básica de desempenho.

## 11. Arquitetura

```text
                    +----------------+
                    |    Cliente     |
                    +-------+--------+
                            |
                          compra
                            |
                            v
                    +----------------+
                    |    Produto     |
                    +-------+--------+
                            |
                       similaridade
                            |
                            v
                    +----------------+
                    |    Produto     |
                    +----------------+

                            |
                            v
                         BFS
                            |
                            v
                    +----------------+
                    | Recomendações  |
                    +----------------+
```

## 12. Estrutura do projeto

```text
megastore/
├── src/
│   ├── main.rs
│   ├── produto.rs
│   ├── cliente.rs
│   ├── categoria.rs
│   ├── grafo.rs
│   └── recomendacao.rs
├── tests/
│   └── integracao.rs
├── Cargo.toml
└── README.md
```

## 13. Escalabilidade

Para catálogos muito grandes, a lista de adjacência evita o custo de uma matriz de tamanho V x V quando o grafo possui poucas conexões.

O uso de HashMap também permite acesso rápido aos registros pelo identificador.

Em uma aplicação real, o projeto poderia evoluir para armazenamento persistente em banco de dados, cache, processamento paralelo e distribuição dos dados. Essas extensões não fazem parte da implementação mínima deste trabalho.

## 14. Vídeo Pitch

https://youtu.be/N2qsVRLxTbo




