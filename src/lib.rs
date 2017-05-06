extern crate term;

pub mod question;
pub mod answer;

use std::io::{stdin, Write};
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
        let mut id: u32 = 1;
        let mut t = term::stdout().unwrap();

        for ques in list {
            write!(t, "{}. ", id.to_string()).unwrap();
            t.fg(term::color::GREEN).unwrap();
            write!(t, "{}: ", ques.read()).unwrap();
            t.reset().unwrap();
            t.flush().unwrap();

            let mut answer_string = "".to_string();
            stdin().read_line(&mut answer_string).unwrap();

            answers.push(Answer::new(id, ques, answer_string.trim().to_string()));
            id += 1;
        }

        Ok(answers)
    }
}
