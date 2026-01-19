use yew::prelude::*;

#[function_component(Grid3c)]
pub fn grid_3c() -> Html {
    html!(
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-3 lg:gap-8">
            <div class="h-32 rounded-lg bg-gray-200 border"></div>
            <div class="h-32 rounded-lg bg-gray-200"></div>
            <div class="h-32 rounded-lg bg-gray-200"></div>
        </div>
    )
}
