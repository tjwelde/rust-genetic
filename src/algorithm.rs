use rand;

use self::rand::Rng;
use crate::individual::Individual;
use crate::population::Population;

const TOURNAMENT_SIZE: usize = 5;

pub fn evolve_population(pop: &Population, elitism: bool, solution: &Vec<i8>) -> Population {
  let mut new_pop = Population::new(pop.size(), false, solution);

  // Loop over population size and create new individuals with crossover
  let mut index = 0;
  while index < pop.size() {
    let indiv1 = tournament_selection(pop, solution);
    let indiv2 = tournament_selection(pop, solution);
    let new_indiv = crossover(indiv1, indiv2, solution);
    new_pop.save_individual(index, new_indiv);
    index += 1;
  }

  // TODO: Mutate evolution

  new_pop
}

fn crossover(indiv1: Individual, indiv2: Individual, solution: &Vec<i8>) -> Individual {
  let mut new_sol = Individual::new(solution);

  let mut index = 0;
  while index < indiv1.size() {
    if rand::random::<f32>() <= 0.5 {
      new_sol.set_gene(index, indiv1.genes[index])
    } else {
      new_sol.set_gene(index, indiv2.genes[index])
    }
    index += 1;
  }

  new_sol
}

pub fn tournament_selection(pop: &Population, solution: &Vec<i8>) -> Individual {
  let mut tournament = Population::new(TOURNAMENT_SIZE, false, solution);

  // For each place in tournament get a random individual
  let mut i = 0;
  let result = loop {
    let random_id: usize = rand::thread_rng().gen_range(0, pop.size());
    tournament.save_individual(i, pop.clone_individual(random_id));
    i += 1;
    if i > TOURNAMENT_SIZE {
      break tournament;
    }
  };
  result.get_fittest()
}
