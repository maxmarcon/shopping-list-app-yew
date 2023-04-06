use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: &'static str,
    pub checked: bool,
}

#[function_component(Entry)]
pub fn entry(Props { name, checked }: &Props) -> Html {
    html! {
        <div>
        <span>{ name }</span>
        if *checked {
            <div>{ "Ok" }</div>
        }
        </div>
    }
}
