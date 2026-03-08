use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product_name: String,
    pub product_price: f32,
}

#[function_component(ProductCard)]
fn product_card(props: &Props) -> Html {
    let style = use_style!(
        r#"
        background: white;
        border: 1px solid #E1E8ED;
        border-radius: 8px;
        padding: 1.5rem;
        text-align: center;

        img {
            width: 100%;
            height: 200px;
            object-fit: cover;
            border-radius: 4px;
            margin-bottom: 1rem;
        }

        h3 {
            color: #406280;
            margin-bottom: 0.5rem;
        }

        .price {
            font-size: 1.5rem;
            color: #2C3E50;
            font-weight: bold;
        }
        "#
    );
    html! {
        <div class={style}>
            <img src="https://placehold.co/600x400" alt={props.product_name.clone()} />
            <h3>{ props.product_name.clone() }</h3>
            <p class="price">{ format!("€{:.2}", props.product_price) }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let style = use_style!(
        r#"
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 2rem;
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
        "#
    );

    html! {
        <div class={style}>
            <ProductCard product_name="Headphones" product_price={79.10.0} />
            <ProductCard product_name="Test Product" product_price={99.99} />
            <ProductCard product_name="Test Product" product_price={99.99} />
            <ProductCard product_name="Test Product" product_price={99.99} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
