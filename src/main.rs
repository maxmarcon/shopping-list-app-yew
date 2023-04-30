mod components;

use components::itemlist::ItemList;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
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
    if html_input.value().is_empty() {
        return;
    }
    if items
        .iter()
        .find(|i| i.name.to_lowercase() == html_input.value().to_lowercase())
        .is_some()
    {
        error_msg.set(Some("item already exists"));
        return;
    }
    error_msg.set(None);
    let mut updated_items = (*items).clone();
    updated_items.push(Item {
        name: html_input.value().into(),
        checked: false,
    });
    sort_items(&mut updated_items);
    items.set(updated_items);
    html_input.set_value("");
}

fn clear_checked(items: UseStateHandle<Vec<Item>>) {
    let updated_items = (*items)
        .clone()
        .into_iter()
        .filter(|i| !i.checked)
        .collect();

    items.set(updated_items);
}

#[function_component]
fn App() -> Html {
    let error_msg = use_state_eq(|| None);
    let items = use_state_eq(|| Vec::<Item>::new());
    let input_ref = use_node_ref();
    let input_text = use_state(|| None::<String>);

    let oninput = {
        let input_ref = input_ref.clone();
        let input_text = input_text.clone();
        Callback::from(move |_| {
            let input_ref = input_ref.cast::<HtmlInputElement>().unwrap();
            log::debug!("input = {}", input_ref.value());
            input_text.set(match input_ref.value() {
                value if value.is_empty() => None,
                value => Some(value),
            });
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
            items.set(updated_items);
        })
    };

    let onsubmit = {
        let items = items.clone();
        let error_msg = error_msg.clone();
        let input_ref = input_ref.clone();
        let input_text = input_text.clone();
        Callback::from(move |_| {
            add_item(items.clone(), error_msg.clone(), input_ref.clone());
            input_text.set(None);
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
                <form {onsubmit} action="#">
                    <div class="flex flex-wrap justify-center gap-1">
                            <div class="form-control">
                                <input type="text" class="input input-bordered" placeholder="Enter an item" {oninput} ref={input_ref} />
                                <label class="label">
                                    <span class="label-text-alt text-error">
                                    {*error_msg}
                                    </span>
                                </label>
                            </div>
                            <button type="submit" class="btn" disabled={input_text.is_none()}>{"Add"}</button>
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
