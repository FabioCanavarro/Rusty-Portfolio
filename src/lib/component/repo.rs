use leptos::*;

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

