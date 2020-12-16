
use crate::{Building, Population, Individual};

/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    population: Population,
    buildings: Vec<Building>,
    inactive: Vec<Individual>, 
}

impl Board {
	/// Returns the current state of the buildings
	pub fn buildings(&self) -> &Vec<Building> {
		&self.buildings
	}
	/// Advance a stage in the game.
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
	pub fn visit(&mut self) {
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
		// for i in self.population {
		// 	self.inactive.push(i);
		// }
	}

	fn visit_building(&mut self, index: usize) -> &Building {
		while !self.buildings[index].is_full() {
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
			building.propagate()
		}
	}

	/// Third step of any stage
	///
	/// In this step, the population returns home and changes are recorded.
	pub fn go_back(&mut self) {
		let mut new_population = Vec::new();
		// Collect from buildings
		for building in self.buildings.iter_mut() {
			new_population.append(&mut building.empty())
		}
		new_population.append(&mut self.inactive);
		todo!()
		// counting_table and diagram 
	}
}

impl Default for Board {
	fn default() -> Self { 
		let population = Population::default();
		let concert_hall = {
			let mut building = Building::new(5, 4);
			building.set_name("Concert Hall");
			building
		};
		let bakery = {
			let mut building = Building::new(2, 2);
			building.set_name("Bakery");
			building
		};
		let school = {
			let mut building = Building::new(4, 4);
			building.set_name("School");
			building
		};
		let pharmacy = {
			let mut building = Building::new(2, 2);
			building.set_name("Pharmacy");
			building
		};
		let restaurant = {
			let mut building = Building::new(3, 2);
			building.set_name("Restaurant");
			building
		};
		let gym = {
			let mut building = Building::new(4, 2);
			building.set_name("Gym");
			building
		};
		let supermarket = {
			let mut building = Building::new(2, 2);
			building.set_name("Supermarket");
			building
		};
		let shopping_center = {
			let mut building = Building::new(4, 2);
			building.set_name("Shopping Center");
			building
		};
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
		
		Board{ population, buildings, inactive: Vec::new()}
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use ndarray::array;

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
}