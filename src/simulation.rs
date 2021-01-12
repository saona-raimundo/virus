
use crate::recording::CountingTable;
use crate::prelude::{Board, BoardBuilder, Individual};
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
    pub fn run(&self) -> Report {
        let mut counting_tables = Vec::new();
        for _ in 0..*self.report_plan.num_simulations() {
            let mut board = self.board.clone();
            board.advance_many(*self.report_plan.days());
            counting_tables.push(board.counting_table().clone());
        }
        Report { counting_tables }
    }

    /// Returns the result of the last day of the simulation.
    pub fn run_last_day(&self) -> ReportLastDay {
        // Set-up
        let mut healthy: Vec<usize> = Vec::new(); 
        let mut sick: Vec<usize> = Vec::new(); 
        let mut contained: Vec<bool> = Vec::new(); 

        // Computation
        for _ in 0..*self.report_plan.num_simulations() {
            let mut healthy_sim = self.board().counting_table().inner()[&Individual::Healthy][0];
            let mut infected1_sim = self.board().counting_table().inner()[&Individual::Infected1][0];
            let mut infected2_sim = self.board().counting_table().inner()[&Individual::Infected2][0];
            let mut infected3_sim = self.board().counting_table().inner()[&Individual::Infected3][0];
            let mut sick_sim = self.board().counting_table().inner()[&Individual::Sick][0];
            let mut board = self.board.clone();
            for _ in 0..*self.report_plan.days() {
                board.visit();
                board.propagate();
                let newly_infected = board.go_back();
                healthy_sim -= newly_infected;
                sick_sim += infected3_sim;
                infected3_sim = infected2_sim;
                infected2_sim = infected1_sim;
                infected1_sim = newly_infected;
            }
            let infected_sim = infected1_sim + infected2_sim + infected3_sim;
            let contained_sim: bool = (healthy_sim > 0) && (infected_sim == 0);

            // Saving results
            healthy.push(healthy_sim);
            sick.push(sick_sim);
            contained.push(contained_sim);
        }
        ReportLastDay { healthy, sick, contained }
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

    #[test]
    fn run2() {
        let simulation_builder = SimulationBuilder {
            board_builder: BoardBuilder{
                    healthy: 100,
                    infected1: 0,
                    infected2: 0,
                    infected3: 0,
                    sick: 3,
                    inmune: 20,
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
            (Individual::Inmune, vec![20, 20])]);
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
                    inmune: 0,
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
            (Individual::Inmune, vec![0, 0])]);
        assert_eq!(report.counting_tables(), &vec![expected]);
    }
}