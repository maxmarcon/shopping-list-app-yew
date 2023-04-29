mod components;

use components::itemlist::ItemList;
use web_sys::{HtmlInputElement, KeyboardEvent};
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
    let input = input_ref.cast::<HtmlInputElement>().unwrap();

    let new_item = input.value().trim().to_owned();
    let mut updated_items = (*items).clone();
    if !new_item.is_empty() {
        if items
            .iter()
            .find(|i| i.name.to_lowercase() == new_item.to_lowercase())
            .is_some()
        {
            error_msg.set(Some("item already exists"));
            return;
        }
        error_msg.set(None);
        updated_items.push(Item {
            name: new_item,
            checked: false,
        });
        sort_items(&mut updated_items);
        items.set(updated_items);
        input.set_value("");
    }
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
    let error_msg = use_state(|| None);
    let items = use_state(|| Vec::<Item>::new());
    let input_ref = use_node_ref();

    let onkeydown = {
        let items = items.clone();
        let error_msg = error_msg.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |k: KeyboardEvent| {
            if k.key_code() == 13 {
                add_item(items.clone(), error_msg.clone(), input_ref.clone());
            }
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

    let item_added = {
        let items = items.clone();
        let error_msg = error_msg.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            add_item(items.clone(), error_msg.clone(), input_ref.clone());
        })
    };

    let clear_checked = {
        let items = items.clone();
        Callback::from(move |_| {
            clear_checked(items.clone());
        })
    };

    let any_checked = use_memo(|items| items.iter().any(|i| i.checked), (*items).clone());

    html! {
        <div class="flex flex-col p-2 gap-2 items-center">
            <div class="my-2 w-full md:w-1/2">
                <ItemList items={(*items).clone()} {item_click} {item_delete} />
            </div>
            <div class="flex flex-wrap justify-center gap-1">
                <div class="form-control">
                    <input type="text" class="input input-bordered" {onkeydown} ref={input_ref} />
                    <label class="label">
                        <span class="label-text-alt text-error">
                        {*error_msg}
                        </span>
                    </label>
                </div>
                <button type="button" class="btn" onclick={item_added}>{"Add"}</button>
                <button type="button" class="btn" onclick={clear_checked} disabled={!*any_checked}>{"Delete checked"}</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
