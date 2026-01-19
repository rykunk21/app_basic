use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub nav_items: Vec<NavItem>,
    #[prop_or_default]
    pub show_auth_buttons: bool,
    #[prop_or_default]
    pub logo_href: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub href: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let menu_open = use_state(|| false);

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    let logo_href = props.logo_href.clone().unwrap_or_else(|| "#".to_string());

    // Default nav items if none provided
    let nav_items = if props.nav_items.is_empty() {
        vec![
            NavItem {
                label: "About".to_string(),
                href: "#".to_string(),
            },
            NavItem {
                label: "Careers".to_string(),
                href: "#".to_string(),
            },
            NavItem {
                label: "History".to_string(),
                href: "#".to_string(),
            },
            NavItem {
                label: "Services".to_string(),
                href: "#".to_string(),
            },
            NavItem {
                label: "Projects".to_string(),
                href: "#".to_string(),
            },
            NavItem {
                label: "Blog".to_string(),
                href: "#".to_string(),
            },
        ]
    } else {
        props.nav_items.clone()
    };

    html! {
        <header class="bg-white">
            <div class="mx-auto flex h-16 max-w-7xl items-center gap-8 px-4 sm:px-6 lg:px-8">
                // Logo Image
                <a class="block" href={logo_href}>
                    <img
                        src="/media/logo.svg"
                        alt="Logo"
                        class="h-8 w-auto"
                    />
                </a>

                <div class="flex flex-1 items-center justify-end md:justify-between">
                    // Desktop Navigation
                    <nav aria-label="Global" class="hidden md:block">
                        <ul class="flex items-center gap-6 text-sm">
                            { for nav_items.iter().map(|item| {
                                html! {
                                    <li>
                                        <a
                                            class="text-gray-500 transition hover:text-gray-500/75"
                                            href={item.href.clone()}
                                        >
                                            { &item.label }
                                        </a>
                                    </li>
                                }
                            })}
                        </ul>
                    </nav>

                    <div class="flex items-center gap-4">
                        // Auth Buttons
                        if props.show_auth_buttons {
                            <div class="sm:flex sm:gap-4">
                                <a
                                    class="block rounded-md bg-teal-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-teal-700"
                                    href="#"
                                >
                                    {"Login"}
                                </a>

                                <a
                                    class="hidden rounded-md bg-gray-100 px-5 py-2.5 text-sm font-medium text-teal-600 transition hover:text-teal-600/75 sm:block"
                                    href="#"
                                >
                                    {"Register"}
                                </a>
                            </div>
                        }

                        // Mobile Menu Toggle
                        <button
                            class="block rounded-sm bg-gray-100 p-2.5 text-gray-600 transition hover:text-gray-600/75 md:hidden"
                            onclick={toggle_menu}
                        >
                            <span class="sr-only">{"Toggle menu"}</span>
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="size-5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            // Mobile Menu (conditional)
            if *menu_open {
                <div class="md:hidden border-t border-gray-200">
                    <nav class="px-4 py-4">
                        <ul class="space-y-2">
                            { for nav_items.iter().map(|item| {
                                html! {
                                    <li>
                                        <a
                                            class="block text-gray-500 transition hover:text-gray-500/75 py-2"
                                            href={item.href.clone()}
                                        >
                                            { &item.label }
                                        </a>
                                    </li>
                                }
                            })}
                        </ul>
                    </nav>
                </div>
            }
        </header>
    }
}
