#![recursion_limit="1024"]

const DEBUG: bool = true;

mod debugging;

use crate::debugging::*;

use core::fmt::Debug;
use yew::prelude::*;
use virus_alarm::prelude::*;
use virus_alarm::recording::CountingTable;

#[derive(Debug)]
enum Msg {
    // Input
    Inmune(ChangeData),
    ToggleConcertHall,
    ToggleBakery,
    ToggleSchool,
    TogglePharmacy,
    ToggleRestaurant,
    ToggleGym,
    ToggleSupermarket,
    ToggleShoppingCenter,
    // Action
    Simulate,
    SimulateMany,
    LoadSimulateMany,
}

#[derive(Debug, PartialEq)]
enum Output {
    Simulation(CountingTable),
    SimulationMany(Report),
    Loading,
}

#[derive(Debug)]
struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    board: Board,
    output: Option<Output>,
}

impl Model {
    fn output(&self) -> Html {
        match &self.output {
            Some(Output::Simulation(c)) => {
                html!{ c }
            },
            Some(Output::SimulationMany(r)) => {
                html!{ format!("{:?}", r) }
            },
            Some(Output::Loading) => {
                html!{ "Loading!" }
            }
            None => html!{ "" },
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            board: Board::default(),
            output: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Simulate => {
                time("One simulation");
                let counting_table = self.board.clone()
                    .advance_many(10)
                    .recording()
                    .counting_table().to_owned();
                self.output = Some(Output::Simulation(counting_table));
                time_end("One simulation");
                true
            },
            Msg::SimulateMany => {
                time("Many simulations");
                let report_last_day = 
                    Simulation::new(
                        self.board.clone(),
                        ReportPlan { num_simulations: 100, days: 10 }
                    )
                    .run();
                self.output = Some(Output::SimulationMany(report_last_day));
                time_end("Many simulations");
                false
            },
            Msg::LoadSimulateMany => {
                self.output = Some(Output::Loading);
                self.link.send_message(Msg::SimulateMany);
                false
            },
            Msg::Inmune(change_data) => {
                time("Change immune");
                if let yew::ChangeData::Value(s) = change_data {
                    let num = s.parse::<usize>().expect("Could not parse vaccinated individuals.");
                    let mut current = self.board.population().counting(Individual::Immune); 
                    while current < num {
                        self.board.immunize().expect("Could not immunize a individual.");
                        current += 1;
                    }
                    while current > num {
                        self.board.reverse_immunize().expect("Could not revese immunize a individual.");
                        current -= 1;
                    }
                }
                time_end("Change immune");
                false
            },
            Msg::ToggleConcertHall => {
                self.board.toggle("Concert Hall");
                false
            },
            Msg::ToggleBakery => {
                self.board.toggle("Bakery");
                false
            },
            Msg::ToggleSchool => {
                self.board.toggle("School");
                false
            },
            Msg::TogglePharmacy => {
                self.board.toggle("Pharmacy");
                false
            },
            Msg::ToggleRestaurant => {
                self.board.toggle("Restaurant");
                false
            },
            Msg::ToggleGym => {
                self.board.toggle("Gym");
                false
            },
            Msg::ToggleSupermarket => {
                self.board.toggle("Supermarket");
                false
            },
            Msg::ToggleShoppingCenter => {
                self.board.toggle("Shopping Center");
                false
            },
        }
    }

    /// # Reamrks
    ///
    /// Should only return "true" if new properties are different to
    /// previously received properties.
    ///
    /// This component has no properties so we will always return "false".
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        // self.update(Msg::Simulate);
    }

    fn destroy(&mut self) {
        if self.output == Some(Output::Loading) {
            self.link.send_message(Msg::SimulateMany);
        }
    }

    fn view(&self) -> Html {
        debug(self);
        html! {
            <>
            { "Hi! Please set the configuration before simulating. / Bitte hier die Einstellungen festlegen." }
            <form id="input_form" name="input_form">
                <input type="number" id="inmune" name="inmune" value=self.board.population().counting(Individual::Immune) min="0" max="98" size="2" onchange=self.link.callback(|i| Msg::Inmune(i))/>
                <label for="inmune">{ " Vaccinated individuals / Geimpfte (0-98)" }</label>
                <br/>
                { "Open buildings / Offene Gebäude:" }
                <br/>
                <input type="checkbox" id="concert_hall" name="concert_hall" checked={ self.board.buildings()[0].is_open() } onclick=self.link.callback(|_| Msg::ToggleConcertHall)/>
                <label for="concert_hall">{ " Concert hall / Konzerthaus (20)" }</label>
                <br/>
                <input type="checkbox" id="bakery" name="bakery" checked={ self.board.buildings()[1].is_open() } onclick=self.link.callback(|_| Msg::ToggleBakery)/>
                <label for="bakery">{ " Bakery / Bäckerei (4)" }</label>
                <br/>
                <input type="checkbox" id="school" name="school" checked={ self.board.buildings()[2].is_open() } onclick=self.link.callback(|_| Msg::ToggleSchool)/>
                <label for="school">{ " School / Schule (16)" }</label>
                <br/>
                <input type="checkbox" id="pharmacy" name="pharmacy" checked={ self.board.buildings()[3].is_open() } onclick=self.link.callback(|_| Msg::TogglePharmacy)/>
                <label for="pharmacy">{ " Pharmacy / Apotheke (4)" }</label>
                <br/>
                <input type="checkbox" id="restaurant" name="restaurant" checked={ self.board.buildings()[4].is_open() } onclick=self.link.callback(|_| Msg::ToggleRestaurant)/>
                <label for="restaurant">{ " Restaurant (12)" }</label>
                <br/>
                <input type="checkbox" id="gym" name="gym" checked={ self.board.buildings()[5].is_open() } onclick=self.link.callback(|_| Msg::ToggleGym)/>
                <label for="gym">{ " Gym / Sporthalle (8)" }</label>
                <br/>
                <input type="checkbox" id="supermarket" name="supermarket" checked={ self.board.buildings()[6].is_open() } onclick=self.link.callback(|_| Msg::ToggleSupermarket)/>
                <label for="supermarket">{ " Supermarket (4)" }</label>
                <br/>
                <input type="checkbox" id="shopping_center" name="shopping_center" checked={ self.board.buildings()[7].is_open() } onclick=self.link.callback(|_| Msg::ToggleShoppingCenter)/>
                <label for="shopping_center">{ " Shopping Center (8)" }</label>
                <br/>
            </form>

            <div id="actions" name="actions">
                <button onclick=self.link.callback(|_| Msg::Simulate)>{ "Simulate!" }</button>
                <button onclick=self.link.callback(|_| Msg::LoadSimulateMany)>{ "Simulate many!" }</button>
            </div>

            <noscript>
                { "This page contains webassembly and javascript content, please enable javascript in your browser." }
            </noscript>

            <pre id="output" name="output">
                { self.output() }
            </pre>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
