use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    id: i32,
    qtd: i32,
    title: String,
    completed: bool    

}

impl Hello {
    pub fn new(title: String) -> Hello {   
        Hello { id: 7, qtd: 6520, title: title, completed: true}
    }
}

