use leptos::*;

use super::SkillsAndConnect;
use super::ProfileInfo;

#[component]
pub fn RightSection(expanded: RwSignal<bool>) -> impl IntoView {
    view! {
        <div 
            class=move || {
                let base_class = "w-full md:w-3/5 min-h-screen p-8 transition-all duration-700 ease-in-out transform";
                match expanded.get() {
                    true => format!("{} md:translate-x-full md:opacity-100 scale-95", base_class),
                    false => format!("{} md:translate-x-0 md:opacity-100 scale-100", base_class)
                }
            }
        >
            <AnimatedShow
                when=Signal::derive(move || !expanded.get())
                hide_delay=std::time::Duration::from_millis(300)
                show_class="transition-all duration-700 ease-out transform opacity-100 translate-y-0"
                hide_class="opacity-0 translate-y-10 scale-95"
            >
                <div class="relative flex flex-col md:flex-row justify-end space-y-4 md:space-y-0 md:space-x-4">
                    <ProfileInfo/>
                    <div class="divider divider-horizontal"></div>
                    <SkillsAndConnect/>
                </div>
            </AnimatedShow>
        </div>
    }
}