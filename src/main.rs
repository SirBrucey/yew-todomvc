use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

mod state;

use state::{Entity, State};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "TodoMVC - Yew" }</h1>
            <AddEntry />
            <ListEntities />
        </div>
    }
}

#[function_component(AddEntry)]
fn add_entry() -> Html {
    let onkeypress = Dispatch::<State>::new().reduce_mut_callback_with(|s, e: KeyboardEvent| {
        if e.key() == "Enter" {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            input.set_value("");
            if !value.is_empty() {
                s.entities.push(Entity {
                    description: value.trim().to_string(),
                    ..Default::default()
                })
            }
        }
    });

    html! {
        <input type="text" placeholder="Add todo" {onkeypress} />
    }
}

#[function_component(ListEntities)]
fn list_entities() -> Html {
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
        <div>
            <h3>{"Entries"}</h3>
            <ul>
                { entries }
            </ul>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
