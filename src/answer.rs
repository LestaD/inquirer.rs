use std::fmt;

use ::question::Question;

#[derive(Debug)]
pub struct Answer<'a> {
  id: u32,
  question: &'a Question,
  string: String,
}

impl<'a> fmt::Display for Answer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(answer)")
    }
}

impl<'a> Answer<'a> {
  pub fn new(id: u32, question: &'a Question, string: String) -> Answer {
    Answer { id, question, string }
  }

  pub fn get_answer(&self) -> String {
    self.string.clone()
  }
}
