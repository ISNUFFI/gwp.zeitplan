use yew::{html, function_component, Html};

const HEADER_HEIGHT: &str = "55px"; 

#[function_component(Header)]
pub fn header() -> Html{
    html! {
        <>
            <div id="header" style={format!("height: {HEADER_HEIGHT};")}>
                <div style="width: 20%; height: 100%">
                    <h2 style="color: white; width: auto; font-weight: 100; font-size: 20px">{"Сотрудники"}</h2>
                </div>
                    
            </div>
            <div style={format!("height: {HEADER_HEIGHT};")}></div>
        </>
    }
}