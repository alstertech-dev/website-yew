use grammar::get_definite_article;
use yew::prelude::*;
use yew_router::prelude::*;

mod grammar;

const EMAIL: &str = "alstertech@alstergymnasium.de";

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
    #[at("/imprint")]
    Imprint,
}

#[derive(Clone, PartialEq)]
pub struct Product {
    pub name: AttrValue,
    pub price: f32,
    pub image: AttrValue,
    pub gender: grammar::Gender,
    pub number: grammar::Number,
    pub manual: Option<AttrValue>,
    pub resources: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub product: Product,
    pub on_click: Callback<Product>,
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav class="nav">
            <Link<Route> to={Route::Home}><img src="img/altertech-logo.svg"/></Link<Route>>
            <ul class="nav-menu">
                <li><Link<Route> to={Route::Home} classes="btn btn-outline">{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::Products} classes="btn btn-outline">{"Produkte"}</Link<Route>></li>
                <li><Link<Route> to={Route::Imprint} classes="btn btn-outline">{"Impressum"}</Link<Route>></li>
            </ul>
        </nav>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="footer">
        <img src="img/altertech-logo-light.svg"/>
        <a href="imprint">{"Impressum"}</a>
        </footer>
    }
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

fn product_list() -> Vec<Product> {
    vec![
        Product {
            name: "Kopfhörer".into(),
            price: 79.10,
            image: "earphones.webp".into(),
            gender: grammar::Gender::Feminine,
            number: grammar::Number::Plural,
            manual: Some("downloads/Kopfhörer Anleitung.pdf".into()),
            resources: None,
        },
        Product {
            name: "T-Rex Modelle".into(),
            price: 15.00,
            image: "t-rex.webp".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
            manual: None,
            resources: Some("downloads/Masse eines Tyrannosaurus Rex Arbeitsblatt.pdf".into()),
        },
        Product {
            name: "Spektrometer".into(),
            price: 15.00,
            image: "spektrometer.webp".into(),
            gender: grammar::Gender::Neutral,
            number: grammar::Number::Singular,
            manual: None,
            resources: None,
        },
    ]
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

    let products = product_list();

    html! {
        <main>
        <h1>{"Unsere Produkte"}</h1>
        <div class="products-grid">
            { for products.iter().map(|p| html! {
                <ProductCard product={p.clone()} on_click={open_modal.clone()} />
            })}
        </div>
        <div class={ if modal.is_some() { "modal-overlay open" } else { "modal-overlay" } } onclick={close_modal}>
            if let Some(product) = (*modal).clone() {
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <h3 class="modal-product-name">{&product.name}{" — €"}{format!("{:.2}", &product.price)}</h3>
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
                    {" bestellen möchten, oder uns einfach eine Frage stellen wollen, schreiben Sie uns gerne eine E-Mail an:"}
                </p>
                <a class="modal-email" href={format!("mailto:{}",EMAIL)}>
                {EMAIL}
                </a>
            </div>
            }
        </div>
        </main>
    }
}

#[function_component(Imprint)]
fn imprint() -> Html {
    html! {
        <main>
          <h1>{ "Impressum" }</h1>
          <div class="imprint-wrapper">
            <div class="imprint-section">
              <h2>{ "Geschäftsleitung" }</h2>
              <p>
              {"Jannes Neufert, Geschäftsführer"}
              </p>
              <h2>{"Anschrift"}</h2>
              <address>
              {"AlsterTech"}
              <br/>
              {"Maurepasstraße 67, 24558 Henstedt-Ulzburg"}
              <br/>
              {"E-Mail: "}
              <a href={format!("mailto:{}", EMAIL)}>{EMAIL}</a>
              </address>
            </div>
          </div>
        </main>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <main>
        <h1> {"Die Schülerfirma des Alstergymnasiums"}</h1>
        <p>{
            "Willkommen bei AlsterTech! Wir wurden 2025 als Projekt im Profilseminar des Physikprofils am Alstergymnasium in Henstedt-Ulzburg gegründet. Unser Ziel ist es, für einen spannenderen Physikunterricht zu sorgen, indem wir interaktive Produkte anbieten, die den häufig sehr theoretischen Physikunterricht anschaulicher machen sollen."
        }</p>
        <h2> {"Was ist eine Schülerfirma?"}</h2>
        <h2> {"Unser Team"} </h2>
        <p>{
            "AlsterTech besteht aus einem engagierten Team von Schüler*innen, die zum ersten Mal die Erfahrung machen, Teil einer Firma zu sein. Für die Entwicklung und Produktion unserer Produkte verwenden wir den Makerspace, einen Raum mit 3D-Druckern und anderen technischen Geräten."
        }</p>
        </main>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Home/>),
        Route::Products => html! {<ProductsPage/>},
        Route::Imprint => html! {<Imprint/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
