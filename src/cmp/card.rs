/*
    Appellation: card <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use dioxus::prelude::*;

pub fn prefixer_dash<T>(prefix: &str, value: T) -> String
where
    T: core::fmt::Display,
{
    format!("{prefix}{value}")
}

pub fn prefix_option<T>(prefix: &str, value: Option<T>) -> String
where
    T: AsRef<str>,
{
    value.map(|v| format!("{prefix}{val}", val = v.as_ref())).unwrap_or_default()
}

pub fn args_to_class<T: ToString>(args: Vec<Option<T>>) -> String {
    args.iter().map(|arg| arg.as_ref().map(|i| i.to_string()).unwrap_or_default()).collect::<Vec<_>>().join(" ")
}

#[component]
pub fn Card(background: Option<String>, child: Element) -> Element {
    let bg = background.unwrap_or("bg-white".to_string());
    // let color = color.unwrap_or("text-black");
    // let margin = prefix_option("m-", margin);
    // let padding = prefix_option("p-", padding);
    // let rounded = rounded.unwrap_or("rounded-lg");
    // let sh = shadow.unwrap_or("");
    
    rsx! {
        div { class: "{bg}", {child} }
    }
}


