use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="yew logo" />
            <span class="subtitle">{ "from yew"}<i class="heart" /></span>
        </main>
    }
}
