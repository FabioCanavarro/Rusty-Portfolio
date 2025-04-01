pub mod left_section;
pub mod profile;
pub mod repo;
pub mod right_section;
pub mod skill;

use left_section::LeftSection;
use leptos::*;
use profile::*;
use repo::*;
use right_section::RightSection;
use skill::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    let expanded = create_rw_signal(false);

    view! {
        <div class="bg-[#1e1e2e] text-[#cdd6f4] font-inter overflow-x-hidden min-h-screen">
            <div class="flex flex-col md:flex-row w-full min-h-screen">
                <LeftSection expanded=expanded/>
                <RightSection expanded = expanded/>
            </div>
        </div>
    }
}
