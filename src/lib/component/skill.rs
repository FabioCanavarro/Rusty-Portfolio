use leptos::*;

#[allow(dead_code)]
enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Proficient,
    Expert,
}

impl Into<String> for SkillLevel {
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
fn SkillCategory(category: &'static str, skills: Vec<(&'static str, SkillLevel)>) -> impl IntoView {
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
