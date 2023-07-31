use yew::{html, function_component, Html, prelude::*};
use crate::model::employee::Employee;

#[function_component(Preview)]
pub fn preview() -> Html {
    let employees_state:UseStateHandle<Option<Vec<Employee>>> = use_state_eq(|| None);
    
    // employees_state.set(Some(vec![Employee{
    //     id: 1,
    //     first_name: Some("Zalupa".to_string()),
    //     last_name: Some("kek".to_string()),
    //     email: "asdasd".to_string(),
    //     bitrix_user_id: Some(64),
    //     call_gear_employees: None,
    //     is_hidden: false,
    // }]));

    let table = match (*employees_state).clone(){
        Some(content) => html!(
            <table style="width: 100%; table-layout: fixed;">
                <tr>
                    <th class="preview_header">{"Сотрудник"}</th>
                    <th class="preview_header">{"Роль"}</th>
                </tr>
                {content.iter().map(|emp: &Employee| 
                    html!(
                        <tr id="preview_row">
                            <td>{format!("{} {}", emp.first_name.clone().unwrap_or_default(), emp.last_name.clone().unwrap_or_default())}</td>
                            <td>{""}</td>
                        </tr>
                    )
                ).collect::<Html>()}
            </table>
        ),
        None => html!(
            <h1>{"Not data yet"}</h1>
        )
    };
    html!(
        <>
            {table}
        </>
    )
}