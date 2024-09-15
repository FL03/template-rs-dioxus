/*
    Appellation: model <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! model {

    (@decl $(#[$meta:meta])* $vis:vis struct $name:ident $($fields:tt),* $(,)?) => {
        $(#[$meta])*
        $vis struct $name $($fields)*

        model!(@impl $name($item) { $($fields),* });
        builder!($(#[$meta])* paste::paste![<$name Builder>]<$item> { $($fields: $ty),* });
    };
    (@impl $name:ident { $($vis:vis $field:ident: $ty:ty),* $(,)? }) => {
        impl $name {
            $(
                paste::paste! {
                    $vis const fn $field(&self) -> &$ty {
                        &self.$field
                    }

                    $vis fn [< $field _mut >](&mut self) -> &mut $ty {
                        &mut self.$field
                    }
                }
            )*
        }
    };
}

macro_rules! builder {
    ($(#[$meta:meta])* $name:ident<$out:ident> { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$meta:meta])*
        #[derive(Default)]
        pub struct $name {
            $(pub(crate) $field: Option<$ty>),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($field: None),*
                }
            }
            $(
                pub fn $field(self, data: $ty) -> Self {
                    Self {
                        $field: Some(data),
                        ..self
                    }
                }
            )*

            pub fn is_complete(&self) -> bool {
                $(
                    self.$field.is_some()
                )&&*
            }

            pub fn build(self) -> Option<$out> {
                Some($out { $($field: self.$field?),*};)
            }

        }
    };
}
