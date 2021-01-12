use virus_alarm::prelude::*;


fn main() {
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
                days: 10,
        }
    };

    let pretty = ron::ser::PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    println!("{}", ron::ser::to_string_pretty(&vec![simulation_builder], pretty).unwrap());
}