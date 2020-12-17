use virus_alert::simulation::ReportPlan;
use virus_alert::building::Spreding;
use virus_alert::prelude::*;
// use ron::de::from_reader;

// const CONFIG_PATH: &str = "config.ron";

fn main() {
	// let simulation = initialize();
	// let report = simulation.run();


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

    let pretty = ron::ser::PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    println!("{}", ron::ser::to_string_pretty(&simulation_builder, pretty).unwrap());
}



// fn initialize() -> Board {
// 	let f = match std::fs::File::open(CONFIG_PATH) {
// 		Ok(x) => x,
// 		Err(e) => {
// 			println!("Failed opening file, please locate it in the same directory as the executable file.\nFor more info: {}", e);
//             std::process::exit(1);
// 		},
// 	};
//     let b: BoardBuilder = match from_reader(f) {
//         Ok(x) => x,
//         Err(e) => {
//             println!("Failed to load config: {}", e);
//             std::process::exit(1);
//         }
//     };
//     b.build()
// }