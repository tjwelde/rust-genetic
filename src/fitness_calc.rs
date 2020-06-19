use individual::Individual;

use SOLUTION;

pub fn get_fitness(indiv: &Individual) -> usize {
  let mut fitness = 0;

  let individual_size = indiv.size();
  let mut i: usize = 0;

  while i < individual_size && (i as usize) < SOLUTION.len() {
    if indiv.genes[i] == SOLUTION[i] {
      fitness += 1;
    }
    i += 1;
  }
  fitness
}
pub fn get_max_fitness() -> usize {
  SOLUTION.len()
}
