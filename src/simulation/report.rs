use crate::simulation::CountingTable;
use ndarray::Array2;
use crate::prelude::{Individual};
use getset::{Getters, Setters, MutGetters};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

// mod last_day;

// pub use last_day::*;

/// Builder for `Report`.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, Serialize, Deserialize, Default)]
pub struct ReportPlan {
    /// Number of simulations
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub num_simulations: usize,
    /// Number of days the game advances
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub days: usize,
}

/// Report of a simulation of a game.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Default)]
pub struct Report {
    /// Counting tables.
    #[getset(get = "pub")]
    pub(crate) counting_tables: Vec<CountingTable>,
}

impl Report {
    /// Returns the average "counting table" over all simulations. 
    ///
    /// # Remarks
    ///
    /// It can not return a `CountingTable` since the averages are `f64`, so it returns the numerical table only.
    pub fn average_counting_table(&self) -> Array2<average::Variance> {
        let individual_variants_num = Individual::iter().len();
        if self.counting_tables.is_empty() {
            Array2::from_elem((individual_variants_num, 0), average::Variance::new())
        } else {
            let days = self.counting_tables()[0].days();
            let mut average_array = Array2::from_elem((individual_variants_num, days), average::Variance::new());
            let counting_tables: Vec<_> = self.counting_tables().iter().map(|counting_table| Array2::from(counting_table)).collect();
            for row in 0..individual_variants_num {
                for col in 0..days {
                    average_array[[row, col]] = counting_tables.iter().map(|counting_table| counting_table[[row, col]] as f64).collect();
                }
            }
            average_array
        }
    }
    /// Returns the trajectory over time of healthy individuals for all realizations. Each element of the vector is a realization, 
    /// which consists in a vector of values that represent the evolution of healthy individuals over time.
    ///
    /// # Remarks
    ///
    /// Realizations that do not have healthy individuals are omitted.
    pub fn healthy(&self) -> Vec<&Vec<usize>> {
        let mut healthy_vec = Vec::new();
        for counting_table in self.counting_tables() {
            if let Some(vec) = counting_table.inner().get(&Individual::Healthy) {
                healthy_vec.push(vec);
             } 
        }
        healthy_vec
    }

    /// Returns the trajectory over time of healthy individuals for all realizations. Each element of the vector is a day of the game, 
    /// which has a vector of values that represent each realization.
    ///
    /// # Remarks
    ///
    /// Realizations that do not have healthy individuals are omitted.
    pub fn healthy_transpose(&self) -> Vec<Vec<usize>> {
        let mut healthy_vec = Vec::new();
        let healthy_all = self.healthy();
        for day in 0..self.counting_tables()[0].days() {
            healthy_vec.push( healthy_all.iter().map(|realization| realization[day]).collect() );
        }
        healthy_vec
    }

    /// Returns the average of healthy people over all simulations. 
    pub fn average_healthy(&self) -> Vec<average::Variance> {
        let mut healthy_vec = Vec::new();
        let healthy_all = self.healthy();
        for day in 0..self.counting_tables()[0].days() {
            healthy_vec.push( healthy_all.iter().map(|realization| realization[day] as f64).collect() );
        }
        healthy_vec
    }

    /// Returns the trajectory over time of healthy individuals for all realizations.
    ///
    /// # Remarks
    ///
    /// Realizations that do not have healthy individuals are omitted.
    pub fn healthy_last(&self) -> Vec<&usize> {
        let mut healthy_vec = Vec::new();
        for healthy_realization in self.healthy() {
            healthy_vec.push(healthy_realization.last().expect("Empty vector!"));
        }
        healthy_vec
    }

    /// Returns the initial number of healthy individuals, which is the same for all realizations.
    ///
    /// # Panics
    ///
    /// If the number of simulations is zero.
    pub fn healthy_initial(&self) -> usize {
        if self.healthy().is_empty() {
            panic!("There is no simulation to compute the number of initial healthy individuals!")
        } else {
            self.healthy()[0][0]
        }
    }
}

impl From<Vec<CountingTable>> for Report {
    fn from(counting_tables: Vec<CountingTable>) -> Self { 
        Report { counting_tables } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn average_counting_table() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0])).collect(),
            Individual::iter().map(|i| (i, vec![1])).collect()
        ];
        let report = Report { counting_tables };
        let average_counting_table = report.average_counting_table();
        let variance: average::Variance = vec![0., 1.].into_iter().collect();
        assert_eq!(average_counting_table.map(|v| v.mean()), Array2::from_elem((6, 1), variance.mean()));
        assert_eq!(average_counting_table.map(|v| v.error()), Array2::from_elem((6, 1), variance.error()));
    }

    #[test]
    fn healthy() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0, 0])).collect(),
            Individual::iter().map(|i| (i, vec![1, 2])).collect()
        ];
        let report = Report { counting_tables };
        assert_eq!(report.healthy(), vec![&vec![0, 0], &vec![1, 2]]);
    }

    #[test]
    fn healthy_tranpose() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0, 0])).collect(),
            Individual::iter().map(|i| (i, vec![1, 2])).collect()
        ];
        let report = Report { counting_tables };
        assert_eq!(report.healthy_transpose(), vec![vec![0, 1], vec![0, 2]]);
    }

    #[test]
    fn average_healthy() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0, 0])).collect(),
            Individual::iter().map(|i| (i, vec![8, 9])).collect(),
            Individual::iter().map(|i| (i, vec![16, 0])).collect(),
        ];
        let report = Report { counting_tables };
        let average_healthy = report.average_healthy();
        assert_eq!(average_healthy.iter().map(|v| v.mean()).collect::<Vec<f64>>(), vec![8.0, 3.0]);
        assert_eq!(average_healthy.iter().map(|v| v.error()).collect::<Vec<f64>>(), vec![4.618802153517006, 3.0]);
    }

    #[test]
    fn healthy_last() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0, 0])).collect(),
            Individual::iter().map(|i| (i, vec![1, 2])).collect()
        ];
        let report = Report { counting_tables };
        assert_eq!(report.healthy_last(), vec![&0, &2]);
    }

    #[test]
    fn healthy_initial() {
        let counting_tables: Vec<CountingTable> = vec![
            Individual::iter().map(|i| (i, vec![0, 0])).collect(),
            Individual::iter().map(|i| (i, vec![1, 2])).collect()
        ];
        let report = Report { counting_tables };
        assert_eq!(report.healthy_initial(), 0);
    }
}