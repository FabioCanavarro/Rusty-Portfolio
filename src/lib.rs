use leptos::*;
use leptos_meta::{self, Stylesheet};
use leptos_router::{self, Route, Router, Routes};
use leptos::component;
use thaw::Button;
use leptos::AnimatedShow;

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

#[component]
fn Portfolio() -> impl IntoView {
    let expanded = create_rw_signal(false);
    
    view! {
        <div class="bg-[#1e1e2e] text-[#cdd6f4] font-inter overflow-x-hidden min-h-screen">
            // Changed to flex-col on mobile, flex-row on desktop
            <div class="flex flex-col md:flex-row w-full min-h-screen">
                <LeftSection expanded=expanded/>
                <RightSection/>
            </div>
        </div>
    }
}
