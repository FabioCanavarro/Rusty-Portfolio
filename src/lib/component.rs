use leptos::AnimatedShow;
use leptos::*;
use leptos_meta::{self, Stylesheet};
use leptos_router::{self, Route, Router, Routes};
use thaw::Button;

#[allow(dead_code)]
enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Proficient,
    Expert,
}

impl Into <String> for SkillLevel {
    fn into(self) -> String {
        match self {
            SkillLevel::Beginner => "10".to_string(),
            SkillLevel::Intermediate => "30".to_string(),
            SkillLevel::Advanced => "60".to_string(),
            SkillLevel::Proficient => "70".to_string(),
            SkillLevel::Expert => "90".to_string(),
        }
    }
}

#[component]
pub fn RepoCard(
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
) -> impl IntoView {
    
    let is_long_description = description.lines().count() > 3 || description.len() > 150;

    view! {
        <div class="bg-[#1e1e2e]/70 rounded-lg p-6 border-l-4 border-[#cba6f7]">
            <h3 class="text-xl mb-2 text-[#f5e0dc] font-medium">{title}</h3>
            // Show dropdown button only if description is long
            {move || {
                if is_long_description {
                    view! {
                        <div tabindex="0" class="collapse collapse-arrow bg-[#a37ecF] border-base-300 border ">
                            <div class="collapse-title font-semibold">Description</div>
                            <div class="collapse-content text-sm">
                                {description}
                            </div>
                        </div>
                        /* <Button
                            on_click=toggle_expansion
                            class="btn btn-soft btn-primary text-xs mt-0.2 py-0 px-1.8 transition-colors h-[1.5rem] hover:bg-[#cba6f7] hover:text-[#1e1e2e] mb-5"
                        >
                            {move || if is_expanded.get() { "Show Less" } else { "Show More" }}
                        </Button> */
                    }.into_view()
                } else {
                    view! {<div tabindex="0" class="collapse collapse-arrow collapse-open bg-base-100 border-base-300 border">
                        <div class="collapse-title font-semibold">Description</div>
                        <div class="collapse-content text-sm">
                            {description}
                        </div>
                    </div>
                    }.into_view()
                }
            }}
            
            <div class="flex flex-wrap gap-2 py-3">
                {tags.into_iter().map(|tag| view! {
                    <span class="bg-[#585b70] text-[#cdd6f4] px-2 py-1 rounded text-xs">{tag}</span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <div class="profile-info z-10 p-6 bg-[#313244] rounded-lg mt-8 w-[60%] bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20 backdrop-saturate-100 backdrop-contrast-120">
            <h1 class="text-3xl md:text-4xl mb-4 font-bold" style="background: linear-gradient(45deg, #f5e0dc, #cba6f7); -webkit-background-clip: text; background-clip: text; color: transparent;">
                Fabio Canavarro
            </h1>
            <div>
                <CodeSnippet/>
            </div>
            <div class="link link-hover flex gap-4 mb-6">
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
pub fn CodeSnippet() -> impl IntoView {
    view! {
        <div class="absolute top-[5px] left-[310px] -z-[2] rounded-lg p-4 font-mono text-xs text-[#cdd6f4]/30 overflow-hidden mt-4 w-full max-w-2xl">
            <pre><code class="relative font-size-10 bottom-[15px] bg-gradient-to-br from-[#cdd6f4]/60 to-transparent text-transparent bg-clip-text">{"
fn main() {
    let fibonacci = |n: u32| -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                
                }
                "}</code></pre>
        </div>
    }
}

#[component]
pub fn GithubStats() -> impl IntoView {
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
pub fn TechStack() -> impl IntoView {
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
pub fn TechIcon(icon: &'static str, name: &'static str) -> impl IntoView {
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
pub fn AllRepos(expanded: Memo<bool>) -> impl IntoView {
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

#[component]
pub fn SkillsAndConnect() -> impl IntoView {
    view! {
        <div class="bg-[#1e1e2e]/70 rounded-lg p-6 mt-6">
            <div class="grid md:grid-cols-2 gap-6">
                <div>
                    <h2 class="text-xl mb-4 text-[#f5e0dc] font-semibold">"🚀 Skills Matrix"</h2>
                    <div class="space-y-4">
                        <SkillCategory 
                            category="Programming" 
                            skills=vec![
                                ("Rust", SkillLevel::Intermediate),
                                ("Python", SkillLevel::Proficient),
                            ]
                        />
                        <SkillCategory 
                            category="Technologies" 
                            skills=vec![
                                ("Git", SkillLevel::Intermediate),
                                ("Leptos", SkillLevel::Beginner),
                            ]
                        />
                    </div>
                </div>

                <div>
                    <h2 class="text-xl mb-4 text-[#f5e0dc] font-semibold">"📬 Let's Connect"</h2>
                    <div class="space-y-4">
                        <div class="flex items-center gap-3">
                            <span class="text-[#cba6f7]">"📧"</span>
                            <a 
                                href="mailto:fakekelios071@gmail.com" 
                                class="text-[#cdd6f4] hover:text-[#f5e0dc] transition-colors"
                            >
                                fabio.canavarro@example.com
                            </a>
                        </div>
                        <div class="flex items-center gap-3">
                            <span class="text-[#cba6f7]">"🌐"</span>
                            <a 
                                href="https://www.linkedin.com/in/fabio-canavarro-584b232a7/" 
                                target="_blank"
                                class="text-[#cdd6f4] hover:text-[#f5e0dc] transition-colors"
                            >
                                LinkedIn Profile
                            </a>
                        </div>
                        <div class="flex items-center gap-3">
                            <span class="text-[#cba6f7]">"🐦"</span>
                            <a 
                                href="https://twitter.com/fabiocanavarro" 
                                target="_blank"
                                class="text-[#cdd6f4] hover:text-[#f5e0dc] transition-colors"
                            >
                                Twitter/X
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        }
    }

#[component]
pub fn SkillCategory(
    category: &'static str, 
    skills: Vec<(&'static str, SkillLevel)>
) -> impl IntoView {
    view! {
        <div>
            <h3 class="text-lg mb-2 text-[#f5e0dc]">{category}</h3>
            <div class="space-y-2">
                {skills.into_iter().map(|(skill, level)| view! {
                    <div class="flex items-center gap-5">
                        <div class="w-1/3 text-sm text-[#cdd6f4]">{skill}</div>
                        <progress class="progress w-56" value={&<SkillLevel as Into<String>>::into(level)} max="100"></progress>

                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}