/*
    Appellation: wrapper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! wrapper {
    (@decl $(#[derive($($meta:ident),*)])? $vis:vis $name:ident($item:ty)) => {

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, $($($meta),*)?)]
        #[repr(transparent)]
        $vis struct $name($vis $item);

        wrapper!(@core $name($item));
    };

    (@core $name:ident($item:ty)) => {
        impl ::core::convert::AsRef<$item> for $name {
            fn as_ref(&self) -> &$item {
                &self.0
            }
        }

        impl ::core::convert::AsMut<$item> for $name {
            fn as_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl ::core::borrow::Borrow<$item> for $name {
            fn borrow(&self) -> &$item {
                &self.0
            }
        }

        impl ::core::borrow::BorrowMut<$item> for $name {
            fn borrow_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $item;

            fn deref(&self) -> &$item {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $item {
                &mut self.0
            }
        }

        impl From<$item> for $name {
            fn from(data: $item) -> Self {
                Self(data)
            }
        }

        impl From<$name> for $item {
            fn from(item: $name) -> $item {
                item.get()
            }
        }
    };
    (@impl $name:ident($item:ty)) => {
        impl $name {
            pub fn from_value(value: $item) -> Self {
                Self(value)
            }

            pub fn get(self) -> $item {
                self.0
            }

            pub fn get_ref(&self) -> &$item {
                &self.0
            }

            pub fn get_mut(&mut self) -> &mut $item {
                &mut self.0
            }

            pub fn replace(&mut self, value: $item) -> $item {
                core::mem::replace(&mut self.0, value)
            }

            pub fn set(&mut self, value: $item) {
                self.0 = value;
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self.as_ref(), f)
            }
        }

        impl $crate::traits::Wrapper<$item> for $name {
            fn get(self) -> $item {
                self.0
            }

            fn get_ref(&self) -> &$item {
                &self.0
            }

            fn get_mut(&mut self) -> &mut $item {
                &mut self.0
            }

            fn replace(&mut self, value: $item) -> $item {
                core::mem::replace(&mut self.0, value)
            }

            fn set(&mut self, value: $item) {
                self.0 = value;
            }

            fn with(self, value: $item) -> Self {
                Self(value)
            }
        }
    };


    ($($name:ident($item:ty)),*) => {
        $(
            wrapper!(@decl pub $name($item));
            wrapper!(@impl $name($item));
        )*
    };
    (slim($($name:ident($item:ty)),*)) => {
        $(
            wrapper!(@decl pub $name($item));
        )*
    };
}

macro_rules! wrapper_t {
    (@decl $(#[derive($($meta:ident),*)])? $vis:vis $name:ident<$T:ident>) => {

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, $($($meta),*)?)]
        #[repr(transparent)]
        $vis struct $name<$T>($vis $T);
    };
    (@traits $name:ident<$T:ident>) => {
        impl<T> ::core::convert::AsRef<T> for $name<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> ::core::convert::AsMut<T> for $name<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T> ::core::borrow::Borrow<T> for $name<T> {
            fn borrow(&self) -> &T {
                &self.0
            }
        }

        impl<T> ::core::borrow::BorrowMut<T> for $name<T> {
            fn borrow_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T> ::core::ops::Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        impl<T> ::core::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T> From<T> for $name<T> {
            fn from(data: T) -> Self {
                Self(data)
            }
        }

        impl<T> From<$name<T>> for T {
            fn from(item: $name<T>) -> T {
                item.get()
            }
        }
    };
    (@impl $name:ident<$T:ident>) => {
        impl<$T> $name<$T> {
            pub fn from_value(value: $T) -> Self {
                Self(value)
            }

            pub fn get(self) -> $T {
                self.0
            }

            pub fn get_ref(&self) -> &$T {
                &self.0
            }

            pub fn get_mut(&mut self) -> &mut $T {
                &mut self.0
            }

            pub fn replace(&mut self, value: $T) -> $T {
                core::mem::replace(&mut self.0, value)
            }

            pub fn set(&mut self, value: $T) {
                self.0 = value;
            }
        }
    };
    (@fmt $name:ident<$T:ident>) => {
        impl<$T: ::core::fmt::Display> ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self.as_ref(), f)
            }
        }
    };

    ($($name:ident<$T:ident>),*) => {
        $(
            wrapper_t!(@decl pub $name<$T>);
            wrapper_t!(@traits $name<$T>);
            wrapper_t!(@impl $name<$T>);
            wrapper_t!(@fmt $name<$T>);
        )*
    };
}
