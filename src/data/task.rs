/*
    Appellation: task <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Id;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Task {
    pub(crate) id: Id,
    pub(crate) completed: bool,
    pub(crate) description: String,
    pub(crate) label: String,
    pub(crate) assignees: Vec<Id>,
}

impl Task {
    pub fn new(label: impl ToString, description: impl ToString) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            assignees: Vec::new(),
            completed: false,
            description: description.to_string(),
            label: label.to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn assignees(&self) -> &[Id] {
        &self.assignees
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn assign_to(&mut self, id: impl ToString) {
        self.assignees.push(id.to_string());
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
