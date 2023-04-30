use yew::prelude::*;

use super::icons::Trash;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<crate::Item>,
    pub item_click: Callback<String>,
    pub item_delete: Callback<String>,
}

#[function_component]
pub fn ItemList(
    Props {
        items,
        item_click,
        item_delete,
    }: &Props,
) -> Html {
    let list_items: Vec<Html> = items
        .iter()
        .map(|i| html! { <Item item={i.clone()} {item_click} {item_delete} /> })
        .collect();

    html! {
        if list_items.is_empty() {
        <p class="text-center italic font-medium">{"No items"}</p>  
        } else {
        <ul class="list-none flex flex-col gap-2">
        { list_items }
        </ul>
        } 
    }
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: crate::Item,
    item_click: Callback<String>,
    item_delete: Callback<String>,
}

#[function_component]
fn Item(
    ItemProps {
        item,
        item_click,
        item_delete,
    }: &ItemProps,
) -> Html {
    let onclick = {
        let item = item.clone();
        let item_click = item_click.clone();
        Callback::from(move |_| {
            item_click.emit(item.name.clone());
        })
    };

    let item_delete = {
        let item = item.clone();
        let item_delete = item_delete.clone();
        Callback::from(move |_| {
            item_delete.emit(item.name.clone());
        })
    };

    let base_item_classes = vec!["flex", "justify-between", "rounded-md", "p-2"];
    let item_color = if item.checked {
        vec!["bg-base-300", "text-base-content", "line-through"]
    } else {
        vec!["bg-info", "text-info-content"]
    };

    let button_color = if item.checked { None } else { Some("btn-info") };

    html! {
        <li key={item.name.clone()} class={classes!(base_item_classes, item_color)}>
            <div {onclick} class="flex w-full">
                <span class="text-lg">{ item.name.clone() }</span>
            </div>
            <button class={classes!("btn", "btn-xs", button_color)} onclick={item_delete}><Trash /></button>
        </li>
    }
}
