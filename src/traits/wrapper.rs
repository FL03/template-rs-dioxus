/*
    Appellation: wrapper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Wrapper<T> {
    fn get(self) -> T;

    fn get_ref(&self) -> &T;

    fn get_mut(&mut self) -> &mut T;

    fn replace(&mut self, value: T) -> T;

    fn set(&mut self, value: T);

    fn with(self, value: T) -> Self;
}
