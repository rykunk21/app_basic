use yew::prelude::*;

#[function_component(Services)]
pub fn services() -> Html {
    let flip = use_state(|| 0.0);

    let on_input = {
        let flip = flip.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                flip.set(input.value_as_number());
            }
        })
    };

    let rotation = format!("[transform:rotateY({}deg)]", *flip * 180.0);

    html! {
        <div class="flex flex-col items-center gap-8 mt-10 mx-4"> // mx-4 adds horizontal margin
            // Slider
            <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={flip.to_string()}
                oninput={on_input}
                class="w-64"
            />

            // Flip card container

            <div class="w-4/5 max-w-lg h-[80vh] sm:h-64 [perspective:1000px]">
                // w-4/5 → 80% of parent width
                // max-w-lg → optional maximum width
                <div class={classes!(
                    "relative",
                    "w-full",
                    "h-full",
                    "transition-transform",
                    "duration-700",
                    "[transform-style:preserve-3d]",
                    rotation
                )}>
                    // Front
                    <div class="absolute inset-0 bg-pink-600 flex flex-col items-center justify-center [backface-visibility:hidden] rounded-lg shadow-lg">
                        <h2 class="text-2xl text-white mb-2">{ "Models" }</h2>
                        <div class="w-24 h-24 bg-white rounded-md shadow-inner flex items-center justify-center text-black">
                            { "Model Box" }
                        </div>
                    </div>

                    // Back
                    <div class="absolute inset-0 bg-teal-500 flex flex-col items-center justify-center [transform:rotateY(180deg)] [backface-visibility:hidden] rounded-lg shadow-lg">
                        <h2 class="text-2xl text-white mb-2">{ "Agents" }</h2>
                        <div class="w-24 h-24 bg-white rounded-md shadow-inner flex items-center justify-center text-black">
                            { "Agent Box" }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
