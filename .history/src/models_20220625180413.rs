use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    id: i32,
    qtd: Optional<i32>,
    title: String,
    completed: bool    

}

impl Hello {
    pub fn new(title: String) -> Hello {   
        Hello { 7, 6520, title, true}
    }
}

