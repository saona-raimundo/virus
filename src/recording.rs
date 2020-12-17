
use std::collections::HashMap;
use crate::{Building, Individual, Population};
use getset::{Getters, MutGetters};
use strum::IntoEnumIterator;

/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
pub struct Recording {
	/// Returns a table with the counting of individual types per day.  
    #[getset(get = "pub", get_mut = "pub")]
    counting_table: HashMap<Individual, Vec<usize>>,
	/// Returns a table with the following information per day: Total infected, total sick and total healthy.  
    #[getset(get = "pub", get_mut = "pub")]
    diagram: [Vec<usize>; 3],
    /// Returns the current stage.  
    #[getset(get = "pub", get_mut = "pub")]
    timeline: usize, 
    /// Returns a table with the counting of penalty for each building per day.  
    #[getset(get = "pub", get_mut = "pub")]
    penalty: Vec<(Building, Vec<usize>)>,
    /// Returns the score obtained per day.  
    #[getset(get = "pub", get_mut = "pub")]
    daily_score: Vec<isize>,
}

impl Recording {

	/// Creates a new `Recording` with the population given for the initial state. 
	///
	/// Buildings are needed to keep track of penalties. 
	pub fn new(population: Population, buildings: Vec<Building>) -> Self { 
		let mut recording = Recording::default();
		for building in buildings {
			recording.penalty.push((building, vec![0]));
		}
		recording.counting_table = population.counting_all().iter().map(|(&i, &val)| (i, vec![val])).collect();
		recording.diagram = [
			vec![population.counting(Individual::Infected1) + population.counting(Individual::Infected2) + population.counting(Individual::Infected3)],
			vec![population.counting(Individual::Sick)],
			vec![population.counting(Individual::Healthy)],
		];

		recording
	}


	/// Increments the count of statges by one
	fn increment_time(&mut self) -> &mut Self {
		self.timeline += 1;
		self
	}

	/// Main functions that registers newly infected individuals
	///
	/// # Panics
	///
	/// If the number of newly infected is larger than the number of healthy individuals available
	pub fn register(&mut self, newly_infected: usize, _buildings: &Vec<Building>) -> &mut Self {
		self.register_diagram(newly_infected);
		self.register_counting_table(newly_infected);
		// self.register_penalty(buildings);
		// self.register_daily_score(buildings);
		self.increment_time();
		self
	}

	fn register_counting_table(&mut self, newly_infected: usize) {
	 	let last_values = self.last_counting_table();

		let counting_table = self.counting_table_mut();
	 	counting_table.entry(Individual::Healthy).and_modify(|v| v.push(last_values[0] - newly_infected));
	 	counting_table.entry(Individual::Infected1).and_modify(|v| v.push(newly_infected));
	 	counting_table.entry(Individual::Infected2).and_modify(|v| v.push(last_values[1]));
	 	counting_table.entry(Individual::Infected3).and_modify(|v| v.push(last_values[2]));
	 	counting_table.entry(Individual::Sick).and_modify(|v| v.push(last_values[3] + last_values[4]));
	 	counting_table.entry(Individual::Inmune).and_modify(|v| v.push(last_values[5]));
	}

	fn last_counting_table(&self) -> Vec<usize> {
	 	[
	 		self.counting_table()[&Individual::Healthy].last(),
	 		self.counting_table()[&Individual::Infected1].last(),
	 		self.counting_table()[&Individual::Infected2].last(),
	 		self.counting_table()[&Individual::Infected3].last(),
	 		self.counting_table()[&Individual::Sick].last(),
	 		self.counting_table()[&Individual::Inmune].last(),
	 	].iter().map(|x| x.unwrap().clone()).collect()
	}

	fn register_diagram(&mut self, newly_infected: usize) {
	 	let last_counting_table = self.last_counting_table();

		let diagram = self.diagram_mut();
		diagram[0].push(newly_infected + last_counting_table[1] + last_counting_table[2]);
		diagram[1].push(last_counting_table[3] + last_counting_table[4]);
		diagram[2].push(last_counting_table[0] - newly_infected);
	}
}



impl Default for Recording {
	// add code here
	fn default() -> Self { 
		let counting_table = Individual::iter().map(|i| (i, vec![0])).collect();
		let diagram = [vec![0], vec![0], vec![0]];
		let timeline = 0;
		let penalty = Vec::new();
		let daily_score = vec![0];

		Recording { counting_table, diagram, timeline, penalty, daily_score }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn register_counting_table1() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1, Individual::Infected3]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);

		assert_eq!(recording.last_counting_table(), vec![1, 1, 0, 1, 0, 0]);
		recording.register_counting_table(1);
		assert_eq!(recording.last_counting_table(), vec![0, 1, 1, 0, 1, 0]);
	}

	#[test]
	#[should_panic]
	fn register_counting_table2() {
		let population = Population::from(vec![Individual::Healthy]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);

		assert_eq!(recording.last_counting_table(), vec![1, 0, 0, 0, 0, 0]);
		recording.register_counting_table(2);
	}
}