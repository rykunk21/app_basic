use yew::prelude::*;
/// --- Hero component ---
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section style="background-color: var(--almond-cream-50); color: var(--mauve-bark-900);" class="relative overflow-x-hidden">
            <div style="background: linear-gradient(to bottom right, var(--icy-blue-50), white, white);" class="absolute inset-0 -z-10 h-[800px]"></div>

            <main class="max-w-7xl mx-auto px-6 pt-16 pb-24 lg:pt-24">
                <div class="relative grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div class="relative z-10 max-w-2xl">

                        <h1 style="color: var(--icy-blue-500);" class="font-header font-bold text-5xl lg:text-6xl font-bold leading-[1.1] mb-6 tracking-tight">
                            { "I design & build " }
                            <br />
                            <span style="color: var(--mauve-bark-900);">{ "human-centered software" }</span>
                        </h1>

                        <p class="text-lg text-gray-600 mb-8 leading-relaxed max-w-lg">
                            { "From systems thinking to clean interfaces, I help teams and individuals turn complex ideas into reliable, maintainable products." }
                        </p>

                        <div class="flex flex-col sm:flex-row gap-4 mb-10">
                            <a
                                href="#contact"
                                style="background-color: var(--icy-blue-500); color: var(--almond-cream-50);"
                                class="px-8 py-3.5 rounded-full font-medium hover:opacity-90 transition shadow-lg text-center"
                            >
                                { "Start a conversation" }
                            </a>

                            <a
                                href="#process"
                                style="background-color: white; color: var(--mauve-bark-900); border-color: var(--mauve-bark-200);"
                                class="px-8 py-3.5 rounded-full font-medium hover:opacity-90 transition text-center border"
                            >
                                { "View my process" }
                            </a>
                        </div>

                        <div class="
                          space-y-3
                          relative
                          rounded-xl
                          bg-white/70
                          backdrop-blur-md
                          p-4

                          sm:bg-transparent
                          sm:backdrop-blur-0
                          sm:p-0
                            ">
                            { check("Systems-first thinking") }
                            { check("Clear communication") }
                            { check("Long-term maintainability") }
                        </div>
                    </div>


                    <div class="
                            relative mx-auto w-full max-w-[500px]
                            h-[420px] sm:h-[300px] lg:h-[600px]
                            rounded-2xl overflow-hidden animate-float
                            flex items-center justify-center

                            z-0
                            -mt-32 sm:-mt-24 lg:mt-0
                        ">
                    /* Background smear */
                         <img
                             src="media/smear.png"
                             alt=""
                             aria-hidden="true"
                             class="absolute inset-0 w-full h-full object-cover opacity-80"
                         />

                         /* Foreground portfolio image */
                         <img
                             src="media/portfolio.png"
                             alt="Working session"
                             class="relative z-10 w-[400px] h-[500px] object-cover rounded-xl shadow-lg border-4 border-icy-blue-500 shadow-icy-blue-200"
                         />

                            /* Subtle highlight */
                            <div
                                class="absolute inset-0 pointer-events-none z-20"
                                style="background: linear-gradient(to top right, transparent, var(--almond-cream-50)/10, transparent);"
                            ></div>
                        </div>
                    </div>


                <div class="mt-20 rounded-3xl shadow-xl border border-gray-100 p-8 grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x divide-gray-100" style="background-color: white;">
                    { metric("10+ years", "Building software") }
                    { metric("Rust • Web • Systems", "Primary focus") }
                    { metric("Quality > Speed", "Core principle") }
                </div>
            </main>
        </section>
    }
}

// --- helpers ---

fn check(text: &str) -> Html {
    html! {
        <div class="flex items-center gap-3" style="color: var(--mauve-bark-900);">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--mauve-bark-500);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span class="text-sm font-medium">{ text }</span>
        </div>
    }
}

fn metric(value: &str, label: &str) -> Html {
    html! {
        <div class="px-0 md:px-8 py-6 md:py-0">
            <div class="text-3xl font-bold" style="color: var(--mauve-bark-900);">{ value }</div>
            <div class="text-sm mt-1" style="color: var(--mauve-bark-700);">{ label }</div>
        </div>
    }
}
