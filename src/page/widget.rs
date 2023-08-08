use gloo::console::console;
use web_sys::console;
use yew::{html, function_component, Html, prelude::*};
use crate::{component::header::Header, model::employee::Employee, repository::Repository};

#[function_component(Widget)]
pub fn widget() -> Html {
    let employees_state: UseStateHandle<Option<Vec<Employee>>> = use_state_eq(|| None);
    let table_headers = ["Сотрудник ", "Внутренний номер ", "Роль ", "Спрятан "];

    // employees_state.set(Some(vec![Employee{
    //     id: 1,
    //     first_name: Some("Zalupa".to_string()),
    //     last_name: Some("kek".to_string()),
    //     email: "asdasd".to_string(),
    //     bitrix_user_id: Some(64),
    //     call_gear_employees: None,
    //     is_hidden: false,
    // }]));
    let sort_params = use_state(|| (String::from("first_name"), false));
    let sort_params_clone = sort_params.clone();
    let sort_params_change = Callback::from(move |field_name: &str| {
        let is_reverse: bool;
        let field = field_name.clone();
        if field_name == sort_params.0 {
            is_reverse = !sort_params.1;
        }
        else {
            is_reverse = false;
        }
        sort_params.set((field.to_owned(), is_reverse));
        gloo::console::log!(field, is_reverse);
    });
    
    let is_first_load = use_state(|| true);
    let employees_state_clone = employees_state.clone();
    let total = use_state(|| 0);

    use_effect(
        move || {


            if *is_first_load {
                let total = total.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo::console::log!("request");
                    let fetched_users = Repository::get_employees(0, 0, false, "first_name").await;
                    total.set(fetched_users.total);
                });
                is_first_load.set(false);
            }

            if *total > 0 {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Repository::get_employees(*total, 0, false , &sort_params_clone.0).await;
                    employees_state_clone.set(Some(fetched_users.data));
                });
            }
        }
    );

    let table = match (*employees_state).clone(){
        Some(content) => html!(
            <table style="width: 40%;">
                <tr>
                    <th class="employee_header" onclick={
                        let sort_params_change = sort_params_change.clone();
                        Callback::from(move |_|{
                            sort_params_change.emit("first_name");
                        })
                    }>
                        {table_headers[0]}
                    </th>
                    
                    <th class="employee_header" onclick={
                        let sort_params_change = sort_params_change.clone();
                        Callback::from(move |_|{
                            sort_params_change.emit("last_name");
                        })
                    }>
                        {table_headers[1]}
                    </th>

                    <th class="employee_header" onclick={
                        let sort_params_change = sort_params_change.clone();
                        Callback::from(move |_|{
                            sort_params_change.emit("last_name");
                        })
                    }>
                        {table_headers[2]}
                    </th>

                    <th class="employee_header" onclick={
                        let sort_params_change = sort_params_change.clone();
                        Callback::from(move |_|{
                            sort_params_change.emit("last_name");
                        })
                    }>
                        {table_headers[3]}
                    </th>
                </tr>
                {content.iter().map(|emp: &Employee| 
                    html!(
                        <tr id="employee_row">
                            <td>{format!("{} {}", emp.first_name.clone().unwrap_or_default(), emp.last_name.clone().unwrap_or_default())}</td>
                            <td>{emp.call_gear_employee_id.unwrap_or_default()}</td>
                            <td></td>
                            <td>{emp.is_hidden}</td>
                        </tr>
                    )
                ).collect::<Html>()}
            </table>
        ),
        None => html!(
            <h1>{"No data yet"}</h1>
        )
    };
    html!{
        <>
            <Header/>
            <div style="display: flex; justify-content: center; overflow-x: hidden; overflow-y: scroll; height: calc(100vh - 55px)">
                {table}
            </div>
        </>
    }
}