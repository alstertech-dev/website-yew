use grammar::get_definite_article;
use yew::prelude::*;

const EMAIL: &str = "alstertech@alstergymnasium.de";

#[derive(Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub image: String,
    pub gender: grammar::Gender,
    pub number: grammar::Number,
    pub manual: Option<String>,
    pub resources: Option<String>,
}

mod grammar;

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
        Callback::from(move |_: MouseEvent| on_click.emit(product.clone()))
    };
    html! {
        <div class="product-card" onclick={on_click}>
            <img class="product-image" src={format!("/img/products/{}", &props.product.image)} alt={props.product.name.clone()} />
            <div class="product-info">
                <h3 class="product-name">{ props.product.name.clone() }</h3>
                <div class="product-footer">
                    <span class="product-price">{ format!("€{:.2}", props.product.price) }</span>
                    <a style="padding: 0.5rem 1rem; font-size: 0.8rem" class="btn btn-outline">{"Infos"}</a>
                </div>
            </div>
        </div>
    }
}

#[function_component(ProductsPage)]
fn products_page() -> Html {
    let modal = use_state(|| None::<Product>);

    let open_modal = {
        let selected = modal.clone();
        Callback::from(move |product: Product| selected.set(Some(product)))
    };
    let close_modal = {
        let selected = modal.clone();
        Callback::from(move |_| selected.set(None))
    };

    let products = vec![
        Product {
            name: "Kopfhörer".into(),
            price: 79.10,
            image: "headphones.jpg".into(),
            gender: grammar::Gender::Feminine,
            number: grammar::Number::Plural,
            manual: Some("https://github.com/innin-jam".to_owned()),
            resources: None,
        },
        Product {
            name: "Linse".into(),
            price: 249.00,
            image: "lense.jpg".into(),
            gender: grammar::Gender::Feminine,
            number: grammar::Number::Plural,
            manual: None,
            resources: None,
        },
        Product {
            name: "T-Rex Modelle".into(),
            price: 15.00,
            image: "t-rex.jpg".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
            manual: None,
            resources: Some("downloads/Masse eines Tyrannosaurus Rex Arbeitsblatt.pdf".to_owned()),
        },
        Product {
            name: "Test Product".into(),
            price: 99.99,
            image: "t-rex.jpg".into(),
            gender: grammar::Gender::Feminine,
            number: grammar::Number::Plural,
            manual: None,
            resources: None,
        },
        Product {
            name: "Test Product".into(),
            price: 99.99,
            image: "t-rex.jpg".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
            manual: None,
            resources: None,
        },
        Product {
            name: "Test Product".into(),
            price: 99.99,
            image: "t-rex.jpg".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
            manual: None,
            resources: None,
        },
    ];

    html! {
        <>
        <div class="products-grid">
            { for products.iter().map(|p| html! {
                <ProductCard product={p.clone()} on_click={open_modal.clone()} />
            })}
        </div>
        <div class={ if modal.is_some() { "modal-overlay open" } else { "modal-overlay" } } onclick={close_modal}>
            if let Some(product) = (*modal).clone() {
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <h3 class="modal-product-name">{ format!("{} — €{:.2}", product.name, product.price) }</h3>
                <p>
                if let Some(manual) = product.manual {
                    <a style="padding: 0.4rem 1rem; font-size: 0.8rem; margin-right: 10px;" class="btn btn-outline" href={manual} target="_blank" rel="noopener noreferrer">{"Anleitung"}</a>
                }
                if let Some(resources) = product.resources {
                    <a style="padding: 0.4rem 1rem; font-size: 0.8rem;" class="btn btn-outline" href={resources} target="_blank" rel="noopener noreferrer">{"Weitere Ressourcen"}</a>
                }
                </p>
                <p>
                    { "Onlinebestellungen sind auf der Website momentan nicht verfügbar. Wenn Sie " }
                    {get_definite_article(product.gender, product.number, grammar::Case::Accusative)}{" "}
                    <strong>{product.name}</strong>
                    {" bestellen möchten, oder uns einfach eine Frage stellen wollen, schreiben Sie uns gerne eine Email an:"}
                </p>
                <br/>
                <a class="modal-email" href={format!("mailto:{}", EMAIL)}>
                {EMAIL}
                </a>
            </div>
            }
        </div>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav class="nav">
          <a class="logo" href="/"><img src="img/altertech-logo.svg"/></a>
          <ul class="nav-menu">
            <li><a class="btn btn-outline" href="/">{"Home"}</a></li>
            <li><a class="btn btn-outline" href="products">{"Produkte"}</a></li>
            <li><a class="btn btn-outline" href="kontakt">{"Kontakt"}</a></li>
          </ul>
        </nav>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Nav/>
        <ProductsPage/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
