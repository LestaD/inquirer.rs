use std::fmt;

#[derive(Debug)]
pub struct Answer {
  id: usize,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(answer)")
    }
}

impl Answer {
  pub fn new(id: usize) -> Answer {
    Answer {
      id: id,
    }
  }
}
