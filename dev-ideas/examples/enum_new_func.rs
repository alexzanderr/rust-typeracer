pub enum Errors {
    One { index: usize, text: String },
    Two
}

impl Errors {
    pub fn new(
        index: usize,
        text: &str
    ) -> Self {
        let text = text.to_string();
        Self {
            index,
            text
        }
    }
}

fn main() {
    let index = 123;
    let text = "asd";
    let error = Errors::One.new(index, text);
}
