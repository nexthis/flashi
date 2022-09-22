#[derive(Debug)]
pub struct Press {
    key: char,
}

impl Press {
    pub fn new() -> Press {
        Press { key: 't' }
    }
}
