use rand::Rng;

#[derive(Clone)]
pub struct Individual {
  pub genes: Vec<i8>,
  pub size: usize,
  pub solution: &'static [i8],
}

impl Individual {
  pub fn new(solution: &'static [i8]) -> Individual {
    let mut genes = Vec::new();
    let size = solution.len();

    let mut rng = rand::thread_rng();

    for _i in 0..size {
      genes.push(rng.gen::<bool>() as i8);
    }

    Individual {
      genes,
      size,
      solution,
    }
  }

  pub fn set_gene(&mut self, index: usize, value: i8) {
    self.genes[index] = value;
  }

  pub fn get_fitness(&self) -> usize {
    self.genes.iter().zip(self.solution).filter(|(a, b)| a == b).count()
  }
}
