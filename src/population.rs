use std::collections::HashMap;
use crate::Individual;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

/// Population of the game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Population {
    population: Vec<Individual>,
    counter: usize
}

impl Population {
	/// change current population for `new_population`.
	///
	/// # Panics
	///
	/// If the size of the new population does not coincide with the original one.
	pub fn update(&mut self, new_population: Vec<Individual>) {
		assert_eq!(self.len(), new_population.len());
		self.population = new_population;
	}

	/// Shuffles (ie reorders in a random way) the population and restarts the iterator.
	///
	/// # Examples
	/// 
	/// ```
	/// # use virus_alarm::Population;
	/// let mut population = Population::default();
	/// population.shuffle(&mut rand::thread_rng());
	/// println!("A random sapmle (without replacement) is: {:?}", population.next());
	/// population.shuffle(&mut rand::thread_rng());
	/// println!("A fresh random sapmle (without replacement) is: {:?}", population.next());
	/// ```
	/// To draw a random individual from the population, 
	pub fn shuffle<R: ?Sized + rand::Rng>(&mut self, rng: &mut R) {
		let slice = self.population.as_mut_slice();
		slice.shuffle(rng);
		self.counter = 0;
	}

	/// Returns the size of the population
	pub fn len(&self) -> usize {
		self.population.len()
	}

	/// Returns the number of individuals of the given type.
	pub fn counting(&self, query: Individual) -> usize {
		self.population.iter().filter(|&&i| i == query).count()
	}

	/// Returns the number of individuals of each type.
	pub fn counting_all(&self) -> HashMap<Individual, usize> {
		let mut hm = HashMap::new();
		for query in Individual::iter() {
			hm.insert(query, self.counting(query));
		}
		hm
	}

}

impl Default for Population {
	// add code here
	fn default() -> Self { 
		let mut population = vec![Individual::Healthy; 98];
		population.push(Individual::Infected1);
		population.push(Individual::Infected1);

		Population{ population, counter: 0 }
	}
}

impl From<Vec<Individual>> for Population {
	fn from(vec: Vec<Individual>) -> Self { Population{ population: vec, counter: 0 } }
}

impl Iterator for Population {
	type Item = Individual;
	fn next(&mut self) -> Option<Self::Item> {
		if self.counter < self.len() {
			self.counter += 1;
			Some(self.population[self.counter - 1])
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn shuffle() {
		let mut population = Population::default();
		population.shuffle(&mut crate::tests::rng(1));
		let shuffled = {
			let mut vec = vec![Individual::Healthy; 100];
			vec[4] = Individual::Infected1;
			vec[96] = Individual::Infected1;
			Population::from(vec)
		};
		assert_eq!(shuffled, population);
	}

	#[test]
	fn counting1() {
		let population = Population::default();
		assert_eq!(population.counting(Individual::Healthy), 98);
		assert_eq!(population.counting(Individual::Infected1), 2);
	}

	#[test]
	fn counting_all() {
		let population = Population::default();
		let hm = population.counting_all();
		assert_eq!(hm[&Individual::Healthy], 98);
		assert_eq!(hm[&Individual::Infected1], 2);
		assert_eq!(hm[&Individual::Infected2], 0);
		assert_eq!(hm[&Individual::Infected3], 0);
		assert_eq!(hm[&Individual::Sick], 0);
		assert_eq!(hm[&Individual::Inmune], 0);
	}
}