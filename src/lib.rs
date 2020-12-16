//! Virus alert simulation package.
//!
//! This crate allows to simulate and study the dynamics defined in the
//! [Virus Alert](https://ist.ac.at/en/education/ist-for-kids/virus-alert/) educational board game.
//!

pub use building::*;
pub use individual::*;

mod individual {
    /// Individual in the game, it represents a person.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Individual {
        /// Healthy vulnerable person
        Healthy,
        /// Infected person in its first day
        Infected1,
        /// Infected person in its second day
        Infected2,
        /// Infected person in its third (and last) day
        Infected3,
        /// Sick person, who goes to the hospital
        Sick,
        /// Vaccinated, and therefore inmune, person
        Inmune,
    }

    impl Individual {
        /// Return true if `other` can be infected by `self`.
        ///
        /// This is only possible if self is infected and other is healthy.
        pub fn can_infect(&self, other: &Individual) -> bool {
            match self {
                Individual::Healthy | Individual::Sick | Individual::Inmune => false,
                _ => match other {
                    Individual::Healthy => true,
                    _ => false,
                },
            }
        }

        /// Returns true if either can infect the other.
        pub fn interacts_with(&self, other: &Individual) -> bool {
            self.can_infect(other) || other.can_infect(self)
        }
    }

    #[cfg(test)]
    mod tests {
    	use super::*;
    	use test_case::test_case;
    
    	#[test_case(Individual::Healthy, Individual::Infected1, false)]
    	#[test_case(Individual::Infected1, Individual::Healthy, true)]
    	#[test_case(Individual::Infected2, Individual::Healthy, true)]
    	#[test_case(Individual::Infected3, Individual::Healthy, true)]
    	#[test_case(Individual::Infected2, Individual::Inmune, false)]
    	fn can_infect(i: Individual, other: Individual, expected: bool) {
    		assert_eq!(i.can_infect(&other), expected);
    	}

    	#[test_case(Individual::Healthy, Individual::Infected1, true)]
    	#[test_case(Individual::Infected1, Individual::Healthy, true)]
    	#[test_case(Individual::Infected2, Individual::Healthy, true)]
    	#[test_case(Individual::Infected3, Individual::Healthy, true)]
    	#[test_case(Individual::Infected2, Individual::Inmune, false)]
    	#[test_case(Individual::Inmune, Individual::Inmune, false)]
    	fn interacts_with(i: Individual, other: Individual, expected: bool) {
    		assert_eq!(i.interacts_with(&other), expected);
    	}

    	
    }
}

mod building {
    use core::convert::TryFrom;
	use crate::errors::BuildingError;
    use crate::Individual;
    use gamma::graph::DefaultGraph;
    use ndarray::Array2;

    /// Spreding mode inside a building.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Building {
        people: Array2<Option<Individual>>,
        spreding: Spreding,
    }

    impl Building {
    	/// Creates a new and empty building
    	///
    	/// The default mode of propagation is `OneNear`, see `spreding` method for more.
    	pub fn new(columns: usize, rows: usize) -> Self {
    		Building{ 
    			people: Array2::from_elem((rows, columns), None),
    			spreding: Spreding::OneNear,
    		}
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
    	pub fn set_spreding(&mut self, new_spreding: Spreding) {
    		self.spreding = new_spreding;
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
        pub fn empty(&mut self) -> Option<Vec<Individual>> {
            if self.is_empty() {
                None
            } else {
                let vec: Vec<Individual> = self
                    .people
                    .clone()
                    .into_iter()
                    .filter(|i| i.is_some())
                    .map(|i| i.unwrap())
                    .collect();
                self.people = self.people.map_mut(|_| -> Option<Individual> { None });
                Some(vec)
            }
        }
        /// Appends an individual to the first available position in the building.
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
        pub fn propagate(&mut self) {
        	match self.spreding {
        		Spreding::Everyone => self.propagate_everyone(),
        		Spreding::One => todo!(),
        		Spreding::OneNear => self.propagate_onenear()
        	}
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
        	if infect_everyone {
        		self.people = self.people.map_mut(|i| {
        			match i {
        				Some(Individual::Healthy) => Some(Individual::Infected1),
        				Some(Individual::Infected1) => Some(Individual::Infected2),
                        Some(Individual::Infected2) => Some(Individual::Infected3),
                        Some(Individual::Infected3) => Some(Individual::Sick),
                        Some(Individual::Sick) => panic!("There should not have been a sick person in the building"),
                        Some(Individual::Inmune) => Some(Individual::Inmune),
                        None => None,
        			}
        		});
        	}

        }

        /// Propagates by choosing a maximum matching between infected and healthy individuals
        fn propagate_onenear(&mut self) {
            let graph: DefaultGraph = self.clone().into();
            let mut pairing = gamma::matching::Pairing::new();

            gamma::matching::maximum_matching(&graph, &mut pairing);

            let rows = self.people().nrows();
            let columns = self.people().ncols();
            for x in 0..columns {
                for y in 0..rows {
                    if let Some(i) = self.people()[[x, y]] {
                        self.people[[x, y]] = Some(match i {
                            Individual::Healthy => {
                                if pairing.has_node(x + y * columns) {
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
    }

    impl TryFrom<Array2<Option<Individual>>> for Building {
    	type Error = BuildingError;
        fn try_from(array: Array2<Option<Individual>>) -> Result<Self, Self::Error> {
            for i in array.iter() {
            	if let Some(Individual::Sick) = i {
            		Err(BuildingError::Sick)?
            	}
            }
            Ok(Building { 
            	people: array,
    			spreding: Spreding::OneNear,
    		})
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
            for x in 0..columns {
                for y in 0..rows {
                    if self.people()[[x, y]].is_some() {
                        graph.add_node(x + y * columns).unwrap()
                    }
                }
            }
            // Add edges
            for x in 0..columns {
                for y in 0..rows {
                    if let Some(i) = self.people()[[x, y]] {
                    	// Horizontal
                        if x > 0 {
                            if let Some(j) = self.people()[[x - 1, y]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x - 1 + y * columns)
                                        .unwrap()
                                }
                            }
                        }
                        // Vertical
                        if y > 0 {
                            if let Some(j) = self.people()[[x, y - 1]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x + (y - 1) * columns)
                                        .unwrap()
                                }
                            }
                        }
                        // Diagonal
                        if x > 0 && y > 0 {
                        	if let Some(j) = self.people()[[x - 1, y - 1]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x - 1 + (y - 1) * columns)
                                        .unwrap()
                                }
                            }
                        }
                        if x > 0 && y < rows - 1 {
                        	if let Some(j) = self.people()[[x - 1, y + 1]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x - 1 + (y + 1) * columns)
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
    		assert_eq!(Building::new(3, 4).shape(), [4, 3]);
    		
    	}

    	#[test]
    	fn is_empty() {
    		let array = array![[None, None], [None, None]];
    		let building = Building::try_from(array).expect("There is a sick one!");
    		assert!(building.is_empty());
    		assert!(Building::new(3, 4).is_empty());
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

    		let mut building = Building::new(1, 1);
    		assert!(!building.is_full());
    		building.try_push(Individual::Healthy).expect("can not push when it should!");
    		assert!(building.is_full());

    		let mut building = Building::new(2, 1);
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
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Inmune), Some(Individual::Sick)],
		    			[Some(Individual::Infected1), Some(Individual::Infected1)] 
		    		],
		    		spreding: Spreding::Everyone,
	    	};
    		initial.set_spreding(Spreding::Everyone);
    		initial.propagate();
    		assert_eq!(initial, expected);
    	}

    	#[test]
    	fn propagate_everyone4() {
    		let mut initial = Building::try_from(array![
    			[Individual::Inmune, Individual::Infected3],
    			[Individual::Healthy, Individual::Inmune] 
    		]).expect("There is a sick one!");
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Inmune), Some(Individual::Sick)],
		    			[Some(Individual::Infected1), Some(Individual::Inmune)] 
		    		],
		    		spreding: Spreding::Everyone,
		    	};
    		initial.set_spreding(Spreding::Everyone);
    		initial.propagate();
    		assert_eq!(initial, expected);
    	}

    	#[test]
    	fn propagate_everyone5() {
    		let mut initial = Building::try_from(array![
    			[Individual::Healthy, Individual::Inmune],
    			[Individual::Inmune, Individual::Infected3] 
    		]).expect("There is a sick one!");
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Infected1), Some(Individual::Inmune)],
		    			[Some(Individual::Inmune), Some(Individual::Sick)] 
		    		],
		    		spreding: Spreding::Everyone,
		    	};
    		initial.set_spreding(Spreding::Everyone);
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
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Inmune), Some(Individual::Sick)],
		    			[Some(Individual::Infected1), Some(Individual::Healthy)] 
		    		],
		    		spreding: Spreding::OneNear,
	    	};
    		initial.set_spreding(Spreding::OneNear);
    		initial.propagate();
    		assert_eq!(initial, expected);
    	}

    	#[test]
    	fn propagate_onenear4() {
    		let mut initial = Building::try_from(array![
    			[Individual::Inmune, Individual::Infected3],
    			[Individual::Healthy, Individual::Inmune] 
    		]).expect("There is a sick one!");
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Inmune), Some(Individual::Sick)],
		    			[Some(Individual::Infected1), Some(Individual::Inmune)] 
		    		],
		    		spreding: Spreding::OneNear,
		    	};
    		initial.set_spreding(Spreding::OneNear);
    		initial.propagate();
    		assert_eq!(initial, expected);
    	}

    	#[test]
    	fn propagate_onenear5() {
    		let mut initial = Building::try_from(array![
    			[Individual::Healthy, Individual::Inmune],
    			[Individual::Inmune, Individual::Infected3] 
    		]).expect("There is a sick one!");
    		let expected = Building{ 
    				people: array![
		    			[Some(Individual::Infected1), Some(Individual::Inmune)],
		    			[Some(Individual::Inmune), Some(Individual::Sick)] 
		    		],
		    		spreding: Spreding::OneNear,
		    	};
    		initial.set_spreding(Spreding::OneNear);
    		initial.propagate();
    		assert_eq!(initial, expected);
    	}
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum BuildingError {
        #[error("building is full")]
        Full,
        #[error("Sick individuals are not allowed in the buildings")]
        Sick,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
