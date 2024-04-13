use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

/// Shows progress towards a goal
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar
    #[prop(default = 100)]
    max: u16,
    /// how much progress should be displayed
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

///Trait Bounds error

#[component]
fn DynamicList(
    /// The number of Counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id: usize = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id: usize| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| {|| {counter.0}}
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>{move |_| count.get()}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters
                                                .retain(|(counter_id, (signal, _))| {
                                                    if counter_id == &id {
                                                        signal.dispose();
                                                    }
                                                    counter_id != &id
                                                })
                                        });
                                }>

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
    }
}

// #[component]
// fn App() -> impl IntoView {
//     let length = 5;
//     let counters = (1..length).map(|idx| create_signal(idx));

//     let counter_buttons = counters
//         .map(|(count, set_count)| {
//             view! {
//                 <li>
//                     <button on:click=move |_| {set_count.update(|n| *n += 1);}>
//                         {move || count.get()}
//                         </button>
//                 </li>
//             }
//         })
//         .collect_view();
//     view! { <ul>{counter_buttons}</ul> }

//     // let (count, set_count) = create_signal(0);
//     // let double_count = move || count.get() * 2;
//     // view! {
//     //     <button on:click = move |_| {set_count.update(|n| *n += 1);}>
//     //         "Click Me"
//     //     </button>

//     //     <ProgressBar progress=count/>
//     //     <ProgressBar progress=Signal::derive(double_count)/>
// }
