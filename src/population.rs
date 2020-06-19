use crate::individual::Individual;

pub struct Population {
  individuals: Vec<Individual>,
  population_size: usize,
}

impl Population {
  pub fn new(population_size: usize, initialize: bool, solution: &Vec<i8>) -> Population {
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
    let individual = &self.individuals[index];
    Individual {
      genes: individual.genes.clone(),
      size: individual.size,
      solution: individual.solution.clone(),
    }
  }

  pub fn get_fittest(&self) -> Individual {
    let mut fittest = 0;
    let mut index = 0;

    for indiv in &self.individuals {
      if self.individuals[fittest].get_fitness() <= indiv.get_fitness() {
        fittest = index;
      }
      index += 1;
    }
    self.clone_individual(fittest)
  }

  pub fn size(&self) -> usize {
    self.population_size
  }
}
