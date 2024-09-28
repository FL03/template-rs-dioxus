/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub fn rid() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
