use leptos::*;
pub mod lib;
use lib::App;
fn main() {
    mount_to_body(|| view! { <App/> })
}