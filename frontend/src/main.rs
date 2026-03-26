use yew::prelude::*;

mod theme;

#[function_component(App)]
fn app() -> Html {
    theme::apply_palette();

    html! {
        <>
            <h1>{"Hello World"}</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
