use population::Population;

fn evolve_population(pop: Population, elitism: bool) {
  let new_pop = Population::new(pop.size(), false);

  let mut index = 0;
  while index < pop.size() {
    index += 1;
  }
}

fn tournament_Selection(pop: Population, tournament_size: usize) {
  let tournament = Population::new(tournament_size, false);
}
