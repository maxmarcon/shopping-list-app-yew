use yew::prelude::*;

mod entry;

use crate::entry::Entry;

struct Item(&'static str, bool);

#[function_component]
fn App() -> Html {
    let entries: Vec<Html> = [
        Item("Cereals", false),
        Item("Milk", true),
        Item("Jam", true),
        Item("Butter", false),
        Item("Apples", false),
    ]
    .into_iter()
    .map(|Item(name, checked)| html! { <Entry name={name} checked={checked} />})
    .collect();

    html! {
        <ul>
            { entries }
        </ul>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
