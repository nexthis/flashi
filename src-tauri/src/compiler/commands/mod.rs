trait Command<T, E> {
    fn run() -> Result<T, E>;
}

#[derive(Debug)]
pub struct Press {
    key: String,
}

impl Press {
    pub fn new() -> Press {
        Press {
            key: "String".to_string(),
        }
    }
}

impl Command<String, String> for Press {
    fn run() -> Result<String, String> {
        todo!()
    }
}
