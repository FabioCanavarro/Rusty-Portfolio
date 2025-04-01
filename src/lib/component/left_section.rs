use leptos::AnimatedShow;
use leptos::*;
use thaw::Button;

use super::RepoCard;

#[component]
pub fn LeftSection(expanded: RwSignal<bool>) -> impl IntoView {
    let toggle_expanded = move |_| expanded.update(|val| *val = !*val);

    
    let class = move || {
        if expanded.get() {
            "w-full fixed h-screen overflow-auto transition-all duration-500 ease-in-out z-20 transform-none"
        } else {
            // Changed to use sticky positioning and flex-based width
            "w-full md:w-[30%] h-screen md:sticky md:top-0 overflow-hidden transition-all duration-500 ease-in-out z-20"
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
                    <div class="padding-1">
                        <div class="divider divider-horizontal"></div>
                        <Button
                            on_click=toggle_expanded
                            class="bg-[#cba6f7] text-[#1e1e2e] px-3 py-1 rounded hover:bg-[#f5c2e7] transition-colors"
                        >

                            {move || if expanded.get() { "Collapse" } else { "Expand" }}
                        </Button>
                    </div>
                </div>

                <div class="divider divider-vertical"></div>
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
                        <div class="divider divider-vertical"></div>
                    </AnimatedShow>
                <div class="grid grid-cols-1 gap-4">
                    <RepoCard
                        title="Project Name"
                        description="A brief description of this amazing project that showcases your skills and talents.
                            lorem ipsum dolor sit amet .consectetur adipiscing .elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
                            ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat jfejenwbhvjsdn
                            vsdbvghsdvbnf sdvn sdvjersdbjrvgbsjdvbnsd vn dsnvsdnjvnsd vbnds vbn sjvbsnjjvbhsdbvbjSBvb zsbdv zsm vd vbmn zsnbvbs
                            zdvbsnzv .fe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenj
                            enfjkefe bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe bf
                            jenjenfjkefe bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe  bfjenjenfjkefe"
                        tags=vec!["Rust", "WebAssembly", "TypeScript"]
                    />
                    <RepoCard
                        title="Project Name"
                        description="A brief description of this amazing project that showcases your skills and talents."
                        tags=vec!["Rust", "WebAssembly", "TypeScript"]
                    />

                </div>
            </div>
        </div>
    }
}