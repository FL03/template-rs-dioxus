#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;


mod app;

fn main() -> anyhow::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let cnf = dioxus_web::Config::new().hydrate(true);
        dioxus_web::launch_with_props(App, (), cnf);
    }
    // launch the web app
    
    #[cfg(feature = "desktop")]
    {
        let cnf =  dioxus_desktop::Config::new();
        dioxus_desktop::launch_with_props(App, (), cnf);
    }
    #[cfg(feature = "ssr")]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.spawn(async {
            ssr::start().await?;
        })
    }
    Ok(())
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}

#[cfg(feature = "ssr")]
mod ssr {
    use axum::{response::Html, routing::get, Router};
    use dioxus::prelude::*;


    pub async fn rt() -> anyhow::Result<()> {
        
    }

    pub async fn start() -> anyhow::Result<()> {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
        println!("listening on http://{}", addr);

        axum::Server::bind(&addr)
            .serve(
                Router::new()
                    .route("/", get(app_endpoint))
                    .into_make_service(),
            )
            .await
            .unwrap();
        Ok(())
    }

    

    async fn app_endpoint() -> Html<String> {
        // create a VirtualDom with the app component
        let mut app = VirtualDom::new(super::App);
        // rebuild the VirtualDom before rendering
        let _ = app.rebuild();
    
        // render the VirtualDom to HTML
        Html(dioxus_ssr::render_vdom(&app))
    }
}

