use crate::individual::Individual;

pub struct Population {
  individuals: Vec<Individual>,
  population_size: usize,
}

impl Population {
  pub fn new(population_size: usize, initialize: bool, solution: &'static [i8]) -> Population {
    let mut pop = Population {
      population_size,
      individuals: Vec::with_capacity(population_size),
    };

    if initialize {
      for i in 0..population_size {
        let new_indiv = Individual::new(solution);
        pop.save_individual(i, new_indiv);
      }
    }
    pop
  }

  pub fn save_individual(&mut self, i: usize, indiv: Individual) {
    self.individuals.insert(i, indiv);
  }

  pub fn clone_individual(&self, index: usize) -> Individual {
    self.individuals[index].clone()
  }

  pub fn get_fittest(&self) -> &Individual {
    // It's assumed that individuals is not empty. therefore unwrap
    self.individuals.iter().max_by(|a, b| {
      a.get_fitness().cmp(&b.get_fitness())
    }).unwrap()
  }

  pub fn size(&self) -> usize {
    self.population_size
  }
}
