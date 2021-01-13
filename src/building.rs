use core::fmt::Display;
use core::convert::TryFrom;
use crate::errors::BuildingError;
use crate::Individual;
use gamma::graph::DefaultGraph;
use ndarray::Array2;
use serde::{Serialize, Deserialize};
use getset::{Getters, Setters, MutGetters};

/// Spreading mode inside a building.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Spreading {
    /// If there is one person infected in the building, then everyone is infected
    Everyone,
    /// Each infected person infects at least one healthy person, regardless of spatial structure
    One,
    /// Infected individuals try to infect someone near to them considering spatial structure
    ///
    /// Near individuals are does that are in distance one verticaly, horizontaly or diagonaly. 
    /// Also, as there can be more than one infected per building, they work collectively and infect
    /// as much people as possible, under the restriction that each of them infects only one other individual.
    OneNear,
    /// Infected individuals try to infect someone very near to them considering spatial structure.
    ///
    /// Very near individuals are does that are in distance one verticaly or horizontaly. 
    /// Also, as there can be more than one infected per building, they work collectively and infect
    /// as much people as possible, under the restriction that each of them infects only one other individual.
    OneVeryNear,
}

impl Default for Spreading {
    fn default() -> Self { 
        Spreading::OneVeryNear
    }
}


/// Builder struct for `Building`.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BuildingBuilder {
    people: Array2<Option<Individual>>,
    spreading: Spreading,
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

    /// Changes the spreading mode of the building
    pub fn with_spreading(mut self, new_spreading: Spreading) -> Self {
        self.spreading = new_spreading;
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
            spreading: self.spreading,
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
            spreading: Spreading::OneNear,
            name: String::from("Default"),
            penalty: 0,
            open: true,
        }
    }
}

/// Building in the board game where spreading can happen.
#[derive(Debug, Hash, Clone, PartialEq, Eq, Getters, MutGetters, Setters)]
pub struct Building {
    people: Array2<Option<Individual>>,
    spreading: Spreading,
    name: String,
    penalty: usize,
    open: bool,
}

impl Building {
	/// Creates a new and empty building
	///
	/// The default mode of propagation is `OneNear`, see `spreading` method for more.
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
    /// # use virus_alarm::Building;
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
	/// Returns the spreading mode of the building
	pub fn spreading(&self) -> &Spreading {
		&self.spreading
	}
	/// Sets the spreading mode of the building
	pub fn set_spreading(&mut self, new_spreading: Spreading) -> &mut Self {
		self.spreading = new_spreading;
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
    	match self.spreading {
    		Spreading::Everyone => self.propagate_everyone(),
    		Spreading::One => self.propagate_one(),
    		Spreading::OneNear => self.propagate_onenear(),
    		Spreading::OneVeryNear => self.propagate_oneverynear(),
    	}
    }

    /// Propagates by infecting one healthy individual per infected indiviual, if possible
    fn propagate_one(&mut self) -> &mut Self{
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
		self
    }

    /// Propagates by setting all healthy individuals to infected, if there is any infected in the building
    fn propagate_everyone(&mut self) -> &mut Self {
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
		self
    }

    /// Propagates by choosing a maximum matching between infected and healthy individuals
    fn propagate_onenear(&mut self) -> &mut Self {
        let graph: DefaultGraph = self.clone().into();
        let mut pairing = gamma::matching::Pairing::new();

        gamma::matching::maximum_matching(&graph, &mut pairing);

        self.propagate_from_pairing(pairing)
    }

    /// Propagates by choosing a maximum matching between infected and healthy individuals
    fn propagate_oneverynear(&mut self) -> &mut Self {
        let graph: DefaultGraph = self.clone().into();
        let mut pairing = gamma::matching::Pairing::new();

        gamma::matching::maximum_matching(&graph, &mut pairing);

        self.propagate_from_pairing(pairing)
    }

    fn propagate_from_pairing(&mut self, pairing: gamma::matching::Pairing) -> &mut Self {
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
        self
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
        match self.spreading() {
         	Spreading::OneNear | Spreading::OneVeryNear => {
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
		                    // Diagonals
		                    if self.spreading() == &Spreading::OneNear {
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
		        }
         	},
         	_ => todo!(),
         } 
		
        graph
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use test_case::test_case;
	use ndarray::array;

	#[test]
	fn construction() {
		let array = array![[Individual::Healthy, Individual::Infected1], [Individual::Healthy, Individual::Infected1]];
		let building = Building::try_from(array.clone()).expect("There is a sick one!");
		assert_eq!(building.people(), &array.map(|&i| Some(i)));
		assert_eq!(building.capacity(), 4);
		assert!(building.is_full());
		assert_eq!(building.spreading(), &Spreading::OneNear);
	}

	#[test]
	#[should_panic]
	fn no_sick_inside() {
		let array = array![[Individual::Sick]];
		Building::try_from(array).expect("There is a sick one!");
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

	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected1] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected2] 
		]; "double infection")]
	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected1] 
		]; "three options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Infected1] 
		]; "two options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]; "diagonal increasing")]
	#[test_case(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		], array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]; "diagonal decreasing")]
	fn propagate_everyone(initial: Array2<Individual>, expected: Array2<Individual>) {
		let mut initial = Building::unchecked_from(initial);
		let mut expected = Building::unchecked_from(expected);
		initial.set_spreading(Spreading::Everyone);
		expected.set_spreading(Spreading::Everyone);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected1] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected2] 
		]; "double infection")]
	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Healthy, Individual::Healthy] 
		]; "three options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Healthy] 
		]; "two options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]; "diagonal increasing")]
	#[test_case(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		], array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]; "diagonal decreasing")]
	fn propagate_one(initial: Array2<Individual>, expected: Array2<Individual>) {
		let mut initial = Building::unchecked_from(initial);
		let mut expected = Building::unchecked_from(expected);
		initial.set_spreading(Spreading::One);
		expected.set_spreading(Spreading::One);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Infected2] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Infected1, Individual::Infected3] 
		]; "double infection")]
	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Healthy, Individual::Healthy] 
		]; "three options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Healthy] 
		]; "two options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Infected1, Individual::Inmune] 
		]; "diagonal increasing")]
	#[test_case(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		], array![
			[Individual::Infected1, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]; "diagonal decreasing")]
	#[test_case(array![
			[Individual::Healthy, Individual::Healthy],
			[Individual::Healthy, Individual::Sick] 
		], array![
			[Individual::Healthy, Individual::Healthy],
			[Individual::Healthy, Individual::Sick] 
		] => panics "There should not have been a sick person in the building"; "tried to spread from sick")]
	fn propagate_onenear(initial: Array2<Individual>, expected: Array2<Individual>) {
		let mut initial = Building::unchecked_from(initial);
		let mut expected = Building::unchecked_from(expected);
		initial.set_spreading(Spreading::OneNear);
		expected.set_spreading(Spreading::OneNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}

	#[test_case(array![
			[Individual::Healthy, Individual::Infected2],
			[Individual::Healthy, Individual::Infected1] 
		], array![
			[Individual::Infected1, Individual::Infected3],
			[Individual::Infected1, Individual::Infected2] 
		]; "double infection")]
	#[test_case(array![
			[Individual::Healthy, Individual::Infected1],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Infected1, Individual::Infected2],
			[Individual::Healthy, Individual::Healthy] 
		]; "three options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Healthy] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Healthy, Individual::Infected1] 
		]; "two options")]
	#[test_case(array![
			[Individual::Inmune, Individual::Infected3],
			[Individual::Healthy, Individual::Inmune] 
		], array![
			[Individual::Inmune, Individual::Sick],
			[Individual::Healthy, Individual::Inmune] 
		]; "diagonal increasing")]
	#[test_case(array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Infected3] 
		], array![
			[Individual::Healthy, Individual::Inmune],
			[Individual::Inmune, Individual::Sick] 
		]; "diagonal decreasing")]
	fn propagate_oneverynear(initial: Array2<Individual>, expected: Array2<Individual>) {
		let mut initial = Building::unchecked_from(initial);
		let mut expected = Building::unchecked_from(expected);
		initial.set_spreading(Spreading::OneVeryNear);
		expected.set_spreading(Spreading::OneVeryNear);
		initial.propagate();
		assert_eq!(initial, expected);
	}
}