use std::collections::HashMap;
use crate::{Building, Individual, Population, prelude::Spreading};
use getset::{Getters, MutGetters};
use strum::IntoEnumIterator;


mod counting_table;
pub use counting_table::*;

/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
pub struct Recording {
	/// Returns a "table" with the counting of individual types per day.
	///
	/// The quantity of each individual type present in the population is counted and 
	/// the vector of numbers represents the count for each of the days that have passed.
    #[getset(get = "pub", get_mut)]
    counting_table: CountingTable,
    /// Returns the current stage.  
    #[getset(get = "pub", get_mut)]
    timeline: usize, 
    /// Returns a table with the counting of penalty for each building per day.  
    #[getset(get = "pub", get_mut)]
    penalty: Vec<(Building, Vec<usize>)>,
    /// Returns the score obtained per day.  
    #[getset(get = "pub", get_mut)]
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

		recording
	}

	/// Returns `true` if the outbreak of the virus is contained in the last day.
	///
	/// An outbreak  is contained if two conditions hold:
	/// - There is no individual who can infect another
	/// - There is at least one non-sick person (could be vaccinated or healthy unvaccinated)
	///
	/// # Panics
	///
	/// If the counting tabe is empty.
	///
	/// # Examples
	///
	/// A population with a contained outbreak.
	/// ```
	/// # use virus_alarm::prelude::*;
	/// # use virus_alarm::Recording;
	/// let population = Population::from(vec![Individual::Healthy, Individual::Sick, Individual::Immune]);
	/// let buildings = Vec::new();
	/// let recording = Recording::new(population, buildings);
	/// assert_eq!(recording.is_contained(), true);
	/// ```
	///
	/// A population with an outbreak yet to be contained.
	/// ```
	/// # use virus_alarm::prelude::*;
	/// # use virus_alarm::Recording;
	/// let population = Population::from(vec![Individual::Infected1, Individual::Sick, Individual::Immune]);
	/// let buildings = Vec::new();
	/// let recording = Recording::new(population, buildings);
	/// assert_eq!(recording.is_contained(), false);
	/// ```
	pub fn is_contained(&self) -> bool {
		self.counting_table().is_contained()
	}

	/// Increments the count of statges by one
	fn increment_time(&mut self) -> &mut Self {
		self.timeline += 1;
		self
	}

	/// Changes the spreading mode. 
	///
	/// See `Spreading` for more. 
	pub(crate) fn set_spreading(&mut self, new_spreading: Spreading) -> &mut Self {
		for (building, _) in self.penalty_mut().iter_mut() {
			building.set_spreading(new_spreading);
		}
		self
	}

	/// Immunize one person in the population. 
	/// 
	/// # Errors
	///
	/// If there is no healthy individual to immunize.
	pub(crate) fn immunize(&mut self) -> Result<&mut Self, crate::errors::ActionError> {
		let hm = self.counting_table_mut().inner_mut();
		let healthy_last = hm.get_mut(&Individual::Healthy).unwrap().last_mut().unwrap();
		if healthy_last > &mut 0 {
			*healthy_last -= 1;
			let immune_last = hm.get_mut(&Individual::Immune).unwrap().last_mut().unwrap();
			*immune_last += 1;
			Ok(self)
		} else {
			Err(crate::errors::ActionError::NoHealthyLeft)
		}
		
	}

	/// Reverse one individual from immune to healthy in the population. 
	/// 
	/// # Errors
	///
	/// If there is no immune individual to reverse.
	pub(crate) fn reverse_immunize(&mut self) -> Result<&mut Self, crate::errors::ActionError> {
		let hm = self.counting_table_mut().inner_mut();
		let immune_last = hm.get_mut(&Individual::Immune).unwrap().last_mut().unwrap();
		if immune_last > &mut 0 {
			*immune_last -= 1;
			let healthy_last = hm.get_mut(&Individual::Healthy).unwrap().last_mut().unwrap();
			*healthy_last += 1;
			Ok(self)
		} else {
			Err(crate::errors::ActionError::NoImmuneLeft)
		}
		
	}

	/// Returns a "table" with the following information per day: Total healthy, total sick and total infected.  
	///
	/// The information provided in this table is the total number of 
	/// infected, sick and healthy individuals respectively for each day that has been recorded.
	pub fn diagram(&self) -> [Vec<usize>; 3] {
		self.counting_table().diagram()
	}

	/// Main functions that registers newly infected individuals
	///
	/// # Panics
	///
	/// If the number of newly infected is larger than the number of healthy individuals available
	pub(crate) fn register(&mut self, newly_infected: usize, _buildings: &[Building]) -> &mut Self {
		self.register_counting_table(newly_infected);
		// self.register_penalty(buildings);
		// self.register_daily_score(buildings);
		self.increment_time();
		self
	}

	fn register_counting_table(&mut self, newly_infected: usize) {
	 	let last_values = self.last_day_individuals();

		let counting_table = self.counting_table_mut();
	 	counting_table.inner_mut().entry(Individual::Healthy).and_modify(|v| v.push(last_values[&Individual::Healthy] - newly_infected));
	 	counting_table.inner_mut().entry(Individual::Infected1).and_modify(|v| v.push(newly_infected));
	 	counting_table.inner_mut().entry(Individual::Infected2).and_modify(|v| v.push(last_values[&Individual::Infected1]));
	 	counting_table.inner_mut().entry(Individual::Infected3).and_modify(|v| v.push(last_values[&Individual::Infected2]));
	 	counting_table.inner_mut().entry(Individual::Sick).and_modify(|v| v.push(last_values[&Individual::Infected3] + last_values[&Individual::Sick]));
	 	counting_table.inner_mut().entry(Individual::Immune).and_modify(|v| v.push(last_values[&Individual::Immune]));
	}

	/// # Panics
	///
	/// If the counting table is empty.
	fn last_day_individuals(&self) -> HashMap<Individual, usize> {
		self.counting_table().last_day()
	}
}



impl Default for Recording {
	// add code here
	fn default() -> Self { 
		let counting_table = Individual::iter().map(|i| (i, vec![0])).collect();
		// let diagram = [vec![0], vec![0], vec![0]];
		let timeline = 0;
		let penalty = Vec::new();
		let daily_score = vec![0];

		Recording { counting_table, timeline, penalty, daily_score }
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	

	#[test]
	fn immunize() {
		let population = Population::from(vec![Individual::Healthy, Individual::Sick, Individual::Immune]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);
		assert_eq!(recording.counting_table().inner()[&Individual::Healthy], vec![1]);
		assert_eq!(recording.counting_table().inner()[&Individual::Immune], vec![1]);
		recording.immunize().unwrap();
		assert_eq!(recording.counting_table().inner()[&Individual::Healthy], vec![0]);
		assert_eq!(recording.counting_table().inner()[&Individual::Immune], vec![2]);
	}

	#[test]
	fn reverse_immunize() {
		let population = Population::from(vec![Individual::Healthy, Individual::Sick, Individual::Immune]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);
		assert_eq!(recording.counting_table().inner()[&Individual::Healthy], vec![1]);
		assert_eq!(recording.counting_table().inner()[&Individual::Immune], vec![1]);
		recording.reverse_immunize().unwrap();
		assert_eq!(recording.counting_table().inner()[&Individual::Healthy], vec![2]);
		assert_eq!(recording.counting_table().inner()[&Individual::Immune], vec![0]);
	}
}