use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="min-h-screen bg-white text-gray-900 py-16 px-6 flex flex-col md:flex-row items-center justify-between gap-8">
            // Left Side: Image
            <div class="w-full md:w-5/12 flex justify-center h-full md:justify-end">
                <img
                    src="media/headshot.jpg"
                    alt="About Me"
                    class="w-72 h-96 md:w-80 lg:w-96 object-cover rounded-lg shadow-lg"
                />
            </div>

            // Right Side: Text Content
            <div class="w-full md:w-7/12 text-center md:text-left relative">
                // Vertical Text
                <div class="absolute left-[40%] -top-6 md:-left-16 lg:top-0 md:top-6 rotate-0 md:rotate-[-90deg] text-sm tracking-widest text-gray-700">
                    <div class="flex items-center justify-center gap-2">
                        <div class="w-16 h-[2px] bg-gray-900"></div>
                        <p>{ "MORE ABOUT" }</p>
                    </div>
                </div>

                // Main Heading
                <h2 class="text-3xl md:text-5xl font-bold leading-tight mb-4 pl-10 text-gray-900">
                    { "A Passionate " }
                    <br />
                    { "Software Developer" }
                </h2>

                // Description
                <p class="text-gray-700 mb-6 text-sm md:text-base leading-relaxed max-w-2xl mx-auto md:mx-0">

    { "I’m Ryan Kunkel, a software developer who enjoys building thoughtful, reliable applications. I like working with Rust, Linux, and Docker, and I focus on creating user-friendly tools that solve real problems in a maintainable way." }
                </p>

                // Buttons
                <div class="flex flex-col sm:flex-row gap-4 justify-center md:justify-start">
                    <a
                        href="#projects"
                        class="bg-tertiary text-white font-semibold py-2 px-4 rounded-lg hover:bg-tertiary/80 text-center"
                    >
                        { "See Projects" }
                    </a>

                    <a
                        href="#details"
                        class="border border-tertiary text-tertiary font-semibold py-2 px-4 rounded-lg hover:bg-tertiary/10 text-center"
                    >
                        { "More Details" }
                    </a>
                </div>
            </div>
        </section>
    }
}
