#![recursion_limit = "2048"]

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::start_app::<Model>();
}

const DEBUG: bool = false;
const HIDDEN: bool = true;
const NUM_SIMULATIONS: usize = 100;

mod debugging;
mod displaying;
use crate::debugging::*;

// Crates
use virus_alarm::prelude::*;
use yew::prelude::*;

// Structs
use virus_alarm::building::Spreading;
use core::time::Duration;
use yew::services::timeout::TimeoutTask;
use yew::services::TimeoutService;

// Traits
use core::fmt::Debug;

#[derive(Debug)]
pub enum Msg {
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
    SpreadingMode(ChangeData),
    // Action
    LoadSimulate,
    ComputeSimulate,
    LoadSimulateMany,
    ComputeSimulateMany,
}

#[derive(Debug, PartialEq)]
enum Output {
    Simulation([Vec<usize>; 3]),
    SimulationMany([f32; 4]),
}

#[derive(Debug)]
pub struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    board: Board,
    job: Option<TimeoutTask>,
    output: Option<Output>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let board = Board::default().set_spreading(Spreading::OneVeryNear).to_owned();
        Self {
            link,
            board,
            job: None,
            output: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadSimulate => {
                let handle = TimeoutService::spawn(
                    Duration::from_nanos(1),
                    self.link.callback(|_| Msg::ComputeSimulate),
                );
                self.job = Some(handle);
                true
            }
            Msg::ComputeSimulate => {
                self.job = None;
                time("One simulation");
                let diagram = self
                    .board
                    .clone()
                    .advance_many(10)
                    .recording()
                    .counting_table()
                    .diagram();
                self.output = Some(Output::Simulation(diagram));
                time_end("One simulation");
                true
            }
            Msg::LoadSimulateMany => {
                let handle = TimeoutService::spawn(
                    Duration::from_nanos(1),
                    self.link.callback(|_| Msg::ComputeSimulateMany),
                );
                self.job = Some(handle);
                true
            }
            Msg::ComputeSimulateMany => {
                self.job = None;
                time("Many simulations");
                let report = Simulation::new(
                    self.board.clone(),
                    ReportPlan {
                        num_simulations: NUM_SIMULATIONS,
                        days: 10,
                    },
                )
                .run();
                // Summarizing
                let normalization = NUM_SIMULATIONS as f32;
                let healthy_average = report
                    .individual_last(&Individual::Healthy)
                    .iter()
                    .cloned()
                    .sum::<usize>() as f32
                    / normalization;
                let healthy_and_immune_average = healthy_average
                    + report
                        .individual_last(&Individual::Immune)
                        .iter()
                        .cloned()
                        .sum::<usize>() as f32
                        / normalization;
                let sick_average = report
                    .individual_last(&Individual::Sick)
                    .iter()
                    .cloned()
                    .sum::<usize>() as f32
                    / normalization;
                let contained_average = report
                    .counting_tables()
                    .iter()
                    .map(
                        |counting_table| {
                            if counting_table.is_contained() {
                                1
                            } else {
                                0
                            }
                        },
                    )
                    .sum::<usize>() as f32
                    / normalization;
                let immune = self.board.population().counting(Individual::Immune);
                // Updating
                self.output = Some(Output::SimulationMany([
                    healthy_and_immune_average,
                    sick_average,
                    healthy_average / (98 - immune) as f32,
                    contained_average,
                ]));
                time_end("Many simulations");
                true
            }
            Msg::Inmune(change_data) => {
                time("Change immune");
                if let yew::ChangeData::Value(s) = change_data {
                    let num = s
                        .parse::<usize>()
                        .expect("Could not parse vaccinated individuals.");
                    let mut current = self.board.population().counting(Individual::Immune);
                    while current < num {
                        self.board
                            .immunize()
                            .expect("Could not immunize a individual.");
                        current += 1;
                    }
                    while current > num {
                        self.board
                            .reverse_immunize()
                            .expect("Could not revese immunize a individual.");
                        current -= 1;
                    }
                }
                time_end("Change immune");
                false
            }
            Msg::ToggleConcertHall => {
                self.board.toggle("Concert Hall");
                false
            }
            Msg::ToggleBakery => {
                self.board.toggle("Bakery");
                false
            }
            Msg::ToggleSchool => {
                self.board.toggle("School");
                false
            }
            Msg::TogglePharmacy => {
                self.board.toggle("Pharmacy");
                false
            }
            Msg::ToggleRestaurant => {
                self.board.toggle("Restaurant");
                false
            }
            Msg::ToggleGym => {
                self.board.toggle("Gym");
                false
            }
            Msg::ToggleSupermarket => {
                self.board.toggle("Supermarket");
                false
            }
            Msg::ToggleShoppingCenter => {
                self.board.toggle("Shopping Center");
                false
            }
            Msg::SpreadingMode(change_data) => {
                if let ChangeData::Value(s) = change_data {
                    match s.as_str() {
                        "Everyone" => {
                            debug("Seting Spreading to Everyone");
                            self.board.set_spreading(Spreading::Everyone);
                        }
                        "One" => {
                            debug("Seting Spreading to One");
                            self.board.set_spreading(Spreading::One);
                        }
                        "OneNear" => {
                            debug("Seting Spreading to OneNear");
                            self.board.set_spreading(Spreading::OneNear);
                        }
                        "OneVeryNear" => {
                            debug("Seting Spreading to OneVeryNear");
                            self.board.set_spreading(Spreading::OneVeryNear);
                        }
                        _ => todo!(),
                    }
                }
                true
            }
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

    fn view(&self) -> Html {
        // debug(self);
        let has_job = self.job.is_some();
        html! {
            <>
            <p>
                { "Hi! Please set the configuration before simulating. / Bitte hier die Einstellungen festlegen." }
            </p>
            <form id="input_form" name="input_form">
                <fieldset>
                <legend>{ "Vaccinated individuals / Geimpfte" }</legend>
                    <div>
                        <input type="number" id="inmune" name="inmune" value=self.board.population().counting(Individual::Immune) min="0" max="98" size="2" onchange=self.link.callback(|i| Msg::Inmune(i))/>
                        <label for="inmune">
                            // <span class="visuallyhidden">{ "Vaccinated individuals / Geimpfte " }</span>
                            { " (0-98)" }
                        </label>
                    </div>
                </fieldset>
                <fieldset>
                <legend>{ "Open buildings / Offene Gebäude" }</legend>
                    <div>
                        <input type="checkbox" id="concert_hall" name="concert_hall" checked={ self.board.buildings()[0].is_open() } onclick=self.link.callback(|_| Msg::ToggleConcertHall)/>
                        <label for="concert_hall">{ " Concert hall / Konzerthaus (20)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="bakery" name="bakery" checked={ self.board.buildings()[1].is_open() } onclick=self.link.callback(|_| Msg::ToggleBakery)/>
                        <label for="bakery">{ " Bakery / Bäckerei (4)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="school" name="school" checked={ self.board.buildings()[2].is_open() } onclick=self.link.callback(|_| Msg::ToggleSchool)/>
                        <label for="school">{ " School / Schule (16)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="pharmacy" name="pharmacy" checked={ self.board.buildings()[3].is_open() } onclick=self.link.callback(|_| Msg::TogglePharmacy)/>
                        <label for="pharmacy">{ " Pharmacy / Apotheke (4)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="restaurant" name="restaurant" checked={ self.board.buildings()[4].is_open() } onclick=self.link.callback(|_| Msg::ToggleRestaurant)/>
                        <label for="restaurant">{ " Restaurant (12)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="gym" name="gym" checked={ self.board.buildings()[5].is_open() } onclick=self.link.callback(|_| Msg::ToggleGym)/>
                        <label for="gym">{ " Gym / Sporthalle (8)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="supermarket" name="supermarket" checked={ self.board.buildings()[6].is_open() } onclick=self.link.callback(|_| Msg::ToggleSupermarket)/>
                        <label for="supermarket">{ " Supermarket (4)" }</label>
                    </div>
                    <div>
                        <input type="checkbox" id="shopping_center" name="shopping_center" checked={ self.board.buildings()[7].is_open() } onclick=self.link.callback(|_| Msg::ToggleShoppingCenter)/>
                        <label for="shopping_center">{ " Shopping Center (8)" }</label>
                    </div>
                </fieldset>
                <fieldset hidden=HIDDEN>
                <legend>{ "Spreading mode" }</legend>
                    <input type="radio" id="everyone" name="everyone" value="Everyone" checked=self.board.spreading()==&Spreading::Everyone onchange=self.link.callback(|s| Msg::SpreadingMode(s))/>
                    <label for="everyone">
                        { " Everyone in the same building" }
                    </label><br/>
                    <input type="radio" id="one" name="one" value="One" checked=self.board.spreading()==&Spreading::One onchange=self.link.callback(|s| Msg::SpreadingMode(s))/>
                    <label for="one">
                        { " One other in the same building" }
                    </label><br/>
                    <input type="radio" id="one_near" name="one_near" value="OneNear" checked=self.board.spreading()==&Spreading::OneNear onchange=self.link.callback(|s| Msg::SpreadingMode(s))/>
                    <label for="one">
                        { " Vertical, horizontal and diagonal transmissions" }
                    </label><br/>
                    <input type="radio" id="one_very_near" name="one_very_near" value="OneVeryNear" checked=self.board.spreading()==&Spreading::OneVeryNear onchange=self.link.callback(|s| Msg::SpreadingMode(s))/>
                    <label for="one">
                        { " Vertical and horizontal transmissions" }
                    </label>
                </fieldset>
            </form>

            <div id="actions" name="actions">
                <button id="SimulateButton" name="SimulateButton" disabled=has_job onclick=self.link.callback(|_| Msg::LoadSimulate)>{ "Simulate!" }</button>
                <button id="SimulateManyButton" name="SimulateManyButton" disabled=has_job onclick=self.link.callback(|_| Msg::LoadSimulateMany)>{ format!("Simulate {}x!", NUM_SIMULATIONS) }</button>
            </div>

            <pre id="output" name="output">
                { self.output() }
            </pre>
            <footer id="footer" name="footnote">
                <p id="authorship" name="authorship">
                    { "Author: " }<a href="https://saona-raimundo.github.io/">{ "Raimundo Saona" }</a>
                </p>
            </footer>
            </>
        }
    }
}

impl Model {
    fn output(&self) -> Html {
        if self.job.is_some() {
            html! {"Computing!"}
        } else {
            match &self.output {
                Some(Output::Simulation(diagram)) => displaying::diagram(diagram),
                Some(Output::SimulationMany(report)) => displaying::report(report),
                None => {
                    html! {}
                }
            }
        }
    }
}
