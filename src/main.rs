use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product_name: String,
    pub product_price: f32,
}

#[component]
fn ProductCard(
    &Props {
        ref product_name,
        product_price,
    }: &Props,
) -> Html {
    html! {
        <div>
            <img src="" />
            <h3>{ product_name }</h3>
            <p class="price">{ format!("€{product_price}") }</p>
        </div>
    }
}

#[component]
fn App() -> Html {
    let cart_products = use_state(|| 0);
    let add_to_cart = {
        let cart_products = cart_products.clone();
        move |_| {
            let value = *cart_products + 1;
            cart_products.set(value);
        }
    };

    html! {
        <div>
            <div class={classes!("products")}>
                <ProductCard product_name="Test Product" product_price=99.99/>
                <button onclick= {add_to_cart}>{ "In den Warenkorb" }</button>
            </div>
            <div>{ format!("products {}", *cart_products) }</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
