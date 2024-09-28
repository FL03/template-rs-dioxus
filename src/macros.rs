/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_macros)]

#[macro_use]
pub(crate) mod create;
#[macro_use]
pub(crate) mod model;
#[macro_use]
pub(crate) mod wrapper;

#[macro_export]
macro_rules! profile {
    ($($name:expr),* $(,)?) => {
        vec![$($crate::data::Profile::new($name)),*]
    };
}

#[macro_export]
macro_rules! task {
    ($($name:expr),* $(,)?) => {
        vec![
            $(
                $crate::data::Task::new($name)
            ),*
        ]
    };
}
