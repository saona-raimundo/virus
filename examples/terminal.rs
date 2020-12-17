use virus_alert::prelude::*;

fn main() {
	let mut board = Board::default();
	board.advance_many(10);
	for (i, vec) in board.counting_table() {
		println!("{}\n{:?}", i, vec);
	}
	println!("\nDiagram");
	for vec in board.recording().diagram() {
		println!("{:?}", vec);
	}
	
}