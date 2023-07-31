use yew::prelude::*;
use yew_router::prelude::*;

mod component;
mod page;
mod model;
mod repository;

use crate::page::{widget::Widget, preview::Preview, test::Test};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/widget")]
    Widget,
    #[at("/preview")]
    Preview,
    #[at("/test")]
    Test,
}

fn switch(route: Route) -> Html{
    match route{
        Route::Widget => html!(<Widget/>),
        Route::Preview => html!(<Preview/>),
        Route::Test => html!(<Test/>)
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! (
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}