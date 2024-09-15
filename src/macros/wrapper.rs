/*
    Appellation: wrapper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

// pub trait Wrapper<T> {

//     fn get(self) -> T;

//     fn get_ref(&self) -> &T;

//     fn get_mut(&mut self) -> &mut T;

//     fn map<F>(self, f: F) -> Self
//     where
//         F: FnOnce(T) -> T;

//     fn replace(&mut self, value: T) -> T;

//     fn set(&mut self, value: T);

//     fn with(self, value: T) -> Self;
// }

macro_rules! wrapper {
    (@decl $(#[derive($($d:ident),*)])? $vis:vis $name:ident($item:ty)) => {

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, $($($d),*)?)]
        $vis struct $name($vis $item);

        wrapper!(@impl $name($item));
    };
    (@impl $name:ident($item:ty)) => {
        impl $name {
            pub fn from_value(value: $item) -> Self {
                Self(value)
            }

            #[inline]
            pub fn get(self) -> $item {
                self.0
            }

            pub const fn get_ref(&self) -> &$item {
                &self.0
            }

            pub fn get_mut(&mut self) -> &mut $item {
                &mut self.0
            }

            pub fn map<F>(self, f: F) -> Self
            where
                F: FnOnce($item) -> $item,
            {
                Self(f(self.0))
            }

            pub fn replace(&mut self, value: $item) -> $item {
                core::mem::replace(&mut self.0, value)
            }

            pub fn set(&mut self, value: $item) {
                self.0 = value;
            }

            pub fn with(self, value: $item) -> Self {
                Self(value)
            }
        }

        impl AsRef<$item> for $name {
            fn as_ref(&self) -> &$item {
                &self.0
            }
        }

        impl AsMut<$item> for $name {
            fn as_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl core::borrow::Borrow<$item> for $name {
            fn borrow(&self) -> &$item {
                &self.0
            }
        }

        impl core::borrow::BorrowMut<$item> for $name {
            fn borrow_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl core::ops::Deref for $name {
            type Target = $item;

            fn deref(&self) -> &$item {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl From<$item> for $name {
            fn from(item: $item) -> Self {
                Self(item)
            }
        }

        impl From<$name> for $item {
            fn from(wrapper: $name) -> $item {
                wrapper.0
            }
        }
    };

    ($($name:ident($item:ty)),*) => {
        $(
            wrapper!(@decl pub $name($item));
        )*
    };
}
