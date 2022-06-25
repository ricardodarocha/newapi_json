use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct Todo {
    id: i32,
    qtd: Optional<i32>,
    title: String,


}
