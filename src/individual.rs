/// Individual in the game, it represents a person.
#[derive(strum_macros::EnumIter, Hash, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
