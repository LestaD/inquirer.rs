
#[derive(Debug)]
pub struct Question {
    question: String,
}

impl Question {
    pub fn new(question: &str) -> Question {
        Question { question: question.to_string() }
    }

    pub fn read(&self) -> &String {
        &self.question
    }
}
