pub struct GlobalState {
    pub test: String,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            test: "".to_owned(),
        }
    }
}
