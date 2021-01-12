use std::collections::HashMap;
use crate::{Building, Individual, Population};
use getset::{Getters, MutGetters};
use strum::IntoEnumIterator;
use ndarray::Array2;

/// Represents the state of the game and have high level commands.
#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
pub struct Recording {
	/// Returns a "table" with the counting of individual types per day.
	///
	/// The quantity of each individual type present in the population is counted and 
	/// the vector of numbers represents the count for each of the days that have passed.
    #[getset(get = "pub", get_mut = "pub")]
    counting_table: CountingTable,
    /// Returns the current stage.  
    #[getset(get = "pub", get_mut = "pub")]
    timeline: usize, 
    /// Returns a table with the counting of penalty for each building per day.  
    #[getset(get = "pub", get_mut = "pub")]
    penalty: Vec<(Building, Vec<usize>)>,
    /// Returns the score obtained per day.  
    #[getset(get = "pub", get_mut = "pub")]
    daily_score: Vec<isize>,
}

impl Recording {

	/// Creates a new `Recording` with the population given for the initial state. 
	///
	/// Buildings are needed to keep track of penalties. 
	pub fn new(population: Population, buildings: Vec<Building>) -> Self { 
		let mut recording = Recording::default();
		for building in buildings {
			recording.penalty.push((building, vec![0]));
		}
		recording.counting_table = population.counting_all().iter().map(|(&i, &val)| (i, vec![val])).collect();

		recording
	}

	/// Increments the count of statges by one
	fn increment_time(&mut self) -> &mut Self {
		self.timeline += 1;
		self
	}

	/// Returns a "table" with the following information per day: Total healthy, total sick and total infected.  
	///
	/// The information provided in this table is the total number of 
	/// infected, sick and healthy individuals respectively for each day that has been recorded.
	pub fn diagram(&self) -> [Vec<usize>; 3] {
		self.counting_table().diagram()
	}

	/// Main functions that registers newly infected individuals
	///
	/// # Panics
	///
	/// If the number of newly infected is larger than the number of healthy individuals available
	pub fn register(&mut self, newly_infected: usize, _buildings: &Vec<Building>) -> &mut Self {
		self.register_counting_table(newly_infected);
		// self.register_penalty(buildings);
		// self.register_daily_score(buildings);
		self.increment_time();
		self
	}

	fn register_counting_table(&mut self, newly_infected: usize) {
	 	let last_values = self.last_counting_table();

		let counting_table = self.counting_table_mut();
	 	counting_table.inner_mut().entry(Individual::Healthy).and_modify(|v| v.push(last_values[0] - newly_infected));
	 	counting_table.inner_mut().entry(Individual::Infected1).and_modify(|v| v.push(newly_infected));
	 	counting_table.inner_mut().entry(Individual::Infected2).and_modify(|v| v.push(last_values[1]));
	 	counting_table.inner_mut().entry(Individual::Infected3).and_modify(|v| v.push(last_values[2]));
	 	counting_table.inner_mut().entry(Individual::Sick).and_modify(|v| v.push(last_values[3] + last_values[4]));
	 	counting_table.inner_mut().entry(Individual::Inmune).and_modify(|v| v.push(last_values[5]));
	}

	fn last_counting_table(&self) -> Vec<usize> {
	 	[
	 		self.counting_table().inner()[&Individual::Healthy].last(),
	 		self.counting_table().inner()[&Individual::Infected1].last(),
	 		self.counting_table().inner()[&Individual::Infected2].last(),
	 		self.counting_table().inner()[&Individual::Infected3].last(),
	 		self.counting_table().inner()[&Individual::Sick].last(),
	 		self.counting_table().inner()[&Individual::Inmune].last(),
	 	].iter().map(|x| x.unwrap().clone()).collect()
	}
}



impl Default for Recording {
	// add code here
	fn default() -> Self { 
		let counting_table = Individual::iter().map(|i| (i, vec![0])).collect();
		// let diagram = [vec![0], vec![0], vec![0]];
		let timeline = 0;
		let penalty = Vec::new();
		let daily_score = vec![0];

		Recording { counting_table, timeline, penalty, daily_score }
	}
}

/// Represents the state of the game and have high level commands.
///
/// # Examples
///
/// This is how it looks. 
/// ```
/// # use virus_alert::prelude::*;
/// # use virus_alert::recording::CountingTable;
/// let counting_table = CountingTable::from(vec![
/// 	(Individual::Healthy, vec![98, 97]),
/// 	(Individual::Infected1, vec![2, 1]),
/// 	(Individual::Infected2, vec![0, 2]),
/// 	(Individual::Infected3, vec![0, 0]),
/// 	(Individual::Sick, vec![0, 0]),
/// 	(Individual::Inmune, vec![0, 0]),
/// ]);
/// assert_eq!(counting_table.to_string(), String::from("\
/// 	Individual\\Day 0  1  \n\
/// 	Healthy        98 97 \n\
/// 	Infected1      2  1  \n\
/// 	Infected2      0  2  \n\
/// 	Infected3      0  0  \n\
/// 	Sick           0  0  \n\
/// 	Inmune         0  0  \n\
/// "));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Getters, MutGetters)]
pub struct CountingTable {
	/// Returns a "table" with the counting of individual types per day.
	///
	/// The quantity of each individual type present in the population is counted and 
	/// the vector of numbers represents the count for each of the days that have passed.
    #[getset(get = "pub", get_mut = "pub")]
    inner: HashMap<Individual, Vec<usize>>,
}

impl CountingTable {

	/// Returns the number of days counted.
	pub fn days(&self) -> usize {
		self.inner().get(&Individual::Healthy).expect("Tried to write an empty counting table!").len()
	}
	/// Writes the contents of the counting table on the writer.
	///
	/// # Remarks
	///
	/// Recall that a writer needs to be flushed to show in the output stream.
	pub fn write_on<W: std::io::Write>(&self, writer: W) -> csv::Result<csv::Writer<W>> {
		let mut writer = csv::Writer::from_writer(writer);
		let table: Vec<Vec<String>> = self.clone().into();
		for row in table {
			writer.serialize(row)?;
		}
	    Ok(writer)
	}

	/// Returns a "table" with the following information per day: Total healthy, total sick and total infected.  
	///
	/// The information provided in this table is the total number of 
	/// infected, sick and healthy individuals respectively for each day that has been recorded.
	pub fn diagram(&self) -> [Vec<usize>; 3] {
		let healthy = &self.inner()[&Individual::Healthy];
		let infected = self.inner()[&Individual::Infected1].iter()
			.zip(
			self.inner()[&Individual::Infected2].iter()
			).zip(
			self.inner()[&Individual::Infected3].iter()
			).map(|((inf1, inf2), inf3)| inf1 + inf2 + inf3)
			.collect();
		let sick = &self.inner()[&Individual::Sick];
		[healthy.to_vec(), infected, sick.to_vec()]
	}
}

impl Into<Vec<Vec<String>>> for CountingTable {
	fn into(self) -> Vec<Vec<String>> {
		let mut table = Vec::new();
		table.push({
			let mut row = vec!["Individual\\Day".to_string()];
			row.extend((0..self.days()).map(|day| day.to_string()));
			row
			});
		for i in Individual::iter() {
			table.push({
				let mut row = vec![i.to_string()];
				row.extend((0..self.days()).map(|day| self.inner()[&i][day].to_string()));
				row
				});
		}
		table
	}
}

impl core::fmt::Display for CountingTable {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
		let table: Vec<Vec<String>> = self.clone().into();
		let mut out = String::new();
		for row in table {
			out += &format!("{:<15}", row[0]);
			for index in 1..row.len() {
				out += &format!("{:<3}", row[index]);
			}
			out += "\n"
		}

		write!(f, "{}", out)
	}
}

impl<T> From<T> for CountingTable 
where
	T: IntoIterator<Item = (Individual, Vec<usize>)>,
{
	fn from(iter: T) -> Self {
		CountingTable{ inner: iter.into_iter().collect() }
	}
}

impl core::iter::FromIterator<(Individual, Vec<usize>)> for CountingTable {
	fn from_iter<T>(iter: T) -> Self 
	where 
		T: std::iter::IntoIterator<Item = (Individual, Vec<usize>)>, 
	{
		CountingTable{ inner: iter.into_iter().collect() }
	}
}


impl From<CountingTable> for Array2<usize> {
	fn from(counting_table: CountingTable) -> Array2<usize> {
		Array2::from(&counting_table)
	}
}

impl From<&CountingTable> for Array2<usize> {
	fn from(counting_table: &CountingTable) -> Array2<usize> {
		let mut array = Array2::from_elem((6, counting_table.days()), 0);
		let individual_variants: Vec<Individual> = Individual::iter().collect();
		for counter in 0..individual_variants.len() {
			for day in 0..counting_table.days() {
				array[[counter, day]] = counting_table.inner()[&individual_variants[counter]][day];
			}
		}
		array
	}
}

impl From<&CountingTable> for Vec<(String, Vec<usize>)> {
	fn from(counting_table: &CountingTable) -> Vec<(String, Vec<usize>)> {
		Individual::iter().map(|i| (i.to_string(), counting_table.inner()[&i].clone())).collect()
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use ndarray::array;

	#[test]
	fn register_counting_table1() {
		let population = Population::from(vec![Individual::Healthy, Individual::Infected1, Individual::Infected3]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);

		assert_eq!(recording.last_counting_table(), vec![1, 1, 0, 1, 0, 0]);
		recording.register_counting_table(1);
		assert_eq!(recording.last_counting_table(), vec![0, 1, 1, 0, 1, 0]);
	}

	#[test]
	#[should_panic]
	fn register_counting_table2() {
		let population = Population::from(vec![Individual::Healthy]);
		let buildings = Vec::new();
		let mut recording = Recording::new(population, buildings);

		assert_eq!(recording.last_counting_table(), vec![1, 0, 0, 0, 0, 0]);
		recording.register_counting_table(2);
	}

	#[test]
	fn write_on() -> csv::Result<()> {
		let writer = vec![];
		let counting_table: CountingTable = Individual::iter().map(|i| (i, vec![0])).collect();
		let writer = counting_table.write_on(writer)?;
		let data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
		assert_eq!(data, String::from("Individual\\Day,0\nHealthy,0\nInfected1,0\nInfected2,0\nInfected3,0\nSick,0\nInmune,0\n"));
		Ok(())
	}

	#[test]
	fn array2() {
		let counting_table: CountingTable = Individual::iter().map(|i| (i, vec![0])).collect();
		let expected = array![[0], [0], [0], [0], [0], [0]];
		assert_eq!(Array2::from(&counting_table), expected);
	}

	#[test]
	fn diagram() {
		let counting_table: CountingTable = Individual::iter().map(|i| (i, vec![1, 2])).collect();
		let expected = [vec![1, 2], vec![3, 6], vec![1, 2]];
		assert_eq!(counting_table.diagram(), expected);
	}

	#[test]
	fn display() {
		let counting_table: CountingTable = Individual::iter().map(|i| (i, vec![0])).collect();
		let expected = String::from("\
			Individual\\Day 0  \n\
			Healthy        0  \n\
			Infected1      0  \n\
			Infected2      0  \n\
			Infected3      0  \n\
			Sick           0  \n\
			Inmune         0  \n");
		println!("{}", counting_table);
		assert_eq!(format!("{}", counting_table), expected);
	}
}