use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[function_component]
pub fn Products() -> Html {
    let data: UseStateHandle<Vec<Product>> = use_state(|| vec![]);
    let data_clone = data.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let fetched_data = reqwest::get("http://localhost:3000/api/products")
            .await
            .expect("cannot get data from url")
            .json::<Vec<Product>>()
            .await
            .expect("cannot convert to json");

        data_clone.set(fetched_data);
    });

    html! {
        <div class="container">
            <h2>{"List of products: "} {data.len()}{" products"}</h2>
            <p></p>
        </div>
    }
}