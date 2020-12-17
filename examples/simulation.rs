use virus_alert::prelude::*;
use ron::de::from_reader;

const CONFIG_PATH: &str = "config.ron";

fn main() {
	let simulation = initialize();
	let report = simulation.run();
	println!("{:?}", report);
}

fn initialize() -> Simulation {
	let f = match std::fs::File::open(CONFIG_PATH) {
		Ok(x) => x,
		Err(e) => {
			println!("Failed opening file, please locate it in the same directory as the executable file.\nFor more info: {}", e);
            std::process::exit(1);
		},
	};
    let b: SimulationBuilder = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    b.build()
}