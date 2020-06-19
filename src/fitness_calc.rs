use crate::individual::Individual;

pub fn get_fitness(indiv: &Individual, solution: &Vec<i8>) -> usize {
  let mut fitness = 0;

  let individual_size = indiv.size();
  let mut i: usize = 0;

  while i < individual_size && (i as usize) < solution.len() {
    if indiv.genes[i] == solution[i] {
      fitness += 1;
    }
    i += 1;
  }
  fitness
}
pub fn get_max_fitness(solution: &Vec<i8>) -> usize {
  solution.len()
}
