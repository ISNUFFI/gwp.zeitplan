use yew::{html, function_component, Html, prelude::*};
use crate::{component::header::Header, model::employee::Employee, repository::Repository};

#[function_component(Widget)]
pub fn widget() -> Html {
    let employees_state: UseStateHandle<Option<Vec<Employee>>> = use_state_eq(|| None);

    employees_state.set(Some(vec![Employee{
        id: 1,
        first_name: Some("Zalupa".to_string()),
        last_name: Some("kek".to_string()),
        email: "asdasd".to_string(),
        bitrix_user_id: Some(64),
        call_gear_employees: None,
        is_hidden: false,
    }]));
    
    // {
    //     let employees_state = employees_state.clone();
    //     use_effect(
    //         || {
    //             wasm_bindgen_futures::spawn_local(async move {
    //                 let fetched_users = Repository::get_employees().await;
    //                 employees_state.set(Some(fetched_users));
    //             });
    //         }
    //     );
    // }

    let table = match (*employees_state).clone(){
        Some(content) => html!(
            <table style="width: 40%;">
                <tr>
                    <th class="employee_header">{"Сотрудник"}</th>
                    <th class="employee_header">{"Внутренний номер"}</th>
                    <th class="employee_header">{"Роль"}</th>
                    <th class="employee_header">{"Спрятан"}</th>
                </tr>
                {content.iter().map(|emp: &Employee| 
                    html!(
                        <tr id="employee_row">
                            <td>{format!("{} {}", emp.first_name.clone().unwrap_or_default(), emp.last_name.clone().unwrap_or_default())}</td>
                            <td>{emp.bitrix_user_id.unwrap_or_default()}</td>
                            <td></td>
                            <td>{emp.is_hidden}</td>
                        </tr>
                    )
                ).collect::<Html>()}
            </table>
        ),
        None => html!(
            <h1>{"Not data yet"}</h1>
        )
    };
    html!{
        <>
            <Header/>
            <div style="display: flex; justify-content: center">
                {table}
            </div>
        </>
    }
}