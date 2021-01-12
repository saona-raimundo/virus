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
		let average = report.average_counting_table().map(|v| format!("{:.2} +- {:.2}", v.mean(), v.error()));
		// Write on a csv file
		let file = OpenOptions::new().append(true).create(true).open(format!("average_{}.csv", i))?;
		let mut writer = Writer::from_writer(file);
		for row in average.genrows() {
			writer.write_record(row)?;
		}		
		writer.flush()?;
	}
	Ok(())
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