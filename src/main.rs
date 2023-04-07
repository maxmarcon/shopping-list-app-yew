use yew::prelude::*;

mod listitem;

use crate::listitem::ListItem;

#[derive(PartialEq)]
pub struct Item {
    name: &'static str,
    checked: bool
}

#[function_component]
fn App() -> Html {
   
    let entries: Vec<Html> = [
        Item {name:"Cereals", checked: false},
        Item {name:"Milk", checked: true},
        Item {name:"Jam", checked: true},
        Item {name:"Butter", checked: false},
        Item {name:"Apples", checked: false},
    ]
    .into_iter()
    .map(|i| html! { <ListItem item={i} />})
    .collect();

    html! {
        <div class="flex justify-center my-2">
            <ul class="list-none w-full md:w-1/2 w-full flex flex-col gap-2">
                { entries }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
