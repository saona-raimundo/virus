#![recursion_limit="1024"]

use std::collections::HashMap;
use yew::prelude::*;
use virus_alarm::prelude::*;
use virus_alarm::recording::CountingTable;

mod board;
use board::*;

#[derive(Debug)]
enum Msg {
    // Input
    Inmune(ChangeData),
    ToggleConcertHall,
    // Action
    Simulate,
    SimulateMany,
}

#[derive(Debug)]
enum Output {
    Simulation,
    SimulationMany,
}


#[derive(Debug)]
struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    // Input
    board_builder: MyBoardBuilder,
    // Output
    output: Option<Output>,
    counting_table: CountingTable,
    report_last_day: HashMap<Individual, Vec<usize>>,
}

impl Model {
    fn output(&self) -> String {
        match &self.output {
            Some(Output::Simulation) => {
                self.counting_table.to_string()
            },
            Some(Output::SimulationMany) => {
                format!("{:?}", self.report_last_day)
            },
            None => "".to_string(),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut board_builder = MyBoardBuilder::default();
        board_builder.concert_hall = true;
        Self {
            link,
            board_builder,
            output: None,
            counting_table: CountingTable::default(),
            report_last_day: HashMap::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Simulate => {
                self.counting_table = self.board_builder.clone()
                    .build()
                    .advance_many(10)
                    .recording()
                    .counting_table().clone();
                self.output = Some(Output::Simulation);
                true
            },
            Msg::SimulateMany => {
                self.report_last_day = 
                    Simulation::new(
                        self.board_builder.clone().build(),
                        ReportPlan { num_simulations: 1000, days: 10 }
                    )
                    .run_last_day();
                self.output = Some(Output::SimulationMany);
                true
            },
            Msg::Inmune(change_data) => {
                match change_data {
                    yew::ChangeData::Value(s) => {
                        let num = s.parse::<usize>().expect("Could not parse vaccinated individuals.");
                        self.board_builder.inmune = num;
                    },
                    _ => (),
                }
                false
            }
            Msg::ToggleConcertHall => {
                self.board_builder.concert_hall = !self.board_builder.concert_hall;
                false
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
        html! {
            <>
            { "Hi! Please set the configuration before simulating. / Bitte hier die Einstellungen festlegen." }
            <form id="input_form" name="input_form">
                <input type="number" id="inmune" name="inmune" value=self.board_builder.inmune min="0" max="98" size="2" onchange=self.link.callback(|i| Msg::Inmune(i))/>
                <label for="inmune">{ " Vaccinated individuals / Geimpfte (0-98)" }</label>
                <br/>
                <input type="checkbox" id="concert_hall" name="concert_hall" checked={ self.board_builder.concert_hall } onclick=self.link.callback(|_| Msg::ToggleConcertHall)/> //onchange=self.link.callback(|_| Msg::Simulate)/>
                <label for="concert_hall">{ "Concert hall / Konzerthaus (20)" }</label>
                <br/>
            </form>

            <div id="actions" name="actions">
                <button onclick=self.link.callback(|_| Msg::Simulate)>{ "Simulate!" }</button>
                <button onclick=self.link.callback(|_| Msg::SimulateMany)>{ "Simulate many!" }</button>
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
