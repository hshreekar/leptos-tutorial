use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> }) 
}


#[component]
fn ProgressBar(
    #[prop(default=100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>) -> impl IntoView {
    view! {
        <progress 
            max=max
            value=progress
        />
    }
}  

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    view! {
        <button on:click=move |_| {set_count.update(|n| *n += 1);}>
            "Click Me"
        </button>

        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
    }
}
