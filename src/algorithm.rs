extern crate rand;

use self::rand::Rng;
use individual::Individual;
use population::Population;

const TOURNAMENT_SIZE: usize = 5;

pub fn evolve_population(pop: &Population, elitism: bool) -> Population {
  let mut new_pop = Population::new(pop.size(), false);

  let mut index = 0;
  while index < pop.size() {
    let indiv1 = tournament_Selection(pop);
    let indiv2 = tournament_Selection(pop);
    let new_indiv = crossover(indiv1, indiv2);
    new_pop.save_individual(index, new_indiv);
    index += 1;
  }

  new_pop
}

fn crossover(indiv1: &Individual, indiv2: &Individual) -> Individual {
  let new_sol = Individual::new();
  new_sol
}

pub fn tournament_Selection(pop: &Population) -> &Individual {
  let mut tournament = Population::new(TOURNAMENT_SIZE, false);

  let mut i = 0;
  let result = loop {
    let random_id: usize = rand::thread_rng().gen_range(0, 2);
    tournament.save_individual(i, pop.clone_individual(random_id));
    i += 1;
    if i < TOURNAMENT_SIZE {
      break tournament;
    }
  };
  tournament.get_fittest()
}
