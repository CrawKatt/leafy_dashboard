use leptos::prelude::*;

#[component]
pub fn LoadingIndicator() -> impl IntoView {
    let (progress, set_progress) = signal(0);

    Effect::new(move |_| {
        for index in (0..=100).step_by(5) {
            set_progress.set(index);
        }
    });

    view! {
        <div class="flex fixed inset-0 justify-center items-center p-6 bg-gray-800">
            <div class="flex flex-col items-center">
                <img src="/loading.gif" alt="Loading Indicator" class="mb-4 w-42 h-42" />
                <p class="mb-2 text-lg text-white">"Loading..."</p>
                <div class="w-full h-2.5 bg-gray-700 rounded-full">
                    <div
                        class="h-2.5 bg-blue-500 rounded-full animate-pulse"
                        style=move || format!("width: {}%", progress.get())
                    ></div>
                </div>
            </div>
        </div>
    }
}