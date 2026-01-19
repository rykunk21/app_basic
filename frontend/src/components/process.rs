use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ProcessStep {
    pub number: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Properties, PartialEq)]
pub struct ProcessProps {
    #[prop_or_default]
    pub steps: Vec<ProcessStep>,
    #[prop_or_default]
    pub section_title: Option<String>,
}

#[function_component(Process)]
pub fn process(props: &ProcessProps) -> Html {
    let section_title = props
        .section_title
        .clone()
        .unwrap_or_else(|| "My Process".to_string());

    let steps = if props.steps.is_empty() {
        vec![
            ProcessStep {
                number: "1".to_string(),
                title: "Understand the Business".to_string(),
                description: "Before touching design or code, I focus on your goals. Who you are, who your customers are, and what you want them to do when they land on your site. This ensures the website actually supports your business—not just looks good.".to_string(),
                image_url: "https://images.unsplash.com/photo-1454165804606-c3d57bc86b40?auto=format&fit=crop&q=80&w=1160".to_string(),
            },
            ProcessStep {
                number: "2".to_string(),
                title: "Design for Clarity (Mobile First)".to_string(),
                description: "Most visitors will see your site on their phone. I design with mobile in mind first, then scale up to desktop—making sure your message is clear, readable, and visually consistent across devices.".to_string(),
                image_url: "https://images.unsplash.com/photo-1512941937669-90a1b58e7e9c?auto=format&fit=crop&q=80&w=1160".to_string(),
            },
            ProcessStep {
                number: "3".to_string(),
                title: "Build Fast, Clean, and Reliable".to_string(),
                description: "I translate designs into a fast, lightweight website using modern tools. The result is a site that loads quickly, feels responsive, and is easy to maintain or extend in the future.".to_string(),
                image_url: "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&q=80&w=1160".to_string(),
            },
            ProcessStep {
                number: "4".to_string(),
                title: "Review & Refine".to_string(),
                description: "You'll see progress early and often. We iterate together—adjusting layout, content, and details—until the site feels right and matches your brand.".to_string(),
                image_url: "https://images.unsplash.com/photo-1552664730-d307ca884978?auto=format&fit=crop&q=80&w=1160".to_string(),
            },
            ProcessStep {
                number: "5".to_string(),
                title: "Launch & Support".to_string(),
                description: "Once everything is ready, I handle the launch and make sure everything works smoothly. If you need updates, improvements, or new features later, the site is built to grow with you.".to_string(),
                image_url: "https://images.unsplash.com/photo-1519389950473-47ba0277781c?auto=format&fit=crop&q=80&w=1160".to_string(),
            },
        ]
    } else {
        props.steps.clone()
    };

    html! {
    <section class="py-8 px-4 sm:px-6 lg:px-8 bg-gray-50">
                <div class="max-w-6xl mx-auto">
                    <h2 class="text-2xl font-bold text-gray-900 text-center mb-8 md:text-3xl">
                        { section_title }
                    </h2>

                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                        { for steps.iter().map(|step| html! {
                            <ProcessCard step={step.clone()} />
                        }) }
                    </div>
                </div>
            </section>    }
}

#[derive(Properties, PartialEq)]
pub struct ProcessCardProps {
    pub step: ProcessStep,
}

#[function_component(ProcessCard)]
fn process_card(props: &ProcessCardProps) -> Html {
    let step = &props.step;

    html! {
        <a href="#" class="group relative block bg-black h-64 sm:h-80 lg:h-96 rounded overflow-hidden">
            <img
                alt={format!("Step {}: {}", step.number, step.title)}
                src={step.image_url.clone()}
                class="absolute inset-0 h-full w-full object-cover opacity-75 transition-opacity group-hover:opacity-50"
            />

            <div class="relative p-4 sm:p-6 lg:p-8 h-full flex flex-col">
                // Number and Title (always visible)
                <div>
                    <p class="text-sm font-medium tracking-widest text-pink-500 uppercase">
                        { format!("Step {}", step.number) }
                    </p>
                    <p class="text-xl font-bold text-white sm:text-2xl mt-2">
                        { &step.title }
                    </p>
                </div>

                // Description (revealed on hover)
                <div class="mt-auto">
                    <div class="translate-y-8 transform opacity-0 transition-all group-hover:translate-y-0 group-hover:opacity-100">
                        <p class="text-sm text-white">
                            { &step.description }
                        </p>
                    </div>
                </div>
            </div>
        </a>
    }
}
