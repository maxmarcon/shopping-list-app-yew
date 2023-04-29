use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<crate::Item>,
    pub item_clicked: Callback<String>,
}

#[function_component]
pub fn ItemList(
    Props {
        items,
        item_clicked,
    }: &Props,
) -> Html {
    let list_items: Vec<Html> = items
        .iter()
        .map(|i| html! { <Item item={i.clone()} item_clicked={item_clicked} /> })
        .collect();

    html! {
      <ul class="list-none w-full md:w-1/2 w-full flex flex-col gap-2">
        { list_items }
      </ul>
    }
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: crate::Item,
    item_clicked: Callback<String>,
}

#[function_component]
fn Item(ItemProps { item, item_clicked }: &ItemProps) -> Html {
    let onclick = {
        let item = item.clone();
        let item_clicked = item_clicked.clone();
        Callback::from(move |_| {
            item_clicked.emit(item.name.clone());
        })
    };

    let base_item_classes = vec!["flex", "w-full", "justify-between", "rounded-md", "p-2"];
    let item_color = if item.checked {
        vec!["bg-base-300", "text-base-content", "line-through"]
    } else {
        vec!["bg-info", "text-info-content"]
    };

    html! {
        <li key={item.name.clone()} {onclick} class={classes!(base_item_classes, item_color)}>
        <span class="text-lg">{ item.name.clone() }</span>
        <input type="checkbox" checked={item.checked} class="checkbox" />
        </li>
    }
}
