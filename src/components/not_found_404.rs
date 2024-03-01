use leptos::*;

#[component]
pub fn NotFound404() -> impl IntoView {
    view! {
        <div class="h-full w-full flex place-items-center place-content-center">
            <h1 class="-translate-y-8 text-3xl font-bold">"Not Found"</h1>
        </div>
    }
}
