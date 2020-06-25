mod algorithm;
mod fitness_calc;
mod individual;
mod population;

// // use individual::Individual;
use population::Population;

static SOLUTION: [i8; 60] = [
  0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1,
  0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
];

fn main() {
  // Trying out setters and getters
  //let mut indiv1 = Individual::new();
  // println!("genes #5: {}, fitness: {}", indiv1.genes[4], indiv1.fitness);
  // indiv1.set_gene(4, 8);
  // let fitness = { indiv1.get_fitness() };
  // println!("genes #5: {}, fitness: {}", indiv1.genes[4], fitness);

  // Trying out fitness calculation
  // let indiv = Individual::new();
  // let fitness = fitness_calc::get_fitness(&indiv);
  // println!("{}", indiv.to_string());
  // println!("fitness: {}", fitness);

  println!("Max fitness: {}", fitness_calc::get_max_fitness(&SOLUTION));

  // Trying out Algorithm
  let mut my_pop = Population::new(50, true, &SOLUTION);

  let mut last = 0;
  for i in 0..400 {
    last = i;
    println!(
      "Generation: {:2} Fittest: {}",
      i,
      my_pop.get_fittest().get_fitness()
    );
    my_pop = algorithm::evolve_population(&my_pop, &SOLUTION);
    if my_pop.get_fittest().get_fitness() == fitness_calc::get_max_fitness(&SOLUTION) {
      break;
    }
  }
  println!(
    "Generation: {:2} Fittest: {}",
    last,
    my_pop.get_fittest().get_fitness()
  );
}
