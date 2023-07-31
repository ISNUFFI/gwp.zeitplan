use yew::{html, function_component, Html};
use crate::component::header::Header;

#[function_component(Test)]
pub fn test() -> Html{
    html!(
        <>
            <Header/>
            <table style="">
                <tr>
                    <th></th>
                    <th class="employee_header">{"Пн"}</th>
                    <th class="employee_header">{"Ср"}</th>
                    <th class="employee_header">{"Чт"}</th>
                    <th class="employee_header">{"Пт"}</th>
                    <th class="employee_header">{"Вт"}</th>
                    <th class="employee_header">{"Вс"}</th>
                    <th class="employee_header">{"Сб"}</th>
                </tr>
                <tr>
                    <td></td>
                    {""}
                </tr>
            </table>
        </>
    )
}