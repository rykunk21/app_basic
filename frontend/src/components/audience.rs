use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AudienceType {
    pub number: String,
    pub title: String,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct AudienceProps {
    #[prop_or_default]
    pub audience_types: Vec<AudienceType>,
    #[prop_or_default]
    pub section_title: Option<String>,
    #[prop_or_default]
    pub section_description: Option<String>,
}

#[function_component(Audience)]
pub fn audience(props: &AudienceProps) -> Html {
    let section_title = props
        .section_title
        .clone()
        .unwrap_or_else(|| "Who I Work With".to_string());

    let section_description = props.section_description.clone()
        .unwrap_or_else(|| "I specialize in building fast, mobile-first websites for local businesses. Whether you're getting online for the first time or need a fresh start, I'll create a site that works for your customers and your business.".to_string());

    let audience_types = if props.audience_types.is_empty() {
        vec![
            AudienceType {
                number: "1".to_string(),
                title: "Bars & Restaurants".to_string(),
                description: "Local hospitality businesses that need mobile-first sites, menus, and clear contact info.".to_string(),
            },
            AudienceType {
                number: "2".to_string(),
                title: "Contractors & Trades".to_string(),
                description: "Plumbers, electricians, builders, etc. who need simple sites to show services, credibility, and get leads.".to_string(),
            },
            AudienceType {
                number: "3".to_string(),
                title: "Service Businesses".to_string(),
                description: "Cleaning, landscaping, repair, consulting — businesses selling services rather than products.".to_string(),
            },
            AudienceType {
                number: "4".to_string(),
                title: "Local Shops / Small Businesses".to_string(),
                description: "Brick-and-mortar or small online sellers who need visibility, hours, and product highlights.".to_string(),
            },
        ]
    } else {
        props.audience_types.clone()
    };

    html! {

    <section style="background-color: #a8d8ff;">
        <div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
            /* Section Header */
            <div class="mx-auto max-w-lg text-center">
                <h2 class="text-2xl font-bold text-gray-900 sm:text-3xl">
                    { section_title }
                </h2>
                <p class="mt-4 text-pretty text-gray-700">
                    { section_description }
                </p>
            </div>

            /* Grid of Audience Types */
            <div class="mt-8 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4 md:gap-8">
                { for audience_types.iter().map(|audience_type| {
                    html! { <AudienceCard audience_type={audience_type.clone()} /> }
                })}
            </div>
        </div>
    </section>
        }
}

#[derive(Properties, PartialEq)]
pub struct AudienceCardProps {
    pub audience_type: AudienceType,
}

#[function_component(AudienceCard)]
fn audience_card(props: &AudienceCardProps) -> Html {
    let audience_type = &props.audience_type;

    html! {
        <div class="group relative grid place-content-center p-6 sm:p-8 bg-gray-50 rounded-lg hover:bg-[#a8d8ff] hover:shadow-lg transition-all duration-300">
            <div class="text-center">
                /* Number Badge */
                <div
                    class="inline-flex items-center justify-center w-12 h-12 mb-4 rounded-full text-white font-bold text-xl group-hover:scale-110 transition-transform"
                    style="background-color: #3ba2f6;"
                >
                    { &audience_type.number }
                </div>

                /* Title */
                <h3 class="text-lg font-bold text-gray-900 mb-2 group-hover:text-[#3ba2f6] transition-colors">
                    { &audience_type.title }
                </h3>

                /* Description */
                <p class="text-sm text-gray-600">
                    { &audience_type.description }
                </p>
            </div>
        </div>
    }
}
