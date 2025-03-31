use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post2 {
    pub prompt: String,
    pub generated: Option<String>,
}

impl Post2 {
    pub fn new(prompt: String) -> Self {
        Post2 { prompt, generated: None }
    }

    pub fn generate(&self) -> Self {
        // put generate function here 
        let generated= String::from("This is a generated post from prompt = {&self.prompt}.");
        Post2 { 
            prompt: String::from(&self.prompt),
            generated: Some(generated)
        }
}
}


