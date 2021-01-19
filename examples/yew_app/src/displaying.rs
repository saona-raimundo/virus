use yew::prelude::*;
use crate::HIDDEN;

pub fn diagram(diagram: &[Vec<usize>; 3]) -> Html {
    let immune = 98 - diagram[0][0];
    html! {
        <>
        <table summary="Evolution of the population" id="simulation_one">
            <caption hidden=true>{ "Results" }</caption>
            <tr>
                <th scope="col">{ "Day / Tag" }</th>
                <th scope="col">{ "0" }</th>
                <th scope="col">{ "1" }</th>
                <th scope="col">{ "2" }</th>
                <th scope="col">{ "3" }</th>
                <th scope="col">{ "4" }</th>
                <th scope="col">{ "5" }</th>
                <th scope="col">{ "6" }</th>
                <th scope="col">{ "7" }</th>
                <th scope="col">{ "8" }</th>
                <th scope="col">{ "9" }</th>
                <th scope="col">{ "10" }</th>
            </tr>
            <tr>
                <td scope="row">{ "healthy incl. vaccinated / gesund inkl. Geimpfte" }</td>
                { diagram[0].iter().map(|x| html!{<td>{ x + immune }</td> }).collect::<Html>() }
            </tr>
            <tr>
                <td scope="row">{ "total infected / Infizierte gesamt" }</td>
                { diagram[1].iter().zip(diagram[2].iter()).map(|(infected, sick)| html!{<td>{ infected + sick }</td> }).collect::<Html>() }
            </tr>
            <tr>
                <td scope="row">{ "sick / krank" }</td>
                { diagram[2].iter().map(|x| html!{<td>{ x }</td> }).collect::<Html>() }
            </tr>
        </table>
        </>
    }
}

pub fn report(report: &[f32; 4]) -> Html {
    html! {
        <>
        <table summary="Average result at the final day" id="simulation_many">
            <caption hidden=true>{ "Results" }</caption>
            <tr>
                <th scope="colgroup" colspan="2">{ "Mean after 10 days / Mittelwert nach 10 Tagen" }</th>
            </tr>
            <tr>
                <td>{ format!("{:.2}", report[0]) }</td>
                <td>{ "healthy (incl. vaccinated) / gesund (inkl. Geimpfte)" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}", 98. - report[0]) }</td>
                <td>{ "total infected / Infizierte gesamt" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}", report[1]) }</td>
                <td>{ "sick / krank" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}%", 100. * report[2]) }</td>
                <td>{ "unvaccinated people still healthy / noch gesunde nicht-Geimpfte" }</td>
            </tr>
            <tr hidden=HIDDEN>
                <td>{ format!("{:.2}%", 100. * report[3]) }</td>
                <td>
                    { "contained outbreaks / eingedämmte Ausbrüche " }
                    <sup>
                        { "[1]" }
                    </sup>
                </td>
            </tr>
        </table>
        <p id="contained"  hidden=HIDDEN>{
            "[1] An outbreak is contained if the virus can no \
            longer spread before infecting everyone.\n\
            Ein Ausbruch gilt als eingedämmt wenn das Virus \
            sich nicht mehr weiter ausbreiten kann vordem es \
            alle infiziert hat."
        }</p>
        </>
    }
}
