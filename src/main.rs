use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

mod state;

use state::{Entity, State};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="flex flex-col justify-center bg-slate-700 min-h-screen w-screen">
            <header class="flex grow-0 justify-center">
                <h1>{ "TodoMVC - Yew" }</h1>
            </header>
            <div class="flex grow-0 justify-center">
                <AddEntry />
            </div>
            <main class="flex flex-col items-center content-start justify-evenly grow">
                <ListEntities />
            </main>
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
    let focus_ref = use_node_ref();
    let (state, dispatch) = use_store::<State>();

    let view_entry = |(index, entity): (usize, &Entity)| {
        let view = {
            let ondblclick = dispatch.reduce_mut_callback(move |s| {
                s.edited_value = s.entities[index].description.clone();
                s.toggle_edit(index);
            });

            html! {
                <label { ondblclick }>{ &entity.description }</label>
            }
        };

        let edit_field = {
            let edit = move |input: HtmlInputElement, state: &mut State| {
                let value = input.value();
                input.set_value("");

                state.update_description(index, value);
                state.edited_value = "".to_string();
            };

            let onkeypress = dispatch.reduce_mut_callback_with(move |s, e: KeyboardEvent| {
                if e.key() == "Enter" {
                    edit(e.target_unchecked_into(), s);
                }
            });

            let onblur = dispatch.reduce_mut_callback_with(move |s, e: FocusEvent| {
                edit(e.target_unchecked_into(), s);
            });

            let onmouseover = {
                let focus_ref = focus_ref.clone();
                Callback::from(move |_| {
                    if let Some(input) = focus_ref.cast::<HtmlInputElement>() {
                        input.focus().unwrap();
                    }
                })
            };

            if entity.editing {
                html! {
                    <input
                        type="text"
                        ref={ focus_ref.clone() }
                        value={ state.edited_value.clone() }
                        { onkeypress }
                        { onmouseover }
                        { onblur }
                    />
                }
            } else {
                view
            }
        };

        html! {
            <li>
                { edit_field }
            </li>
        }
    };

    let entries = state
        .entities
        .iter()
        .enumerate()
        .map(view_entry)
        .collect::<Html>();

    html! {
        <>
            <div class="flex grow-0">
                <h3>{"Entries"}</h3>
            </div>
            <div class="flex grow">
                <ul>
                    { entries }
                </ul>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
