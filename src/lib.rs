use leptos::*;

#[component]
pub fn SimpleCounter(cx: Scope) -> impl IntoView {
    let (counter, set_counter) = create_signal(cx, -1);

    let reset_counter = move |_| set_counter(3);

    view! { cx,
        <div>
            <span>"counter: " {counter}</span>
            <button on:click=reset_counter >"+"</button>
        </div>
    }
}
