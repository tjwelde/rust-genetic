use crate::individual::Individual;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const TOURNAMENT_SIZE: usize = 5;
const UNIFORM_RATE: f32 = 0.5;
const MUTATION_RATE: f32 = 0.015;
const ELITISM: bool = true;

pub fn evolve_population(pop: &Population, solution: &'static [i8]) -> Population {
  let mut new_pop = Population::new(pop.size(), false, solution);

  // Keep our best individual
  let elitism_offset = if ELITISM {
    new_pop.save_individual(0, pop.get_fittest().clone());
    1
  } else {
    0
  };

  // Loop over population size and create new individuals with crossover
  for index in elitism_offset..pop.size() {
    let indiv1 = tournament_selection(pop, solution);
    let indiv2 = tournament_selection(pop, solution);
    let new_indiv = crossover(&indiv1, &indiv2, solution);
    new_pop.save_individual(index, new_indiv);
  }

  // Mutate population
  for i in elitism_offset..new_pop.size() {
    let mut indiv = new_pop.clone_individual(i);
    self::mutate(&mut indiv);
    new_pop.save_individual(i, indiv);
  }

  new_pop
}

fn crossover(indiv1: &Individual, indiv2: &Individual, solution: &'static [i8]) -> Individual {
  let mut new_sol = Individual::new(solution);

  for index in 0..indiv1.size {
    if rand::random::<f32>() <= UNIFORM_RATE {
      new_sol.set_gene(index, indiv1.genes[index])
    } else {
      new_sol.set_gene(index, indiv2.genes[index])
    }
  }

  new_sol
}

fn mutate(indiv: &mut Individual) {
  let mut generator = rand::thread_rng();
  for i in 0..indiv.size {
    if rand::random::<f32>() <= MUTATION_RATE {
      let gene = generator.gen::<bool>() as i8;
      indiv.set_gene(i, gene);
    }
  }
}

pub fn tournament_selection(pop: &Population, solution: &'static [i8]) -> Individual {
  let mut tournament = Population::new(TOURNAMENT_SIZE, false, solution);

  let mut rng = rand::thread_rng();
  let between = Uniform::from(0..pop.size());

  // For each place in tournament get a random individual
  for i in 0..TOURNAMENT_SIZE {
    let random_id: usize = between.sample(&mut rng);
    tournament.save_individual(i, pop.clone_individual(random_id));
  }
  tournament.get_fittest().clone()
}
