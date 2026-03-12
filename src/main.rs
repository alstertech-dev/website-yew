use yew::prelude::*;

const EMAIL: &str = "alstertech@alstergymnasium.de";

#[derive(Clone, PartialEq)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub image: String,
    pub gender: grammar::Gender,
    pub number: grammar::Number,
}

mod grammar {
    #[derive(Clone, PartialEq)]
    pub enum Gender {
        Masculine,
        Feminine,
        Neutral,
    }

    #[derive(Clone, PartialEq)]
    pub enum Number {
        Singular,
        Plural,
    }

    #[derive(Clone, PartialEq)]
    pub enum Case {
        Nominative,
        Accusative,
    }

    pub fn get_definite_article(gender: Gender, number: Number, case: Case) -> String {
        match number {
            Number::Plural => "die",
            Number::Singular => match gender {
                Gender::Masculine => match case {
                    Case::Nominative => "der",
                    Case::Accusative => "den",
                },
                Gender::Feminine => match case {
                    Case::Nominative | Case::Accusative => "die",
                },
                Gender::Neutral => match case {
                    Case::Nominative | Case::Accusative => "das",
                },
            },
        }
        .to_owned()
    }
}

use grammar::get_definite_article;

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
        },
        Product {
            name: "Linse".into(),
            price: 249.00,
            image: "lense.jpg".into(),
            gender: grammar::Gender::Feminine,
            number: grammar::Number::Plural,
        },
        Product {
            name: "Test Product".into(),
            price: 99.99,
            image: "t-rex.jpg".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
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
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <p class="modal-product-name">{ format!("{} -- €{:.2}", product.name, product.price) }</p>
                <p>
                    {
                        format!("Onlinebestellungen sind noch nicht verfügbar.
                            Wenn Sie {} {} bestellen möchten, oder uns einfach eine Frage stellen wollen, schreiben Sie uns gerne eine Email an:
                        ", get_definite_article(product.gender, product.number, grammar::Case::Accusative), product.name)
                    }
                </p>
                <p class="modal-email">
                {format!("{EMAIL}")}
                </p>
            </div>
            }
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
