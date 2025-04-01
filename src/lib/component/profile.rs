use leptos::*;

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
