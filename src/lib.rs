use leptos::component;
use leptos::AnimatedShow;
use leptos::*;
use leptos_meta::{self, Stylesheet};
use leptos_router::{self, Route, Router, Routes};
use thaw::Button;

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

#[component]
fn RightSection() -> impl IntoView {
    view! {
        <div class="w-full md:w-3/5 min-h-screen p-8">
            <div class="relative flex justify-end">
                <ProfileInfo/>
            </div>
        </div>
    }
}

#[component]
fn ProfileInfo() -> impl IntoView {
    view! {
        <div class="profile-info z-10 p-6 bg-[#1e1e2e]/80 backdrop-blur-sm rounded-lg mt-8">
            <h1 class="text-3xl md:text-4xl mb-4 font-bold" style="background: linear-gradient(45deg, #f5e0dc, #cba6f7); -webkit-background-clip: text; background-clip: text; color: transparent;">
                Fabio Canavarro
            </h1>
            <div>
                <CodeSnippet/>
            </div>
            <div class="flex gap-4 mb-6">
                <a
                    href="https://github.com/FabioCanavarro"
                    target="_blank"
                    class="text-[#cdd6f4] no-underline flex items-center gap-2 transition-colors duration-300 hover:text-[#cba6f7]"
                >
                    GitHub
                </a>
            </div>

            <GithubStats/>
            <TechStack/>
        </div>
    }
}

#[component]
fn CodeSnippet() -> impl IntoView {
    view! {
        <div class="absolute -top-[60px]  left-[100px] -z-[2] bg-[#1e1e2e]/30 rounded-lg p-4 font-mono text-sm text-[#cdd6f4] overflow-hidden mt-4 w-full max-w-2xl">
            <pre><code class="relative font-size-10 bottom-[15px] bg-gradient-to-br from-[#cba6f7] to-transparent text-transparent bg-clip-text">{"fn main() {
    let fibonacci = |n: u32| -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    };
    
    for i in 0..10 {
        println!(\"Fibonacci({}) = {}\", i, fibonacci(i));
    }
}"}</code></pre>
        </div>
    }
}

#[component]
fn GithubStats() -> impl IntoView {
    view! {
        <div class="mb-6">
            <h2 class="text-lg md:text-xl mb-3 text-[#f5e0dc] font-semibold">"📊 GitHub Stats"</h2>
            <div class="flex flex-col gap-3">
                <img src="https://github-readme-stats.vercel.app/api?username=FabioCanavarro&theme=catppuccin_mocha&hide_border=false&include_all_commits=true&count_private=true" alt="GitHub Stats" class="w-full" />
                <img src="https://nirzak-streak-stats.vercel.app/?user=FabioCanavarro&theme=catppuccin_mocha&hide_border=false" alt="GitHub Streak" class="w-full" />
                <img src="https://github-readme-stats.hackclub.dev/api/wakatime?username=149&api_domain=hackatime.hackclub.com&theme=omni&custom_title=Hackatime+Stats&layout=compact&cache_seconds=0&langs_count=8" alt="Wakatime Stats" class="w-full" />
            </div>
        </div>
    }
}

#[component]
fn TechStack() -> impl IntoView {
    view! {
        <div class="my-4">
            <h2 class="text-lg md:text-xl mb-3 text-[#f5e0dc] font-semibold">"💻 Tech Stack"</h2>
            <div class="flex flex-wrap gap-4 mt-2">
                <TechIcon icon="🦀" name="Rust" />
                <TechIcon icon="PY" name="Python" />
            </div>
        </div>
    }
}

#[component]
fn TechIcon(icon: &'static str, name: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-1">
            <div class="w-10 h-10 bg-[#313244] rounded-lg flex items-center justify-center text-xl">
                {icon}
            </div>
            <div class="text-xs">{name}</div>
        </div>
    }
}

#[component]
fn AllRepos(expanded: Memo<bool>) -> impl IntoView {
    let display_class = move || {
        if expanded.get() {
            "w-3/5 p-8 min-h-screen fixed right-0 top-0 z-30 overflow-y-auto bg-[#1e1e2e]"
        } else {
            "hidden"
        }
    };

    view! {
        <div class=display_class>
            <h2 class="text-2xl mb-4 text-[#f5e0dc] font-semibold mt-16">All Repositories</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-8">
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
    }
}
