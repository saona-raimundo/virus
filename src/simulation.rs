use crate::recording::CountingTable;
use crate::prelude::{Board, BoardBuilder};
use getset::{Getters, Setters, MutGetters};
use serde::{Serialize, Deserialize};


pub mod report;

pub use report::*;

/// Builder for `Simulation`.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, Serialize, Deserialize, Default)]
pub struct SimulationBuilder {
    /// Board setup
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub board_builder: BoardBuilder,
    /// Report setup
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub report_plan: ReportPlan,
}

impl SimulationBuilder {
	pub fn build(self) -> Simulation {
		let board = self.board_builder.build();
		Simulation { board, report_plan: self.report_plan }
	}
}

/// Simulation of a game.
///
/// 
#[derive(Debug, Clone, PartialEq, Eq, Getters, Default)]
pub struct Simulation {
    /// Board setup
    #[getset(get = "pub")]
    board: Board,
    /// Report plan that determines the result announced after running the simulation.
    #[getset(get = "pub")]
    report_plan: ReportPlan,
}

impl Simulation {
    /// Constructor
    ///
    /// # Examples
    ///
    /// ```
    /// # use virus_alarm::prelude::*;
    /// let board = Board::default();
    /// let report_plan = ReportPlan { num_simulations: 10, days: 10 };
    /// Simulation::new(board, report_plan);
    /// ```
    pub fn new(board: Board, report_plan: ReportPlan) -> Self {
        Self { board, report_plan }
    }

    /// Returns the result of the simulation.
    pub fn run(&self) -> Report {
        let mut counting_tables = Vec::new();
        for _ in 0..*self.report_plan.num_simulations() {
            let mut board = self.board.clone();
            board.advance_many(*self.report_plan.days());
            counting_tables.push(board.counting_table().clone());
        }
        Report { counting_tables }
    }

    // /// Returns the result of the last day of the simulation, 
    // /// grouped by individual variant.
    // pub fn run_last_day(&self) -> HashMap<Individual, Vec<usize>> {
    //     let mut hm: HashMap<Individual, Vec<usize>> = 
    //         Individual::iter().map(|i| (i, Vec::new())).collect();

    //     for _ in 0..*self.report_plan.num_simulations() {
    //         let mut board = self.board.clone();
    //         for _ in 0..*self.report_plan.days() {
    //             board.advance_population();
    //         } 
    //         let hm_sim = board.population().counting_all();
    //         for individual in Individual::iter() {
    //             hm.entry(individual).or_insert(Vec::new()).push(hm_sim[&individual]);
    //         }
    //     }
    //     hm
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Individual, building::Spreading};

    #[test]
    fn run() {
        let simulation_builder = SimulationBuilder {
            board_builder: BoardBuilder{
                    healthy: 100,
                    infected1: 0,
                    infected2: 0,
                    infected3: 0,
                    sick: 3,
                    immune: 20,
                    buildings: vec![(0, 0)],
                    spreading: Spreading::OneNear,
            },
            report_plan: ReportPlan{
                    num_simulations: 1,
                    days: 0,
            }
        };
        let simulation = simulation_builder.build();
        let report = simulation.run();
        let expected = CountingTable::from(vec![
            (Individual::Healthy, vec![100]), 
            (Individual::Infected1, vec![0]), 
            (Individual::Infected2, vec![0]), 
            (Individual::Infected3, vec![0]), 
            (Individual::Sick, vec![3]), 
            (Individual::Immune, vec![20])]);
        assert_eq!(report.counting_tables(), &vec![expected]);
    }

    #[test]
    fn run2() {
        let simulation_builder = SimulationBuilder {
            board_builder: BoardBuilder{
                    healthy: 100,
                    infected1: 0,
                    infected2: 0,
                    infected3: 0,
                    sick: 3,
                    immune: 20,
                    buildings: vec![(2, 2)],
                    spreading: Spreading::OneNear,
            },
            report_plan: ReportPlan{
                    num_simulations: 1,
                    days: 1,
            }
        };
        let simulation = simulation_builder.build();
        let report = simulation.run();
        let expected = CountingTable::from(vec![
            (Individual::Healthy, vec![100, 100]), 
            (Individual::Infected1, vec![0, 0]), 
            (Individual::Infected2, vec![0, 0]), 
            (Individual::Infected3, vec![0, 0]), 
            (Individual::Sick, vec![3, 3]), 
            (Individual::Immune, vec![20, 20])]);
        assert_eq!(report.counting_tables(), &vec![expected]);
    }

    #[test]
    fn run3() {
        let simulation_builder = SimulationBuilder {
            board_builder: BoardBuilder{
                    healthy: 100,
                    infected1: 1,
                    infected2: 0,
                    infected3: 0,
                    sick: 3,
                    immune: 0,
                    buildings: vec![(200, 200)],
                    spreading: Spreading::OneNear,
            },
            report_plan: ReportPlan{
                    num_simulations: 1,
                    days: 1,
            }
        };
        let simulation = simulation_builder.build();
        let report = simulation.run();
        let expected = CountingTable::from(vec![
            (Individual::Healthy, vec![100, 99]), 
            (Individual::Infected1, vec![1, 1]), 
            (Individual::Infected2, vec![0, 1]), 
            (Individual::Infected3, vec![0, 0]), 
            (Individual::Sick, vec![3, 3]), 
            (Individual::Immune, vec![0, 0])]);
        assert_eq!(report.counting_tables(), &vec![expected]);
    }

    #[test]
    fn run_last_day() {
        let simulation_builder = SimulationBuilder {
            board_builder: BoardBuilder{
                    healthy: 100,
                    infected1: 1,
                    infected2: 0,
                    infected3: 0,
                    sick: 3,
                    immune: 0,
                    buildings: vec![(200, 200)],
                    spreading: Spreading::OneNear,
            },
            report_plan: ReportPlan{
                    num_simulations: 1,
                    days: 1,
            }
        };
        let report = simulation_builder.build().run();
        let result = vec![
            report.individual_last(&Individual::Healthy),
            report.individual_last(&Individual::Infected1),
            report.individual_last(&Individual::Infected2),
            report.individual_last(&Individual::Infected3),
            report.individual_last(&Individual::Sick),
            report.individual_last(&Individual::Immune),
        ];
        let expected = vec![
            vec![&99], 
            vec![&1], 
            vec![&1], 
            vec![&0], 
            vec![&3], 
            vec![&0],
            ];
        assert_eq!(result, expected);
    }
}