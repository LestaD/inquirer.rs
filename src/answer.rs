use std::fmt;

use ::question::Question;

#[derive(Debug)]
pub struct Answer<'a> {
  id: usize,
  question: &'a Question,
}

impl<'a> fmt::Display for Answer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(answer)")
    }
}

impl<'a> Answer<'a> {
  pub fn new(id: usize, question: &'a Question) -> Answer {
    Answer { id, question }
  }
}
