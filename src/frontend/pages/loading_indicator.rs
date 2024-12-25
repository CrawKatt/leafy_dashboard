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
        <div class="fixed inset-0 flex items-center justify-center bg-gray-800 p-6">
            <div class="flex flex-col items-center">
                <img
                    src="/loading.gif"
                    alt="Loading Indicator"
                    class="w-42 h-42 mb-4"
                />
                <p class="text-white text-lg mb-2">"Loading..."</p>
                <div class="w-full bg-gray-700 rounded-full h-2.5">
                    <div
                        class="bg-blue-500 h-2.5 rounded-full animate-pulse"
                        style=move || format!("width: {}%", progress.get())
                    ></div>
                </div>
            </div>
        </div>
    }
}