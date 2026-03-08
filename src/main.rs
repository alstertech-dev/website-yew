use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product_name: String,
    pub product_price: f32,
}

#[component]
fn ProductCard(props: &Props) -> Html {
    html! {
        <div>
            <img src="https://placehold.co/600x400" alt={props.product_name.clone()} />
            <h2>{ props.product_name.clone() }</h2>
            <p class="price">{ format!("€{:.2}", props.product_price) }</p>
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
    let remove_from_cart = {
        let cart_products = cart_products.clone();
        move |_| {
            if *cart_products > 0 {
                let value = *cart_products - 1;
                cart_products.set(value);
            }
        }
    };

    let button_style = {
        let value = *cart_products;
        if value == 0 {
            "background-color: grey"
        } else {
            "background-color: white"
        }
        .to_string()
    };

    html! {
        <div style="padding: 2rem; font-family: 'Segoe UI', sans-serif;">
            <ProductCard
                product_name="Test Product"
                product_price={99.99}
            />
            <div style="margin-top: 1rem;">
                <button onclick={add_to_cart} style="margin-right: 0.5rem; padding: 0.5rem 1rem;">
                    { "In den Warenkorb" }
                </button>
                <button onclick={remove_from_cart} style={button_style}>
                    { "Entfernen" }
                </button>
            </div>
            <div style="margin-top: 1rem; font-weight: bold;">
                { format!("Im Warenkorb: {}", *cart_products) }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
