/*
    Appellation: screens <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{
    blog::Blog,
    home::Home,
};

mod blog;
mod home;

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}