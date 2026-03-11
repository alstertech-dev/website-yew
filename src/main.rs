use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub image: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product: Product,
    pub on_click: Callback<Product>,
}

#[function_component(ProductCard)]
fn product_card(props: &Props) -> Html {
    let on_click = {
        let product = props.product.clone();
        let on_click = props.on_click.clone();
        move |_: MouseEvent| on_click.emit(product.clone())
    };

    html! {
        <div class="product-card" onclick={on_click}>
            <img src={format!("/img/products/{}", &props.product.image)} alt={props.product.name.clone()} />
            <h2>{ props.product.name.clone() }</h2>
            <p class="price">{ format!("€{:.2}", props.product.price) }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let modal = use_state(|| None::<Product>);

    let open_modal = {
        let selected = modal.clone();
        move |product: Product| selected.set(Some(product))
    };
    let close_modal = {
        let selected = modal.clone();
        move |_| selected.set(None)
    };

    let products = vec![
        Product {
            name: "Headphones".into(),
            price: 79.10,
            image: "headphones.jpg".into(),
        },
        Product {
            name: "Camera Lense".into(),
            price: 249.00,
            image: "lense.jpg".into(),
        },
        Product {
            name: "Test Product".into(),
            price: 99.99,
            image: "t-rex.jpg".into(),
        },
    ];

    html! {
        <>
        <div class="products">
            { for products.iter().map(|p| html! {
                <ProductCard product={p.clone()} on_click={open_modal.clone()} />
            })}
        </div>
        <div class={ if modal.is_some() { "modal-overlay open" } else { "modal-overlay" } } onclick={close_modal}>
            if let Some(product) = (*modal).clone() {
                <div class="modal">
                    <p class="modal-product-name">{ format!("{} -- €{:.2}", product.name, product.price) }</p>
                </div>
            }
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
