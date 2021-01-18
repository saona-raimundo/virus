use yew::prelude::*;

pub fn diagram(diagram: &[Vec<usize>; 3]) -> Html {
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
                <td scope="row">{ "healthy / gesund" }</td>
                { diagram[0].iter().map(|x| html!{<td>{ x }</td> }).collect::<Html>() }
            </tr>
            <tr>
                <td scope="row">{ "infected / infiziert" }</td>
                { diagram[1].iter().map(|x| html!{<td>{ x }</td> }).collect::<Html>() }
            </tr>
            <tr>
                <td scope="row">{ "sick / krank" }</td>
                { diagram[1].iter().map(|x| html!{<td>{ x }</td> }).collect::<Html>() }
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
                <td>{ format!("{:.2}",report[0]) }</td>
                <td>{ "healthy (incl. vaccinated) / gesund (inkl. Geimpfte)" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}",report[1]) }</td>
                <td>{ "sick / krank" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}%",report[2]) }</td>
                <td>{ "unvaccinated people still healthy / noch gesunde nicht-Geimpfte" }</td>
            </tr>
            <tr>
                <td>{ format!("{:.2}%",report[3]) }</td>
                <td>
                    { "contained outbreaks / eingedämmte Ausbrüche " }
                    <sup>
                        { "[1]" }
                        // <a href="#contained">{ "[1]" }</a>
                    </sup>
                </td>
            </tr>
        </table>
        <p id="contained">{
            "[1] An outbreak is contained if the virus can no \
            longer spread before infecting everyone.\n\
            Ein Ausbruch gilt als eingedämmt wenn das Virus \
            sich nicht mehr weiter ausbreiten kann vordem es \
            alle infiziert hat."
        }</p>
        </>
    }
}
