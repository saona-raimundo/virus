use core::fmt::Display;
use crate::{BuildingBuilder, Building, Population, Individual, Recording};
use getset::Getters;
use strum::IntoEnumIterator;

/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Board {
    population: Population,
    buildings: Vec<Building>,
    inactive: Vec<Individual>, 
    #[getset(get = "pub")]
    recording: Recording,
}

impl Board {
	/// Returns the current state of the buildings
	pub fn buildings(&self) -> &Vec<Building> {
		&self.buildings
	}

	/// Advance the specified number of stages in the game.
	///
	/// # Remarks
	///
	/// This is equivalent to use `advance` many times.
	pub fn advance_many(&mut self, num_stages: usize) {
		for _ in 0..num_stages {
			self.advance()
		}
	}

	/// Advance a stage in the game.
	///
	/// # Remarks
	///
	/// This is a short method for all steps involved in a stage
	pub fn advance(&mut self) {
		self.visit();
		self.propagate();
		self.go_back();
	}

	/// First step of any stage
	///
	/// In this step, buildings are populated by non-sick individuals randomly.
	///
	/// # Errors
	///
	/// If visiting any of the building fails.
	pub fn visit(&mut self) -> &mut Self {
		self.population.shuffle(&mut rand::thread_rng());
		for index in 0..self.buildings.len() {
			self.visit_building(index);
		}
		loop {
			match self.population.next() {
				Some(i) => self.inactive.push(i),
				None => break,
			}
		}
		self
	}

	fn visit_building(&mut self, index: usize) -> &Building {
		while !self.buildings[index].is_full() & self.buildings[index].is_open() {
			match self.population.next() {
				Some(i) => {
					match i {
						Individual::Sick => self.inactive.push(i),
						i => self.buildings[index].try_push(i).expect("pushing on a building with space failed!"),
					}
				},
				None => break,
			}
		}
		&self.buildings[index]
	}

	/// Second step of any stage
	///
	/// In this step, virus is propagated in each building.
	pub fn propagate(&mut self) {
		for building in self.buildings.iter_mut() {
			building.propagate();
		}
	}

	/// Third step of any stage
	///
	/// In this step, the population returns home, changes are recorded and the number of newly infected is returned.
	pub fn go_back(&mut self) -> usize {
		let mut new_vec = Vec::new();
		// Collect from buildings
		for building in self.buildings.iter_mut() {
			new_vec.append(&mut building.empty())
		}
		let newly_infected: usize = new_vec.iter().filter(|&&i| i == Individual::Infected1).count();

		new_vec.append(&mut self.inactive);
		let new_population = Population::from(new_vec);

		// Update
		self.population = new_population;
		self.recording.register(newly_infected, &self.buildings);

		newly_infected
	}

	/// Closes a building
	pub fn close<S: Display>(&mut self, name: S) -> &mut Self {
		let name = name.to_string();
		for building in self.buildings.iter_mut() {
			if building.name() == &name {
				building.close();
			}
		}
		self
	}

	/// Opens a building
	pub fn open<S: Display>(&mut self, name: S) -> &mut Self {
		let name = name.to_string();
		for building in self.buildings.iter_mut() {
			if building.name() == &name {
				building.open();
			}
		}
		self
	}

	/// Returns the current state of the counting table
	pub fn counting_table(&self) -> Vec<(String, &Vec<usize>)> {
		Individual::iter().map(|i| (i.to_string(), &self.recording().counting_table()[&i])).collect()
	}
}

impl Default for Board {
	fn default() -> Self { 
		let population = Population::default();
		let concert_hall = BuildingBuilder::new("Concert Hall").sized(5, 4).build();
		let bakery = BuildingBuilder::new("Bakery").sized(2, 2).build();
		let school = BuildingBuilder::new("School").sized(4, 4).build();
		let pharmacy = BuildingBuilder::new("Pharmacy").sized(2, 2).build();
		let restaurant = BuildingBuilder::new("Restaurant").sized(3, 2).build();
		let gym = BuildingBuilder::new("Gym").sized(4, 2).build();
		let supermarket = BuildingBuilder::new("Supermarket").sized(2, 2).build();
		let shopping_center = BuildingBuilder::new("Shopping Center").sized(4, 2).build();
		let buildings = vec![
			concert_hall,
			bakery,
			school,
			pharmacy,
			restaurant,
			gym,
			supermarket,
			shopping_center,
		];
		let recording = Recording::new(population.clone(), buildings.clone());

		Board{ population, buildings, inactive: Vec::new(), recording }
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use ndarray::array;


	#[test]
	fn visit1() {
		let population = Population::from(vec![Individual::Healthy]);
		let buildings = vec![Building::unchecked_from(array![[None]]), Building::unchecked_from(array![[None]])];
		let default = Board::default();
		let mut board = Board{
			population,
			buildings,
			..default
		};
		board.visit();
		let expected = vec![Building::unchecked_from(array![[Individual::Healthy]]), Building::unchecked_from(array![[None]])];
		assert_eq!(board.buildings(), &expected);
	}

	#[test]
	fn visit_building1() {
		let population = Population::from(vec![Individual::Healthy]);
		let buildings = vec![Building::unchecked_from(array![[None]])];
		let default = Board::default();
		let mut board = Board{
			population,
			buildings,
			..default
		};
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[Individual::Healthy]]));
	}

	#[test]
	fn visit_building2() {
		let population = Population::from(vec![Individual::Sick]);
		let buildings = vec![Building::unchecked_from(array![[None]])];
		let default = Board::default();
		let mut board = Board{
			population,
			buildings,
			..default
		};
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[None]]));
	}

	#[test]
	fn visit_building3() {
		let population = Population::from(vec![Individual::Infected1, Individual::Healthy]);
		let buildings = vec![Building::unchecked_from(array![[Individual::Healthy]])];
		let default = Board::default();
		let mut board = Board{
			population,
			buildings,
			..default
		};
		board.visit_building(0);
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[Individual::Healthy]]));
	}

	#[test]
	fn propagate() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1]);
		let buildings = vec![Building::unchecked_from(array![[Individual::Healthy, Individual::Infected1]])];
		let default = Board::default();
		let mut board = Board {
			population,
			buildings,
			..default
		};
		board.propagate();
		assert_eq!(board.buildings()[0], Building::unchecked_from(array![[Individual::Infected1, Individual::Infected2]]));
	}

	#[test]
	#[should_panic]
	fn close() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1]);
		let buildings = vec![Building::new(2, 1, "My bulding")];
		let default = Board::default();
		let mut board = Board {
			population,
			buildings,
			..default
		};
		board.visit();
		board.close("My bulding");
	}
}