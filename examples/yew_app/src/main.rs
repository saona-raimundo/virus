use yew::prelude::*;
use virus_alarm::prelude::*;
use virus_alarm::recording::CountingTable;
use virus_alarm::simulation::report::ReportLastDay;

enum Msg {
    Simulate,
    SimulateMany,
}

#[derive(Debug)]
struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    // Input
    board: Board,
    // Output
    counting_table: CountingTable,
    report_last_day: ReportLastDay,
}



impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            board: Board::default(),
            counting_table: CountingTable::default(),
            report_last_day: ReportLastDay::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Simulate => {
                self.counting_table = self.board.clone()
                    .advance_many(10)
                    .recording()
                    .counting_table().clone();
                true
            },
            Msg::SimulateMany => {
                self.report_last_day = 
                    Simulation::new(
                        self.board.clone(),
                        ReportPlan { num_simulations: 100, days: 10 }
                    )
                    .run_last_day();
                true
            },
        }
    }

    /// # Reamrks
    /// Should only return "true" if new properties are different to
    /// previously received properties.
    ///
    /// This component has no properties so we will always return "false".
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Simulate)>{ "Simulate!" }</button>
                <button onclick=self.link.callback(|_| Msg::SimulateMany)>{ "Simulate many!" }</button>
                <p>{ &self.counting_table }</p>
                <p>{ format!("{:?}", self.report_last_day) }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
