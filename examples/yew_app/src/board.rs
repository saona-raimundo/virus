use virus_alarm::{Board, BuildingBuilder, Building, Population, Individual, building::Spreading};

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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MyBoardBuilder {
    pub inmune: usize,
    pub concert_hall: bool,
    pub bakery: bool,
    pub school: bool,
    pub pharmacy: bool,
    pub restaurant: bool,
    pub gym: bool,
    pub supermarket: bool,
    pub shopping_center: bool,
    pub spreading: Spreading,
}

impl MyBoardBuilder {
	pub fn build(self) -> Board {
		// Population
		let mut population_vec = vec![Individual::Healthy; 98 - self.inmune];
		population_vec.append(&mut vec![Individual::Infected1; 2]);
		population_vec.append(&mut vec![Individual::Infected2; 0]);
		population_vec.append(&mut vec![Individual::Infected3; 0]);
		population_vec.append(&mut vec![Individual::Sick; 0]);
		population_vec.append(&mut vec![Individual::Inmune; self.inmune]);
		let population = Population::from(population_vec);

        Board::new(population, self.buildings())
    }

    pub fn buildings(&self) -> Vec<Building> {
		let mut buildings = Vec::new();
        if self.concert_hall {
            buildings.push(BuildingBuilder::new("Defult")
                .with_size(5, 4)
                .with_spreading(self.spreading)
                .and_is_open()
                .build()
            )
        }
        // if self.bakery {
        //     buildings.push((2, 2))
        // }
        // if self.school {
        //     buildings.push((4, 4))
        // }
        // if self.pharmacy {
        //     buildings.push((2, 2))
        // }
        // if self.restaurant {
        //     buildings.push((4, 3))
        // }
        // if self.gym {
        //     buildings.push((4, 2))
        // }
        // if self.supermarket {
        //     buildings.push((2, 2))
        // }
        // if self.shopping_center {
        //     buildings.push((4, 2))
        // }

        buildings
    }
}