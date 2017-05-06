
extern crate inquirer;

use inquirer::{Question};

fn main() {
  let list = vec![
    Question::new(),
    Question::new(),
    Question::new(),
  ];

  match inquirer::run(&list) {
    Ok(list) => println!("Answers {:?}", list),
    Err(err) => match err {
      inquirer::Error::EmptyQuestions => println!("No questions!"),
      inquirer::Error::Canceled => println!("Canceled."),
    }
  }
}
