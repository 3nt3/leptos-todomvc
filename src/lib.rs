use leptos::*;

#[component]
pub fn Todos(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<String>>(cx, vec![]);
    let (input_value, set_input_value) = create_signal(cx, String::new());

    let input = move |event| set_input_value.set(event_target_value(&event));
    let add_todo = move |_| {
        set_todos.update(|todos| todos.push(input_value()));
        console_log(format!("{:?}", todos().len()).as_str());
    };
    let clear_todos = move |_| {
        set_todos.update(|todos| *todos = vec![]);
        set_input_value.set(String::new());
    };

    let remove_todo = move |x: String| {
        set_todos.update(|todos| {
            *todos = todos
                .iter()
                .filter(|todo| **todo != x)
                .map(|todo| todo.to_owned())
                .collect()
        });
    };

    view! { cx,
        <div>
            <h1>"TODOS"</h1>
            <span>"n todos: "{move || todos().len()}</span>
            <button on:click=clear_todos>"Clear Todos"</button>
            <input on:input=input />
            <button on:click=add_todo>"Add Todo"</button>
            <ul>
                <For
                    each=todos
                    key=|todo| todo.clone()
                    view=move |todo: String| {
                        view! {cx,
                            <li>
                                <span>{todo.clone()}</span>
                                <button on:click=move |_| remove_todo(todo.clone())>"Remove Todo"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
