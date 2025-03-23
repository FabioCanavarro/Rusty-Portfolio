

use leptos::prelude::*;
use thaw::*;



#[component]
pub fn App() -> impl IntoView{
    let (count,setcount) = signal(0);

    view! {

        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click= move |_| setcount.update(|count| *count += 1) class="btn">
                        "Click number " {count}
                    </button>
                </div>
                <Card title="Card".to_string() img="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp".to_string() body="Body".to_string()/>
            </div>
        </main>
    }

}

#[component]
pub fn Card(title: String, img: String,body: String) -> impl IntoView {
    view! {
        <div class="card bg-base-100 w-96 shadow-sm">
            <figure>
                <img
                src={img}
                alt={title.clone()} />
            </figure>
            <div class="card-body">
                <h2 class="card-title">
                    {title}
                <div class="badge badge-secondary">NEW</div>
                </h2>
                <p>{body}</p>
                <div class="card-actions justify-end">
                <div class="badge badge-outline">Rust</div>
                <div class="badge badge-outline">Products</div>
                </div>
            </div>
        </div>
    }
}
