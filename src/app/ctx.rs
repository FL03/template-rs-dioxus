/*
    Appellation: ctx <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use crate::data::{Profile, Task};
use crate::types::ThemeMode;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Context {
    pub current_date: String,
    pub employees: Vec<Profile>,
    pub tasks: Vec<Task>,
    pub theme_mode: ThemeMode,
    pub settings: Settings,
}

impl Context {
    pub fn from_config(settings: Settings) -> Self {
        Self {
            settings,
            ..Self::default()
        }
    }
    /// Returns the current date.
    pub fn current_date(&self) -> &str {
        &self.current_date
    }

    pub fn employees(&self) -> &[Profile] {
        &self.employees
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn theme_mode(&self) -> ThemeMode {
        self.theme_mode
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            current_date: chrono::Local::now().format("%A, %B %e, %Y").to_string(),
            employees: crate::SAMPLE_PROFILES.clone(),
            settings: Settings::default(),
            tasks: crate::SAMPLE_TASKS.clone(),
            theme_mode: ThemeMode::default(),
        }
    }
}

impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
unsafe impl Send for Context {}

unsafe impl Sync for Context {}
