
extern crate inquirer;

use inquirer::{Question};

///
/// # Foo bar
///
/// ```
/// let foo = 5;
/// assert_eq!(foo, 5);
/// ```

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
