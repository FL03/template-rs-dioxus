/*
    Appellation: create <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_export]
macro_rules! new {
    (@impl $new:ty { $name:expr }) => {
        $new::new($name)
    };
    ($($new:ty { $name:expr }),* $(,)?) => {
        $($crate::new(@impl $new { $name })),*
    };
}
