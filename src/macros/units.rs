/*
    Appellation: wrapper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! wrapper {
    (@impl $(#[derive($($d:ident),*)])? $vis:vis $name:ident($item:ty)) => {

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, $($($d),*)?)]
        $vis struct $name($item);
    };
    (@impl $vis:vis $name:ident($item:ty)) => {

        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
        $vis struct $name($vis $item);

        impl $name {
            $vis fn into_inner(self) -> $item {
                self.0
            }

            $vis fn as_ref(&self) -> &$item {
                &self.0
            }

            $vis fn as_mut(&mut self) -> &mut $item {
                &mut self.0
            }

            $vis fn map<F>(self, f: F) -> Self
            where
                F: FnOnce($item) -> $item,
            {
                Self(f(self.0))
            }

            $vis fn set(&mut self, value: $item) {
                self.0 = value;
            }
        }
    };
}
