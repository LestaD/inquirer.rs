pub mod question;
pub mod answer;

pub use question::Question;
pub use answer::Answer;

pub enum Error {
    EmptyQuestions,
    Canceled,
}

pub fn run(list: &Vec<Question>) -> Result<Vec<Answer>, Error> {
    if list.len() == 0 {
        Err(Error::EmptyQuestions)
    }
    else {
        let mut answers: Vec<Answer> = Vec::new();
        let mut id: usize = 1;

        for ques in list {
            println!("{}. {}: ", id, ques.read());
            answers.push(Answer::new(id, ques));
            id += 1;
        }

        Ok(answers)
    }
}
