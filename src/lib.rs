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

        for i in 0..list.len() as usize {
            answers.push(Answer::new(i));
        }

        Ok(answers)
    }
}
