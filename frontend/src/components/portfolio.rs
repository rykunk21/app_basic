use yew::prelude::*;

/// Portfolio section with grid of projects
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub projects: Vec<String>,
}

#[function_component(Portfolio)]
pub fn portfolio(props: &Props) -> Html {
    let projects = props.projects.clone();

    html! {
        <section class={classes!("portfolio", "py-12", "px-4") }>
            <div class={classes!("max-w-5xl", "mx-auto") }>
                <h2 class={classes!("text-2xl", "font-semibold") }>{ "Portfolio" }</h2>
                <div class={classes!("mt-6", "grid", "gap-6", "sm:grid-cols-2", "md:grid-cols-3") }>
                    { for projects.into_iter().map(|p| html! {
                        <div class={classes!("p-4","border","rounded") }>
                            <h3 class={classes!("font-medium") }>{ p }</h3>
                            <p class={classes!("text-sm","text-gray-600") }>{ "Short project blurb or screenshot goes here." }</p>
                        </div>
                    }) }
                </div>
            </div>
        </section>
    }
}
