mod fitness_calc;
mod individual;
mod population;
mod algorithm;

// use individual::Individual;
use population::Population;

static SOLUTION: [i8; 15] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

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

    // Trying out Algorithm
    let my_pop = Population::new(20, true);

    let mut i = 0;
    println!(
        "Generation: {} Fittest: {} Genes: {:?}",
        i,
        my_pop.get_fittest().get_fitness(),
        my_pop.get_fittest().genes
    );
    algorithm::evolve_population(&my_pop, false);
    // i += 1;
}
