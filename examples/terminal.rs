use virus_alert::prelude::*;

fn main() {
	// let b = BoardBuilder::default();
	// let pretty = ron::ser::PrettyConfig::new()
 //        .with_depth_limit(2)
 //        .with_separate_tuple_members(true)
 //        .with_enumerate_arrays(true);
 //    let pretty_ser = ron::ser::to_string_pretty(&b, pretty).unwrap();
	// println!("{}", pretty_ser);
	// let b2: BoardBuilder = ron::de::from_str(&pretty_ser).unwrap();
	// println!("{:?}", b2);
	// assert_eq!(b2, b);

	let f = match std::fs::File::open(&"config.ron") {
		Ok(x) => x,
		Err(e) => {
			println!("Failed opening file, please locate it in the same directory as the executable file.\nFor more info: {}", e);
            std::process::exit(1);
		},
	};
    let b: BoardBuilder = match ron::de::from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", b);
}