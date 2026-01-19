use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component(CTAButton)]
pub fn cta(props: &Props) -> Html {
    let onclick = props.onclick.clone().unwrap_or_default();
    html! {
        <button {onclick} class={classes!("px-5","py-2","rounded","bg-yellow-500","text-white", props.class.clone()) }>{ &props.label }</button>
    }
}
