use virus_alert::prelude::*;
use text_io::read;

fn main() {
	let mut vaccinated: usize = 100;
	while vaccinated > 98 {
		println!("Please insert the number of vaccinated individuals (at most 98).");
		vaccinated = read!(); 
	}

	let mut board = board(vaccinated);
	board.advance_many(10);

	println!("Vaccinated: {}", vaccinated);

	println!("COUNTING TABLE");
	for (i, vec) in board.counting_table() {
		println!("{}\n{:?}", i, vec);
	}
	
	println!("\nDIAGRAM");
	let totals = board.recording().diagram();
	println!("Infected\n{:?}", totals[0]);
	println!("Sick\n{:?}", totals[1]);
	println!("Healthy\n{:?}", totals[2]);
}

fn board(vaccinated: usize) -> Board {
	assert!(vaccinated <= 98);
	let mut v = vec![Individual::Healthy; 98 - vaccinated];
	v.append(&mut vec![Individual::Infected1; 2]);
	v.append(&mut vec![Individual::Inmune; vaccinated]);
	let population = Population::from(v);

	let default = Board::default();
	Board::new(population, default.buildings().clone())
}