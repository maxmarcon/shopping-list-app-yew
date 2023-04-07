mod components;

use components::itemlist::ItemList as ItemListComponent;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Item {
    name: String,
    checked: bool,
}

#[function_component]
fn App() -> Html {
    let items = use_state(|| {
        let mut items = vec![
            Item {
                name: "Cereals".into(),
                checked: false,
            },
            Item {
                name: "Milk".into(),
                checked: true,
            },
            Item {
                name: "Jam".into(),
                checked: true,
            },
            Item {
                name: "Butter".into(),
                checked: false,
            },
            Item {
                name: "Apples".into(),
                checked: false,
            },
        ];
        items.sort_by_key(|i| i.checked);
        items
    });

    let item_clicked = {
        let items = items.clone();
        Callback::from(move |name: String| {
            let mut updated_items = (*items).clone();
            updated_items
                .iter_mut()
                .filter(|i| i.name == name)
                .for_each(|i| i.checked = !i.checked);

            updated_items.sort_by_key(|i| i.checked);
            items.set(updated_items);
        })
    };

    html! {
        <div class="flex justify-center my-2">
            <ItemListComponent items={(*items).clone()} {item_clicked} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
