
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
    /// Returns the result of the simulation.
    pub fn run(self) -> Report {
        let mut counting_tables = Vec::new();
        for _ in 0..*self.report_plan.num_simulations() {
            let mut board = self.board.clone();
            board.advance_many(*self.report_plan.days());
            counting_tables.push(board.counting_table().clone());
        }
        Report { counting_tables }
    }
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
                    inmune: 20,
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
            (Individual::Inmune, vec![20])]);
        assert_eq!(report.counting_tables(), &vec![expected]);
    }
}