use crate::recording::CountingTable;
use core::fmt::Display;
use crate::{BuildingBuilder, Building, Population, Individual, Recording, building::Spreading};
use getset::{Getters, Setters, MutGetters};
use serde::{Serialize, Deserialize};

/// Builder for the `Board`.
///
/// # Remarks
///
/// Although `Board` can be constructed from `new` and `set_spreading`, this 
/// struct is specifically thought to be serialized and deserialized in a human-frindly way,
/// specially useful as a configuration file.
///   
/// A `Board` could be in the middle of a game, derefore (de)serialization 
/// turns out to be less human-friendly.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, Serialize, Deserialize, Default)]
pub struct BoardBuilder {
	/// Number of healthy individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub healthy: usize,
    /// Number of infected1 individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub infected1: usize,
    /// Number of infected2 individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub infected2: usize,
    /// Number of infected3 individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub infected3: usize,
    /// Number of sick individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub sick: usize,
    /// Number of inmune individuals
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub inmune: usize,
    /// Current state of the buildings in the game
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub buildings: Vec<(usize, usize)>,
    /// Spreading mode
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub spreading: Spreading,
}

impl BoardBuilder {
	pub fn build(self) -> Board {
		// Population
		let mut population_vec = vec![Individual::Healthy; self.healthy];
		population_vec.append(&mut vec![Individual::Infected1; self.infected1]);
		population_vec.append(&mut vec![Individual::Infected2; self.infected2]);
		population_vec.append(&mut vec![Individual::Infected3; self.infected3]);
		population_vec.append(&mut vec![Individual::Sick; self.sick]);
		population_vec.append(&mut vec![Individual::Inmune; self.inmune]);
		let population = Population::from(population_vec);

		// Buildings
		let buildings = self.buildings.iter().map(|&(cols, rows)| 
			BuildingBuilder::new("Defult")
				.with_size(cols, rows)
				.with_spreading(self.spreading)
				.and_is_open()
				.build()
			).collect();

		Board::new(population, buildings)
	}
}


/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
pub struct Board {
	/// Current population in the game
    #[getset(get = "pub")]
    population: Population,
    /// Current state of the buildings in the game
    #[getset(get = "pub")]
    buildings: Vec<Building>,
    inactive: Vec<Individual>, 
    /// Recording device
    #[getset(get = "pub", get_mut)]
    recording: Recording,
}

impl Board {
	/// Creates a new board with the specified population and buildings as default.
	pub fn new(population: Population, buildings: Vec<Building>) -> Self {
		let default = Board::default();
		let recording = Recording::new(population.clone(), buildings.clone());
		Board {
			population,
			buildings,
			recording,
			..default
		}
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
		let newly_infected = self.go_back();
		self.recording.register(newly_infected, &self.buildings);
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
	/// In this step, the population returns home. 
	/// Outputs the number of newly infected.
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

	/// Changes the spreading mode. 
	///
	/// See `Spreading` for more. 
	pub fn set_spreading(&mut self, new_spreading: Spreading) -> &mut Self {
		for building in self.buildings.iter_mut() {
			building.set_spreading(new_spreading);
		}
		self
	}

	/// Returns the current state of the counting table
	pub fn counting_table(&self) -> &CountingTable {
		self.recording().counting_table()
	}
}

impl Default for Board {
	/// Returns an instance of `Board` with default configuration
	///
	/// # Default
	///
	/// Some default values.
	/// ```
	/// # use virus_alarm::prelude::*;
	/// let board = Board::default();
	/// assert_eq!(board.population().len(), 100);
	/// assert_eq!(board.buildings().len(), 8);
	/// ```
	fn default() -> Self { 
		let population = Population::default();
		let concert_hall = BuildingBuilder::new("Concert Hall").with_size(5, 4).build();
		let bakery = BuildingBuilder::new("Bakery").with_size(2, 2).build();
		let school = BuildingBuilder::new("School").with_size(4, 4).build();
		let pharmacy = BuildingBuilder::new("Pharmacy").with_size(2, 2).build();
		let restaurant = BuildingBuilder::new("Restaurant").with_size(4, 3).build();
		let gym = BuildingBuilder::new("Gym").with_size(4, 2).build();
		let supermarket = BuildingBuilder::new("Supermarket").with_size(2, 2).build();
		let shopping_center = BuildingBuilder::new("Shopping Center").with_size(4, 2).build();
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
		let mut board = Board::new(population, buildings);
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[Individual::Healthy]]));
	}

	#[test]
	fn visit_building2() {
		let population = Population::from(vec![Individual::Sick]);
		let buildings = vec![Building::unchecked_from(array![[None]])];
		let mut board = Board::new(population, buildings);
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[None]]));
	}

	#[test]
	fn visit_building3() {
		let population = Population::from(vec![Individual::Infected1, Individual::Infected1]);
		let buildings = vec![Building::unchecked_from(array![[Individual::Healthy]])];
		let mut board = Board::new(population, buildings);
		assert_eq!(board.visit_building(0), &Building::unchecked_from(array![[Individual::Healthy]]));
	}

	#[test]
	fn propagate() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1]);
		let buildings = vec![Building::unchecked_from(array![[Individual::Healthy, Individual::Infected1]])];
		let mut board = Board::new(population, buildings);
		board.propagate();
		assert_eq!(board.buildings()[0], Building::unchecked_from(array![[Individual::Infected1, Individual::Infected2]]));
	}

	#[test]
	#[should_panic]
	fn close() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1]);
		let buildings = vec![Building::new(2, 1, "My bulding")];
		let mut board = Board::new(population, buildings);
		board.visit();
		board.close("My bulding");
	}
}