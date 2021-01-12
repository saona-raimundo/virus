use virus_alarm::prelude::*;
use ron::de::from_reader;
use preexplorer::prelude::*;

const CONFIG_PATH: &str = "config.ron";

fn main() {
	let simulations = initialize();

	let mut averages = Vec::new();
	let mut histograms = Vec::new();
	for i in 0..simulations.len() {
		let simulation = simulations[i].clone();
		let report = simulation.run();
		let healthy: Vec<Vec<f64>> = report.healthy_transpose().iter().map(|v| v.iter().map(|&x| x as f64).collect()).collect();
		// Mean +- Error
		let average = pre::SequenceError::new(healthy.clone())
        	.set_title(format!("config {}", i))
        	.to_owned();
        averages.push(average);
        // Histogram
        let histogram = pre::SequenceBin::new(healthy, 1)
        	.set_title(format!("config {}", i))
        	.to_owned();
        histograms.push(histogram);
	}

	pre::SequenceErrors::new(averages)
		.set_title("Evolution of healthy people under different configurations")
		.plot("averages")
		.unwrap();

	pre::SequenceBins::new(histograms)
		.set_title("Evolution of healthy people under different configurations")
		.plot("histograms")
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