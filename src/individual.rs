extern crate rand;

use self::rand::Rng;

pub struct Individual {
  pub genes: Vec<i8>,
  pub size: usize,
}

impl Individual {
  pub fn new() -> Individual {
    let mut genes = Vec::new();
    let size = 5;

    for _i in 0..size {
      genes.push(rand::thread_rng().gen_range(0, 2));
    }

    Individual { genes, size }
  }

  pub fn set_gene(&mut self, index: usize, value: i8) {
    self.genes[index] = value;
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn to_string(&self) -> String {
    let mut gene_string = String::from("");
    for _i in &self.genes {
      gene_string.push_str(&_i.to_string())
    }
    gene_string
  }
}
