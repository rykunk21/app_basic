use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {
        <div class={classes!("p-4","border","rounded","bg-white") }>
            { if let Some(title) = &props.title { html!{ <h3 class={classes!("font-medium") }>{ title }</h3> } } else { html!{} } }
            <div class={classes!("mt-2") }>{ for props.children.iter() }</div>
        </div>
    }
}
