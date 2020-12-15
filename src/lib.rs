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
}

mod building {
    use crate::errors::BuildingError;
    use crate::Individual;
    use gamma::graph::DefaultGraph;
    use ndarray::Array2;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Building {
        people: Array2<Option<Individual>>,
    }

    impl Building {
        /// Return the shape of the array as a slice.
        pub fn shape(&self) -> &[usize] {
            self.people.shape()
        }
        /// Return the total capacity of the building, ie the number of individual it can host.
        pub fn capacity(&self) -> usize {
            self.people.shape().iter().product()
        }
        /// Checks if the building can not accept more people, ie is full.
        pub fn is_full(&self) -> bool {
            self.people.iter().all(|i| i.is_some())
        }
        /// Checks if the building is empty more people.
        pub fn is_empty(&self) -> bool {
            self.people.iter().all(|i| i.is_none())
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
                self.people.map(|_| -> Option<Individual> { None });
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
            let graph: DefaultGraph = self.clone().into();
            let mut pairing = gamma::matching::Pairing::new();

            gamma::matching::maximum_matching(&graph, &mut pairing);

            let rows = self.people.nrows();
            let columns = self.people.ncols();
            for x in 0..columns {
                for y in 0..rows {
                    if let Some(i) = self.people[[x, y]] {
                        self.people[[x, y]] = Some(match i {
                            Individual::Healthy => {
                                if pairing.has_node(x + y * columns) {
                                    Individual::Infected1
                                } else {
                                    i
                                }
                            }
                            Individual::Infected1 => Individual::Infected2,
                            Individual::Infected2 => Individual::Infected3,
                            Individual::Infected3 => Individual::Sick,
                            Individual::Sick | Individual::Inmune => i,
                        });
                    }
                }
            }
        }
    }

    impl From<Array2<Option<Individual>>> for Building {
        fn from(array: Array2<Option<Individual>>) -> Self {
            Building { people: array }
        }
    }

    impl From<Array2<Individual>> for Building {
        fn from(array: Array2<Individual>) -> Self {
            Building {
                people: array.map(|&p| Some(p)),
            }
        }
    }

    impl Into<DefaultGraph> for Building {
        fn into(self) -> DefaultGraph {
            let mut graph = DefaultGraph::new();
            let rows = self.people.nrows();
            let columns = self.people.ncols();
            // Add nodes
            for x in 0..columns {
                for y in 0..rows {
                    if self.people[[x, y]].is_some() {
                        graph.add_node(x + y * columns).unwrap()
                    }
                }
            }
            // Add edges
            for x in 0..columns {
                for y in 0..rows {
                    if let Some(i) = self.people[[x, y]] {
                        if x > 0 {
                            if let Some(j) = self.people[[x - 1, y]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x - 1 + y * columns)
                                        .unwrap()
                                }
                            }
                        }
                        if y > 0 {
                            if let Some(j) = self.people[[x, y - 1]] {
                                if i.interacts_with(&j) {
                                    graph
                                        .add_edge(x + y * columns, x + (y - 1) * columns)
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
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
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
