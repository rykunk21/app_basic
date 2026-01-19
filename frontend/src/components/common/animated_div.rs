use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(AnimatedDiv)]
pub fn animated_div(props: &Props) -> Html {
    html! {
        <div class={classes!("transition-all","duration-300", props.class.clone().unwrap_or_default()) }>
            { for props.children.iter() }
        </div>
    }
}
