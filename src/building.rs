use core::fmt::Display;
use core::convert::TryFrom;
use crate::errors::BuildingError;
use crate::Individual;
use gamma::graph::DefaultGraph;
use ndarray::Array2;
use serde::{Serialize, Deserialize};

/// Spreding mode inside a building.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Spreding {
    /// If there is one person infected in the building, then everyone is infected
    Everyone,
    /// Each infected person infects at least one healthy person, regardless of spatial structure
    One,
    /// Infected individuals try to infect someone near to them considering spatial structure
    ///
    /// Close individuals are does that are in distance one verticaly, horizontaly or diagonaly. 
    /// Also, as there can be more than one infected per building, they work collectively and infect
    /// as much people as possible, under the restriction that each of them infects only one other individual.
    OneNear,
}

impl Default for Spreding {
    fn default() -> Self { 
        Spreding::OneNear
    }
}


/// Builder struct for `Building`.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BuildingBuilder {
    people: Array2<Option<Individual>>,
    spreding: Spreding,
    name: String,
    penalty: usize,
    open: bool, 
}

impl BuildingBuilder {

    /// Creates a new and empty building builder
    pub fn new<S: Display>(name: S) -> Self {
        let default = BuildingBuilder::default();
        BuildingBuilder { 
            name: name.to_string(),
            ..default
        }
    }

    /// Changes the size of the building
    pub fn with_size(mut self, columns: usize, rows: usize) -> Self {
        self.people = Array2::from_elem((rows, columns), None);
        self
    }

    /// Changes the size of the building
    pub fn with_penalty(mut self, penalty: usize) -> Self {
        self.penalty = penalty;
        self
    }

    /// Changes the spreding mode of the building
    pub fn with_spreding(mut self, new_spreding: Spreding) -> Self {
        self.spreding = new_spreding;
        self
    }

    /// Opens the building
    pub fn and_is_open(mut self) -> Self {
        self.open = true;
        self
    }

    /// Closes the building
    pub fn and_is_close(mut self) -> Self {
        self.open = false;
        self
    }


    /// Returns the corresponding building
    pub fn build(self) -> Building {
        Building {
            people: self.people,
            spreding: self.spreding,
            name: self.name,
            penalty: self.penalty,
            open: self.open,
        }
    }
}

impl Default for BuildingBuilder {
    fn default() -> Self { 
        BuildingBuilder{
            people: Array2::from_elem((0, 0), None),
            spreding: Spreding::OneNear,
            name: String::from("Default"),
            penalty: 0,
            open: true,
        }
    }
}

/// Building in the board game where spreading can happen.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Building {
    people: Array2<Option<Individual>>,
    spreding: Spreding,
    name: String,
    penalty: usize,
    open: bool,
}

impl Building {
	/// Creates a new and empty building
	///
	/// The default mode of propagation is `OneNear`, see `spreding` method for more.
	pub fn new<S: Display>(columns: usize, rows: usize, name: S) -> Self {
        let default = Building::default();
		Building{ 
			people: Array2::from_elem((rows, columns), None),
            name: name.to_string(),
			..default
		}
	}

    /// Returns true if the building is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns true if the building is close
    pub fn is_close(&self) -> bool {
        !self.open
    }

    /// Opens the building
    pub fn open(&mut self) -> &mut Self {
        self.open = true;
        self
    }

    /// Closes the building
    ///
    /// # Panics
    ///
    /// If the building is not empty
    pub fn close(&mut self) {
        assert!(self.is_empty());
        self.open = false;
    }

    /// Rerturns the penalty of the building, which is the cost of closing the building
    pub fn penalty(&self) -> &usize {
        &self.penalty
    }

    /// Changes the penalty of the building
    pub fn set_penalty(&mut self, new_penalty: usize) -> &mut Self {
        self.penalty = new_penalty;
        self
    }

    /// Returns the name of the building
    ///
    /// The default value is "Default".
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Changes the name of the building
    ///
    /// # Examples
    ///
    /// ```
    /// # use virus_alert::Building;
    /// let mut building = Building::default();
    /// building.set_name("My name");
    /// assert_eq!(building.name(), &"My name".to_string());
    /// ```
    pub fn set_name<S: Display>(&mut self, name: S) -> &mut Self {
        self.name = name.to_string();
        self
    }
	/// Returns the people who are currently in the building
	pub fn people(&self) -> &Array2<Option<Individual>> {
		&self.people
	}
	/// Returns the spreding mode of the building
	pub fn spreding(&self) -> &Spreding {
		&self.spreding
	}
	/// Sets the spreding mode of the building
	pub fn set_spreding(&mut self, new_spreding: Spreding) -> &mut Self {
		self.spreding = new_spreding;
        self
	}
    /// Return the shape of the array as a slice.
    pub fn shape(&self) -> &[usize] {
        self.people().shape()
    }
    /// Return the total capacity of the building, ie the number of individual it can host.
    pub fn capacity(&self) -> usize {
        self.people().shape().iter().product()
    }
    /// Checks if the building can not accept more people, ie is full.
    pub fn is_full(&self) -> bool {
        self.people().iter().all(|i| i.is_some())
    }
    /// Checks if the building is empty more people.
    pub fn is_empty(&self) -> bool {
        self.people().iter().all(|i| i.is_none())
    }
    /// Empties the building of people, returning the individuals that were inside
    pub fn empty(&mut self) -> Vec<Individual> {
        let vec: Vec<Individual> = self
            .people
            .clone()
            .into_iter()
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .collect();
        self.people.mapv_inplace(|_| -> Option<Individual> { None });
        vec    
    }
    /// Appends an individual to the first available position in the building.
    ///
    /// # Errors
    ///
    /// If the building is already full or the individual is sick.
    pub fn try_push(&mut self, individual: Individual) -> Result<(), BuildingError> {
        if self.is_full() {
            Err(BuildingError::Full)?
        } if individual == Individual::Sick {
        	Err(BuildingError::Sick)
        } else {
            for i in self.people.iter_mut() {
                if i.is_none() {
                    *i = Some(individual);
                    break;
                }
            }
            Ok(())
        }
    }

    /// Propagates the infection
    pub fn propagate(&mut self) -> &mut Self {
    	match self.spreding {
    		Spreding::Everyone => self.propagate_everyone(),
    		Spreding::One => self.propagate_one(),
    		Spreding::OneNear => self.propagate_onenear()
    	}
        self
    }

    /// Propagates by infecting one healthy individual per infected indiviual, if possible
    fn propagate_one(&mut self) {
    	let mut counter = 0;
    	for i in self.people.iter() {
    		if let Some(i) = i {
    			match i {
    				Individual::Infected1 | Individual::Infected2 | Individual::Infected3 => counter += 1,
    				_ => (),
    			}
    		};
    	}
		self.people.mapv_inplace(|i| {
			match i {
				Some(Individual::Healthy) => {
					if counter > 0 {
						counter -= 1;
						Some(Individual::Infected1)
					} else {
						Some(Individual::Healthy)
					}
				},
				Some(Individual::Infected1) => Some(Individual::Infected2),
                Some(Individual::Infected2) => Some(Individual::Infected3),
                Some(Individual::Infected3) => Some(Individual::Sick),
                Some(Individual::Sick) => panic!("There should not have been a sick person in the building"),
                Some(Individual::Inmune) => Some(Individual::Inmune),
                None => None,
			}
		});
    }

    /// Propagates by setting all healthy individuals to infected, if there is any infected in the building
    fn propagate_everyone(&mut self) {
    	let mut infect_everyone = false;
    	for i in self.people.iter() {
    		if let Some(i) = i {
    			match i {
    				Individual::Infected1 | Individual::Infected2 | Individual::Infected3 => infect_everyone = true,
    				_ => (),
    			}
    		};
    	}
    	
		self.people.mapv_inplace(|i| {
			match i {
				Some(Individual::Healthy) => {
					if infect_everyone {
						Some(Individual::Infected1)
					} else {
						Some(Individual::Healthy)
					}
				},
				Some(Individual::Infected1) => Some(Individual::Infected2),
                Some(Individual::Infected2) => Some(Individual::Infected3),
                Some(Individual::Infected3) => Some(Individual::Sick),
                Some(Individual::Sick) => panic!("There should not have been a sick person in the building"),
                Some(Individual::Inmune) => Some(Individual::Inmune),
                None => None,
			}
		});
    }

    /// Propagates by choosing a maximum matching between infected and healthy individuals
    fn propagate_onenear(&mut self) {
        let graph: DefaultGraph = self.clone().into();
        let mut pairing = gamma::matching::Pairing::new();

        gamma::matching::maximum_matching(&graph, &mut pairing);

        let rows = self.people().nrows();
        let columns = self.people().ncols();
        for col in 0..columns {
            for row in 0..rows {
                if let Some(i) = self.people()[[row, col]] {
                    self.people[[row, col]] = Some(match i {
                        Individual::Healthy => {
                            if pairing.has_node(col + row * columns) {
                                Individual::Infected1
                            } else {
                                Individual::Healthy
                            }
                        }
                        Individual::Infected1 => Individual::Infected2,
                        Individual::Infected2 => Individual::Infected3,
                        Individual::Infected3 => Individual::Sick,
                        Individual::Sick => panic!("There should not have been a sick person in the building"),
                        Individual::Inmune => Individual::Inmune,
                    });
                }
            }
        }
    }

    pub fn unchecked_from<T>(array: Array2<T>) -> Self 
    where
        T: Into<Option<Individual>> + Clone,
    {
        let default = Building::default();
        Building { 
            people: array.mapv(|i| i.into()) ,
            ..default
        }
    }
}

impl Default for Building {
    fn default() -> Self { 
        BuildingBuilder::default().build()
    }
}

impl TryFrom<Array2<Option<Individual>>> for Building {
	type Error = BuildingError;
    fn try_from(array: Array2<Option<Individual>>) -> Result<Self, Self::Error> {
        for i in array.iter() {
        	if let Some(Individual::Sick) = i {
        		Err(BuildingError::Sick)?
        	}
        }
        Ok(Building::unchecked_from(array))
    }
}

impl TryFrom<Array2<Individual>> for Building {
	type Error = BuildingError;
    fn try_from(array: Array2<Individual>) -> Result<Self, Self::Error> {
        Building::try_from(array.map(|&i| Some(i)))
    }
}

impl Into<DefaultGraph> for Building {
    fn into(self) -> DefaultGraph {
        let mut graph = DefaultGraph::new();
        let rows = self.people().nrows();
        let columns = self.people().ncols();
        // Add nodes
        for col in 0..columns {
            for row in 0..rows {
                if self.people()[[row, col]].is_some() {
                    graph.add_node(col + row * columns).unwrap()
                }
            }
        }
        // Add edges
        for col in 0..columns {
            for row in 0..rows {
                if let Some(i) = self.people()[[row, col]] {
                	// Horizontal
                    if col > 0 {
                        if let Some(j) = self.people()[[row, col - 1]] {
                            if i.interacts_with(&j) {
                                graph
                                    .add_edge(col + row * columns, (col - 1) + row * columns)
                                    .unwrap()
                            }
                        }
                    }
                    // Vertical
                    if row > 0 {
                        if let Some(j) = self.people()[[row - 1, col]] {
                            if i.interacts_with(&j) {
                                graph
                                    .add_edge(col + row * columns, col + (row - 1) * columns)
                                    .unwrap()
                            }
                        }
                    }
                    // Diagonal
                    if col > 0 && row > 0 {
                    	if let Some(j) = self.people()[[row - 1, col - 1]] {
                            if i.interacts_with(&j) {
                                graph
                                    .add_edge(col + row * columns, (col - 1) + (row - 1) * columns)
                                    .unwrap()
                            }
                        }
                    }
                    if col > 0 && row < rows - 1 {
                    	if let Some(j) = self.people()[[row + 1, col - 1]] {
                            if i.interacts_with(&j) {
                                graph
                                    .add_edge(col + row * columns, (col - 1) + (row + 1) * columns)
                                    .unwrap()
                            }
                        }
                    }
                }
            }
        }
        graph
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	// use test_case::test_case;
	use ndarray::array;

	#[test]
	fn construction() {
		let array = array![[Individual::Healthy, Individual::Infected1], [Individual::Healthy, Individual::Infected1]];
		let building = Building::try_from(array.clone()).expect("There is a sick one!");
		assert_eq!(building.people(), &array.map(|&i| Some(i)));
		assert_eq!(building.capacity(), 4);
		assert!(building.is_full());
		assert_eq!(building.spreding(), &Spreding::OneNear);
	}

	#[test]
	fn shape() {
		let array = array![[Individual::Healthy, Individual::Infected1], [Individual::Healthy, Individual::Infected1]];
		let building = Building::try_from(array.clone()).expect("There is a sick one!");
		assert_eq!(building.shape(), [2, 2]);
		assert_eq!(Building::new(3, 4, "").shape(), [4, 3]);
		
	}

	#[test]
	fn is_empty() {
		let array = array![[None, None], [None, None]];
		let building = Building::try_from(array).expect("There is a sick one!");
		assert!(building.is_empty());
		assert!(Building::new(3, 4, "").is_empty());
	}

	#[test]
	fn empty() {
		let array = array![[Individual::Healthy, Individual::Infected1], [Individual::Healthy, Individual::Infected1]];
		let mut building = Building::try_from(array).expect("There is a sick one!");
		building.empty();
		assert!(building.is_empty());
	}

	#[test]
	fn try_push() {
		let array = array![[Individual::Healthy, Individual::Infected1], [Individual::Healthy, Individual::Infected1]];
		let mut building = Building::try_from(array).expect("There is a sick one!");
		assert_eq!(building.try_push(Individual::Healthy), Err(BuildingError::Full));

		let mut building = Building::new(1, 1, "");
		assert!(!building.is_full());
		building.try_push(Individual::Healthy).expect("can not push when it should!");
		assert!(building.is_full());

		let mut building = Building::new(2, 1, "");
		building.try_push(Individual::Healthy).expect("can not push when it should!");
		assert_eq!(building.people(), &array![[Some(Individual::Healthy), None]]);
	}

	#[test]
	fn propagate_everyone1() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected1] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected2] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::Everyone);
		initial.propagate();
		expected.set_spreding(Spreding::Everyone);
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_everyone2() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected1] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::Everyone);
		initial.propagate();
		expected.set_spreding(Spreding::Everyone);
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_everyone3() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Infected1] 
    	]);
		initial.set_spreding(Spreding::Everyone);
		expected.set_spreding(Spreding::Everyone);
        initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_everyone4() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]);
		initial.set_spreding(Spreding::Everyone);
        expected.set_spreding(Spreding::Everyone);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_everyone5() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]);
        initial.set_spreding(Spreding::Everyone);
        expected.set_spreding(Spreding::Everyone);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_one1() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected1] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected2] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::One);
		expected.set_spreding(Spreding::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_one2() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::One);
		expected.set_spreding(Spreding::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_one3() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Healthy] 
		]);
        initial.set_spreding(Spreding::One);
        expected.set_spreding(Spreding::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_one4() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]);
        initial.set_spreding(Spreding::One);
        expected.set_spreding(Spreding::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_one5() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]);
        initial.set_spreding(Spreding::One);
        expected.set_spreding(Spreding::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_onenear1() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected1] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected2] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::OneNear);
		expected.set_spreding(Spreding::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_onenear2() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::try_from(array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		initial.set_spreding(Spreding::OneNear);
		expected.set_spreding(Spreding::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_onenear3() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Healthy] 
		]);
        initial.set_spreding(Spreding::OneNear);
        expected.set_spreding(Spreding::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_onenear4() {
		let mut initial = Building::try_from(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]);
        initial.set_spreding(Spreding::OneNear);
        expected.set_spreding(Spreding::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test]
	fn propagate_onenear5() {
		let mut initial = Building::try_from(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		]).expect("There is a sick one!");
		let mut expected = Building::unchecked_from(array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]);
        initial.set_spreding(Spreding::OneNear);
        expected.set_spreding(Spreding::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}
}