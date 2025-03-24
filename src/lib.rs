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

#[component]
fn LeftSection(expanded: RwSignal<bool>) -> impl IntoView {
    let toggle_expanded = move |_| expanded.update(|val| *val = !*val);
    
    let class = move || {
        if expanded.get() {
            "w-full fixed h-screen overflow-auto transition-all duration-500 ease-in-out z-20 transform-none"
        } else {
            // Changed to use sticky positioning and flex-based width
            "w-full md:w-2/5 h-screen md:sticky md:top-0 overflow-hidden transition-all duration-500 ease-in-out z-20"
        }
    };
    
    let overlay_class = move || {
        if expanded.get() {
            "fixed inset-0 bg-gradient-to-br from-black/10 to-black/90 backdrop-blur-lg opacity-75 transition-all duration-500 ease-in-out"
        } else {
            "absolute inset-0 bg-gradient-to-br from-black/10 to-black/90 backdrop-blur-lg transition-all duration-500 ease-in-out"
        }
    };
    
    view! {
        <div 
            class=class
            style="background: url('/api/placeholder/400/320') center center; background-size: cover;"
        >
            <div class=overlay_class></div>
            <div class="featured-repo relative z-10 p-8 h-full opacity-80 text-[#cdd6f4]">
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-2xl text-[#f5e0dc] font-semibold">Featured Projects</h2>
                    <Button
                        on_click=toggle_expanded
                        class="bg-[#cba6f7] text-[#1e1e2e] px-3 py-1 rounded hover:bg-[#f5c2e7] transition-colors"
                    >
                        {move || if expanded.get() { "Collapse" } else { "Expand" }}
                    </Button>
                </div>
                    <AnimatedShow
                        when=Signal::derive(move || true)
                        hide_delay= std::time::Duration::from_millis(300)
                        hide_class="opacity-0 transform translate-y-10"
                        show_class="opacity-100 transform translate-y-0"
                    >
                        <div class="description-box bg-[#1e1e2e]/80 backdrop-blur-sm rounded-lg p-6 mb-6">
                            <h3 class="text-xl text-[#f5e0dc] font-medium mb-2">About Me</h3>
                            <p class="text-sm mb-4">"I'm a passionate developer with expertise in Rust and Web Technologies. 
                            I love building high-performance applications with elegant solutions."</p>
                            
                        </div>
                    </AnimatedShow>
                <div class="grid grid-cols-1 gap-4">
                    <RepoCard 
                        title="Project Name" 
                        description="A brief description of this amazing project that showcases your skills and talents."
                        tags=vec!["Rust", "WebAssembly", "TypeScript"]
                    />
                    <RepoCard 
                        title="Another Project" 
                        description="Another brilliant project you've worked on that demonstrates your coding prowess."
                        tags=vec!["Rust", "Systems", "Performance"]
                    />
                    
                    
                    <h3 class="text-xl text-[#f5e0dc] font-medium mt-6 mb-4">All Repositories</h3>
                    
                    <RepoCard 
                        title="Project 1" 
                        description="Description of this repository and what it does."
                        tags=vec!["Rust", "CLI"]
                    />
                    <RepoCard 
                        title="Project 2" 
                        description="Description of this repository and what it does."
                        tags=vec!["TypeScript", "React"]
                    />
                    <RepoCard 
                        title="Project 3" 
                        description="Description of this repository and what it does."
                        tags=vec!["Python", "ML"]
                    />
                    <RepoCard 
                        title="Project 4" 
                        description="Description of this repository and what it does."
                        tags=vec!["Rust", "WebAssembly"]
                    />

                </div>
            </div>
        </div>
    }
}

#[component]
fn RepoCard(
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-[#1e1e2e]/70 rounded-lg p-6 border-l-4 border-[#cba6f7]">
            <h3 class="text-xl mb-2 text-[#f5e0dc] font-medium">{title}</h3>
            <p class="text-sm mb-2">{description}</p>
            <div class="flex flex-wrap gap-2 mt-2">
                {tags.into_iter().map(|tag| view! {
                    <span class="bg-[#585b70] text-[#cdd6f4] px-2 py-1 rounded text-xs">{tag}</span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
