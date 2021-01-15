mod utils;
#[macro_use]
mod debug;

const DEBUG: bool = false;
const SPREADING: Spreading = Spreading::OneVeryNear;

use virus_alarm::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Input {
    immune: usize,
    concert_hall: bool,
    bakery: bool,
    school: bool,
    pharmacy: bool,
    restaurant: bool,
    gym: bool,
    supermarket: bool,
    shopping_center: bool,
}

impl Input {
    fn buildings(&self) -> Vec<(usize, usize)> {
        let mut buildings = Vec::new();
        if self.concert_hall {
            buildings.push((5, 4))
        }
        if self.bakery {
            buildings.push((2, 2))
        }
        if self.school {
            buildings.push((4, 4))
        }
        if self.pharmacy {
            buildings.push((2, 2))
        }
        if self.restaurant {
            buildings.push((4, 3))
        }
        if self.gym {
            buildings.push((4, 2))
        }
        if self.supermarket {
            buildings.push((2, 2))
        }
        if self.shopping_center {
            buildings.push((4, 2))
        }
        debug!("Buildings created!\n{:?}", buildings);
        buildings
    }

    fn simulation(&self, num_simulations: usize) -> Simulation {
        SimulationBuilder {
            board_builder: BoardBuilder {
                healthy: 98 - self.immune,
                infected1: 2,
                infected2: 0,
                infected3: 0,
                sick: 0,
                immune: self.immune,
                buildings: self.buildings(),
                spreading: SPREADING,
            },
            report_plan: ReportPlan {
                num_simulations,
                days: 10,
            },
        }
        .build()
    }

    fn message(&mut self) -> String {
        let _timer = debug::Timer::new("One simulation");
        // Computing
        let _timer_run = debug::Timer::new("Running one simulation");
        let report = self.simulation(1).run();
        std::mem::drop(_timer_run);
        let diagram = report.counting_tables()[0].diagram();
        
        // Formating
        let mut out = String::new();
        out += "Day / Tag             0  1  2  3  4  5  6  7  8  9  10  \n";
        out += "---------------------+--+--+--+--+--+--+--+--+--+--+--\n";
        out += &format!("{:<22}", "healthy / gesund");
        for day in 0..=10 {
            out += &format!("{:<3}", diagram[0][day] + self.immune);
        }
        out += "\n";
        out += &format!("{:<22}", "infected / infiziert");
        for day in 0..=10 {
            out += &format!("{:<3}", diagram[1][day]);
        }
        out += "\n";
        out += &format!("{:<22}", "sick / krank");
        for day in 0..=10 {
            out += &format!("{:<3}", diagram[2][day]);
        }
        out += "\n";

        debug!("Simulated once and obtained\n{}", out);
        out
    }

    /// Optimized for minimmal memory usage
    fn message_many(&mut self, quantity: usize) -> String {
        // Computing
        let simulation = self.simulation(quantity);
        // Main computation
        let _timer_run = debug::Timer::new("Running many simulations");
        let report = simulation.run();
        let normalization = *simulation.report_plan().num_simulations() as f32;
        // Summarizing
        let healthy_average = report.individual_last(&Individual::Healthy).iter()
            .cloned()
            .sum::<usize>() as f32 / normalization;
        let healthy_and_immune_average = healthy_average +
            report.individual_last(&Individual::Immune).iter()
                .cloned()
                .sum::<usize>() as f32 / normalization;
        let sick_average = report.individual_last(&Individual::Sick).iter()
            .cloned()
            .sum::<usize>() as f32 / normalization;
        let contained_average = (0..*simulation.report_plan().num_simulations())
            .map(|sim_index| {
                let infected_sim = report.individual_last(&Individual::Infected1)[sim_index]
                    + report.individual_last(&Individual::Infected2)[sim_index]
                    + report.individual_last(&Individual::Infected3)[sim_index];
                let healthy_or_immune_sim = report.individual_last(&Individual::Healthy)[sim_index]
                    + report.individual_last(&Individual::Immune)[sim_index];
                (infected_sim == 0) && (healthy_or_immune_sim > 0)
            })
            .map(|b| if b { 1 } else { 0 })
            .sum::<usize>() as f32 / normalization;
        std::mem::drop(_timer_run);

        // Formating
        let mut out = String::new();
        out += "Mean after 10 days / Mittelwert nach 10 Tagen\n";
        out += "---------------------------------------------\n";
        out += &format!("{:<6.2}", healthy_and_immune_average);
        out += "healthy (incl. vaccinated) / gesund (inkl. Geimpfte)\n";
        out += &format!("{:<6.2}", sick_average);
        out += "sick / krank\n";
        out += &format!("{:<6}", format!("{:.0}%", 100. * healthy_average / (98 - self.immune) as f32));
        out += "unvaccinated people still healthy / noch gesunde nicht-Geimpfte\n";
        out += &format!("{:<6}", format!("{:.0}%", 100. * contained_average));
        out += "contained outbreaks / eingedämmte Ausbrüche\n\n\
                An outbreak is contained if the virus can no \
                longer spread before infecting everyone.\n\
                Ein Ausbruch gilt als eingedämmt wenn das Virus \
                sich nicht mehr weiter ausbreiten kann vordem es \
                alle infiziert hat.";
        debug!("Simulated {} times and obtained\n{}", quantity, out);
        out
    }
}

#[wasm_bindgen]
impl Input {
    pub fn new
    (
        immune_init: usize,
        concert_hall: bool,
        bakery: bool,
        school: bool,
        pharmacy: bool,
        restaurant: bool,
        gym: bool,
        supermarket: bool,
        shopping_center: bool,
    ) -> Self {
        utils::set_panic_hook();
        let input = Input {
            immune: immune_init,
            concert_hall,
            bakery,
            school,
            pharmacy,
            restaurant,
            gym,
            supermarket,
            shopping_center,
        };
        debug!("We read input\n{:?}", input);
        input
    }
    pub fn message_js(&mut self) -> String {
        self.message()
    }
    pub fn message_many_js(&mut self, quantity: usize) -> String {
        let _timer = debug::Timer::new("Many simulations");
        self.message_many(quantity)
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn reminder() {
        println!("RECALL to deactivate the DEBUG option!");
        println!("This will wipe out the wasm code from your Rust code.");
        assert_eq!(DEBUG, false);
    }

    #[test_case(Input::new(98, true, true, true, true, true, true, true, true), vec![(5, 4), (2, 2), (4, 4), (2, 2), (4, 3), (4, 2), (2, 2), (4, 2)]; "all buidlings")]
    #[test_case(Input::new(98, false, false, false, false, false, false, false, false), vec![]; "no buildings")]
    #[test_case(Input::new(98, true, false, false, false, false, false, false, true), vec![(5, 4), (4, 2)]; "first and last")]
    fn buildings(input: Input, expected: Vec<(usize, usize)>) {
        let buildings = input.buildings();
        assert_eq!(expected, buildings);
    }

    #[test]
    fn message_many() {
        let result = Input::new(98, true, true, true, true, true, true, true, true)
            .message_many(100);
        let expected = String::from("\
            Mean after 10 days / Mittelwert nach 10 Tagen\n\
            ---------------------------------------------\n\
            98.00 healthy (incl. vaccinated) / gesund (inkl. Geimpfte)\n\
            2.00  sick / krank\n\
            NaN%  unvaccinated people still healthy / noch gesunde nicht-Geimpfte\n\
            100%  contained outbreaks / eingedämmte Ausbrüche\n\n\

            An outbreak is contained if the virus can no longer spread before infecting everyone.\n\
            Ein Ausbruch gilt als eingedämmt wenn das Virus sich nicht mehr weiter ausbreiten kann vordem es alle infiziert hat.\
        ");
        assert_eq!(result, expected);
    }
}