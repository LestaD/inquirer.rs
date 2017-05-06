pub mod question;
pub mod answer;

use std::io::{stdin, stdout, Write};
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
            // let s: String = String::from("asd");

            // let sout = stdout();
            // let mut handle = sout.lock();
            // handle.write(format!("{}. {}: ", id, ques.read()).into_bytes().as_slice());

            println!("{}. {}: ", id, ques.read());

            let mut answer_string = "".to_string();
            stdin().read_line(&mut answer_string).unwrap();

            answers.push(Answer::new(id, ques, answer_string.trim().to_string()));
            id += 1;
        }

        Ok(answers)
    }
}
