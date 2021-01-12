use csv::Writer;
use std::fs::OpenOptions;
use virus_alarm::prelude::*;
use ron::de::from_reader;

const CONFIG_PATH: &str = "config.ron";

fn main() -> anyhow::Result<()> {
	let simulations = initialize();


	for i in 0..simulations.len() {
		// Run each simulation
		let simulation = simulations[i].clone();
		let report = simulation.run();
		let approximation = approx_infection_probability(report);
		// Write on a csv file
		let file = OpenOptions::new().append(true).create(true).open(format!("infection_probability_{}.csv", i))?;
		let mut writer = Writer::from_writer(file);
		writer.serialize((approximation.mean(), "+-", approximation.error()))?;
		writer.flush()?;
	}
	Ok(())
}

fn approx_infection_probability(report: Report) -> average::Variance {
	let healthy_initial = report.healthy_initial() as f64;
	let healthy_last: Vec<f64> = report.healthy_last().iter().map(|&&n| n as f64).collect();
	healthy_last.iter()
		.map(|x| 1. - x / healthy_initial)
		.collect()
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