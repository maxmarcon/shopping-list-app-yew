use yew::prelude::*;

use crate::Item;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item
}

#[function_component(ListItem)]
pub fn entry(Props { item }: &Props) -> Html {
    html! {
        <li class="flex w-full justify-between bg-info text-info-content rounded-md p-2">
        <span class="text-lg">{ item.name }</span>
        if item.checked {
            <input type="checkbox" checked={item.checked} class="checkbox" />
        }
        </li>
    }
}
