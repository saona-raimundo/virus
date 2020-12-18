use virus_alert::prelude::*;
use ron::de::from_reader;
use preexplorer::prelude::*;

const CONFIG_PATH: &str = "config.ron";

fn main() {
	let simulations = initialize();

	let mut comparison = Vec::new();
	for i in 0..simulations.len() {
		let simulation = simulations[i].clone();
		let report = simulation.run();
		let healthy: Vec<Vec<f64>> = report.healthy_transpose().iter().map(|v| v.iter().map(|&x| x as f64).collect()).collect();
		let values = pre::SequenceError::new(healthy)
        	.set_title(format!("config {}", i))
        	.to_owned();
        comparison.push(values);
	}
	pre::SequenceErrors::new(comparison)
		.set_title("Evolution of healthy people under different configurations")
		.plot("ploting simulation")
		.unwrap();
		
}

fn initialize() -> Vec<Simulation> {
	let f = match std::fs::File::open(CONFIG_PATH) {
		Ok(x) => x,
		Err(e) => {
			println!("Failed opening file, please locate it in the same directory as the executable file.\nFor more info: {}", e);
            std::process::exit(1);
		},
	};
    
    let b: Vec<SimulationBuilder> = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    
    b.into_iter().map(|simulation| simulation.build()).collect()
}