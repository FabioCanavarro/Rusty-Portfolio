extern crate leptos;
extern crate thaw;
extern crate leptos_meta;
extern crate leptos_router;

pub mod components;


use leptos::prelude::*;

use components::App;

fn main() {
    mount_to_body(App);
}




