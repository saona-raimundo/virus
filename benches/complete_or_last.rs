use criterion::{criterion_group, criterion_main, Criterion}; //, BenchmarkId};
use virus_alarm::prelude::*;

fn set_up() -> Simulation {
	let board = Board::default();
	let report_plan = ReportPlan { num_simulations: 5, days: 10 };
	Simulation::new(board, report_plan)
}

fn run_complete(simulation: &Simulation) {
	simulation.run();
}

fn run_last(simulation: &Simulation) {
    simulation.run_last_day();
}


fn bench_running(c: &mut Criterion) {
	// Setup
	let simulation = set_up();
    let mut group = c.benchmark_group("Simulation run");
    group.sample_size(1_000);
    group.bench_with_input("Complete", &simulation, |b, sim| b.iter(|| {
    	run_complete(sim) 
    }));
    group.bench_with_input("Last", &simulation, |b, sim| b.iter(|| {
    	run_last(sim) 
    }));
    group.finish();
}

criterion_group!(benches, bench_running);
criterion_main!(benches);
