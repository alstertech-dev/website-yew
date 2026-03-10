use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product_name: String,
    pub product_price: f32,
}

#[function_component(ProductCard)]
fn product_card(props: &Props) -> Html {
    html! {
        <div class="product-card">
            <img src="https://placehold.co/600x400" alt={props.product_name.clone()} />
            <h2>{ props.product_name.clone() }</h2>
            <p class="price">{ format!("€{:.2}", props.product_price) }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="products">
            <ProductCard product_name="Headphones" product_price={79.10} />
            <ProductCard product_name="Test Product" product_price={99.99} />
            <ProductCard product_name="Test Product" product_price={99.99} />
            <ProductCard product_name="Test Product" product_price={99.99} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
