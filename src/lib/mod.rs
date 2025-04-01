pub mod component;

use component::Portfolio;
use leptos::*;
use leptos_meta::{self, Stylesheet};
use leptos_router::{self, Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-portfolio.css"/>
        <Router>
            <Routes>
                <Route path="/" view=move || view! { <Portfolio/> }/>
            </Routes>
        </Router>
    }
}
