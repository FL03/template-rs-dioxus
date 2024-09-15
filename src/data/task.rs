/*
    Appellation: task <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Id;
use crate::types::Timestamp;

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
    pub(crate) assignees: Vec<Id>,
    pub(crate) completed: bool,
    pub(crate) description: String,
    pub(crate) name: String,
    pub(crate) notes: Vec<String>,
    pub(crate) priority: u8,
    pub(crate) tags: Vec<String>,
    pub(crate) completed_at: Option<Timestamp>,
    pub(crate) created_at: Timestamp,
    pub(crate) updated_at: Timestamp,
}

impl Task {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            assignees: Vec::new(),
            completed: false,
            description: String::new(),
            name: name.to_string(),
            notes: Vec::new(),
            priority: 0,
            tags: Vec::new(),
            completed_at: None,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
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
        self.on_update();
        self.completed = true;
        self.completed_at = Some(Timestamp::now());
    }

    pub fn uncomplete(&mut self) {
        self.on_update();
        self.completed = false;
        self.completed_at = None;
    }

    pub fn describe(&mut self, description: impl ToString) {
        self.on_update();
        self.description = description.to_string();
    }

    fn on_update(&mut self) {
        self.updated_at = Timestamp::now();
    }
}
