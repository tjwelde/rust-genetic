use rand::Rng;
use crate::fitness_calc;

#[derive(Clone)]
pub struct Individual {
  pub genes: Vec<i8>,
  pub size: usize,
  pub solution: Vec<i8>,
}

impl Individual {
  pub fn new(solution: &Vec<i8>) -> Individual {
    let mut genes = Vec::new();
    let size = solution.len();

    let mut rng = rand::thread_rng(); 

    for _i in 0..size {
      genes.push((rng.gen::<u16>() % 2) as i8);
    }

    Individual {
      genes,
      size,
      solution: solution.clone(),
    }
  }

  pub fn set_gene(&mut self, index: usize, value: i8) {
    self.genes[index] = value;
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn get_fitness(&self) -> usize {
    fitness_calc::get_fitness(self, &self.solution)
  }
}
