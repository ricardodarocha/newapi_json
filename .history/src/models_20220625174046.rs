use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
struct Hello {
    id: i32,
    qtd: Optional<i32>,
    title: String,
    completed: bool    

}
