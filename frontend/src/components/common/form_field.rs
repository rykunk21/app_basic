use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
}

#[function_component(FormField)]
pub fn form_field(props: &Props) -> Html {
    html! {
        <label class={classes!("block") }>
            { if let Some(label) = &props.label { html!{ <div class={classes!("text-sm","font-medium") }>{ label }</div> } } else { html!{} } }
            <input type="text" class={classes!("w-full","border","p-2","rounded") } placeholder={props.placeholder.clone().unwrap_or_default()} value={props.value.clone().unwrap_or_default()} oninput={props.oninput.clone().unwrap_or_default()} />
        </label>
    }
}
