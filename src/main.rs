use yew::prelude::*;
use yewdux::prelude::*;

mod state;

use state::{Entity, State};

#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();

    let view_entry = |(index, entity): (usize, &Entity)| {
        html! {<li>{ &entity.description }</li>}
    };

    let entries = state
        .entities
        .iter()
        .enumerate()
        .map(view_entry)
        .collect::<Html>();

    html! {
        <ul>
            { entries }
        </ul>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
