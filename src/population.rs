use fitness_calc;
use individual::Individual;

pub struct Population {
    individuals: Vec<Individual>,
    population_size: usize,
}

impl Population {
    pub fn new(population_size: usize, initialize: bool) -> Population {
        let mut pop = Population {
            population_size: population_size,
            individuals: Vec::with_capacity(population_size),
        };

        if initialize {
            for _i in 0..population_size {
                let newIndiv = Individual::new();
                pop.save_individual(_i, newIndiv);
            }
        }
        pop
    }

    pub fn save_individual(&mut self, i: usize, indiv: Individual) {
        self.individuals[i] = indiv;
    }

    pub fn get_individual(&self, index: usize) -> &Individual {
        &self.individuals[index]
    }

    pub fn get_fittest(&self) -> &Individual {
        let mut fittest = 0;
        let mut index = 0;

        for indiv in &self.individuals {
            if fitness_calc::get_fitness(&self.individuals[fittest])
                <= fitness_calc::get_fitness(&indiv)
            {
                fittest = index;
            }
            index += 1;
        }
        &self.individuals[fittest]
    }

    pub fn size(&self) -> usize {
        self.population_size
    }
}
