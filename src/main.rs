mod components;

use components::itemlist::ItemList;
use serde::{Deserialize, Serialize};
use web_sys::{window, HtmlInputElement, Storage};
use yew::prelude::*;

const ITEMS_KEY: &str = "items";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    name: String,
    checked: bool,
}

fn sort_items(items: &mut Vec<Item>) {
    items.sort_by_key(|i| i.name.clone());
    items.sort_by_key(|i| i.checked);
}

fn add_item(
    items: UseStateHandle<Vec<Item>>,
    error_msg: UseStateHandle<Option<&str>>,
    input_ref: NodeRef,
) {
    let html_input = input_ref.cast::<HtmlInputElement>().unwrap();
    let input_text = html_input.value().trim().to_owned();
    if input_text.is_empty() {
        return;
    }
    if items
        .iter()
        .find(|i| i.name.to_lowercase() == input_text.to_lowercase())
        .is_some()
    {
        error_msg.set(Some("item already exists"));
        return;
    }
    error_msg.set(None);
    let mut updated_items = (*items).clone();
    updated_items.push(Item {
        name: input_text.into(),
        checked: false,
    });
    sort_items(&mut updated_items);
    save_items(&updated_items);
    items.set(updated_items);
    html_input.set_value("");
}

fn clear_checked(items: UseStateHandle<Vec<Item>>) {
    let updated_items = (*items)
        .clone()
        .into_iter()
        .filter(|i| !i.checked)
        .collect();

    save_items(&updated_items);
    items.set(updated_items);
}

fn save_items(items: &Vec<Item>) {
    if let Some(storage) = storage() {
        storage
            .set_item(ITEMS_KEY, &serde_json::to_string(&items).unwrap())
            .unwrap();
    }
}

fn load_items() -> Option<Vec<Item>> {
    if let Some(storage) = storage() {
        if let Some(json) = storage.get_item(ITEMS_KEY).unwrap() {
            return serde_json::from_str(&json).map_or_else(
                |e| {
                    log::error!("could not parse JSON: {:?}", e);
                    None
                },
                |items: Vec<Item>| Some(items),
            );
        }
        return None;
    }
    None
}

fn storage() -> Option<Storage> {
    match window() {
        Some(window) => window.local_storage().unwrap_or_else(|e| {
            log::error!("could not retrieve local storage: {:?}", e);
            None
        }),
        None => None,
    }
}

#[function_component]
fn App() -> Html {
    let error_msg = use_state_eq(|| None);
    let items = use_state_eq(|| load_items().unwrap_or(Vec::new()));
    let input_ref = use_node_ref();
    let can_submit = use_state_eq(|| false);

    let oninput = {
        let input_ref = input_ref.clone();
        let can_submit = can_submit.clone();
        Callback::from(move |_| {
            let input_ref = input_ref.cast::<HtmlInputElement>().unwrap();
            log::debug!("input = {}", input_ref.value());
            can_submit.set(!input_ref.value().trim().is_empty());
        })
    };

    let item_click = {
        let items = items.clone();
        Callback::from(move |name: String| {
            let mut updated_items = (*items).clone();
            updated_items
                .iter_mut()
                .filter(|i| i.name == name)
                .for_each(|i| i.checked = !i.checked);

            sort_items(&mut updated_items);
            save_items(&updated_items);
            items.set(updated_items);
        })
    };

    let item_delete = {
        let items = items.clone();
        Callback::from(move |name: String| {
            let mut updated_items = (*items)
                .clone()
                .into_iter()
                .filter(|i| i.name != name)
                .collect();

            sort_items(&mut updated_items);
            save_items(&updated_items);
            items.set(updated_items);
        })
    };

    let onsubmit = {
        let items = items.clone();
        let error_msg = error_msg.clone();
        let input_ref = input_ref.clone();
        let can_submit = can_submit.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            add_item(items.clone(), error_msg.clone(), input_ref.clone());
            can_submit.set(false);
        })
    };

    let clear_checked = {
        let items = items.clone();
        Callback::from(move |_| {
            clear_checked(items.clone());
        })
    };

    let any_checked = use_memo(|items| items.iter().any(|i| i.checked), items.clone());

    html! {
        <div class="flex flex-col p-2 gap-2 items-center">
            <div class="my-2 w-full md:w-1/2">
                <ItemList items={(*items).clone()} {item_click} {item_delete} />
            </div>
                <form {onsubmit}>
                    <div class="flex flex-wrap justify-center gap-1">
                            <div class="form-control">
                                <input type="text" class="input input-bordered" placeholder="Enter an item" {oninput} ref={input_ref} />
                                <label class="label">
                                    <span class="label-text-alt text-error">
                                    {*error_msg}
                                    </span>
                                </label>
                            </div>
                            <button type="submit" class="btn" disabled={!*can_submit}>{"Add"}</button>
                            <button type="button" class="btn" onclick={clear_checked} disabled={!*any_checked}>{"Delete checked"}</button>
                    </div>
                </form>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
