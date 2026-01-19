use yew::prelude::*;

mod components;
mod theme;

use components::{
    about::About, audience::Audience, contact::Contact, header::Header, hero::Hero,
    process::Process, services::Services,
};

#[function_component(App)]
fn app() -> Html {
    theme::apply_palette();

    html! {
        <>
            <Header />
            <Hero />
            <Services />
            <Process />
            <Audience />
            <About />
            <Contact />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
