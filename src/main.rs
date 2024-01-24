use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <div>
        // <img src="https://imgs.search.brave.com/PZAbInr7mW7LkWvAozSRgv1jKbefHuHn9QZzkJQ2Xpw/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9jZG4u/cGl4YWJheS5jb20v/cGhvdG8vMjAxNy8w/Ny8yMi8xOS8yNC9i/YWNrZ3JvdW5kLTI1/Mjk3MTZfNjQwLmpw/Zw" width="500" height="500" />
        
        <div>
        <button {onclick}>
        { "+1" }
        </button>
        
        
    
        <p>{ state.value }</p>
        </div>

        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}