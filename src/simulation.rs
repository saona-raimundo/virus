use std::collections::HashMap;
use crate::prelude::{Board, BoardBuilder, Individual};
use getset::{Getters, Setters, MutGetters};
use serde::{Serialize, Deserialize};

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
        let mut individuals_map: HashMap<Individual, Vec<usize>> = HashMap::new();
        for _ in 0..*self.report_plan.num_simulations() {
            let mut board = self.board.clone();
            board.advance_many(*self.report_plan.days());
            for i in &self.report_plan.query {
                // println!("Checking individual {:?}", i);
                let value: usize = *board.recording().counting_table()[i].last().expect("Simulation results where empty");
                let inner_vec = individuals_map.entry(*i).or_insert(vec![]);
                inner_vec.push(value);
            }
        }
        // println!("{:?}", individuals);
        let mut individuals: Vec<(Individual, Vec<usize>)> = individuals_map.drain().collect();
        individuals.sort_by(|(i, _), (j, _)| i.cmp(j));
        Report { individuals }
    }
}

/// Builder for `Report`.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, Serialize, Deserialize, Default)]
pub struct ReportPlan {
    /// Number of simulations
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub num_simulations: usize,
    /// Number of days the game advances
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub days: usize,
    /// Information that is included in the report of the simulation
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub query: Vec<Individual>,
}

/// Report of a simulation of a game.
///
/// 
#[derive(Debug, Clone, PartialEq, Eq, Getters, Default)]
pub struct Report {
    /// Board setup
    #[getset(get = "pub")]
    individuals: Vec<(Individual, Vec<usize>)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::building::Spreding;

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
                    spreding: Spreding::OneNear,
            },
            report_plan: ReportPlan{
                    num_simulations: 1,
                    days: 10,
                    query: vec![Individual::Healthy, Individual::Inmune, Individual::Sick],
            }
        };
        let simulation = simulation_builder.build();
        let report = simulation.run();
        assert_eq!(report.individuals(), &vec![(Individual::Healthy, vec![100]), (Individual::Sick, vec![3]), (Individual::Inmune, vec![20])]);
    }
}