/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

wrapper!(Timestamp(String), Title(String));

impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Local::now().format("%A, %B %e, %Y").to_string())
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}