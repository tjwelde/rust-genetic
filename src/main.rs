mod algorithm;
mod fitness_calc;
mod individual;
mod population;

// // use individual::Individual;
use population::Population;

// static SOLUTION: [i8; 15] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

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

  let solution: Vec<i8> = vec![
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
  ];

  println!("Max fitness: {}", fitness_calc::get_max_fitness(&solution));

  // Trying out Algorithm
  let mut my_pop = Population::new(20, true, &solution);

  let mut i = 0;
  loop {
    println!(
      "Generation: {:2} Fittest: {} Genes: {:?}",
      i,
      my_pop.get_fittest().get_fitness(),
      my_pop.get_fittest().genes
    );
    my_pop = algorithm::evolve_population(&my_pop, false, &solution);
    i += 1;
    if i > 20 {
      break;
    };
  }
}
