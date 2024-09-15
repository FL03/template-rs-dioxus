/*
    Appellation: name <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIs,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Name {
    Human(HumanName),
    Machine(String),
    Organization(String),
}

impl Default for Name {
    fn default() -> Self {
        Self::Machine(String::new())
    }
}

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
pub struct HumanName {
    pub prefix: Option<String>,
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
    pub suffix: Option<String>,
}

impl HumanName {
    pub fn new(first: impl ToString, last: impl ToString) -> Self {
        Self {
            prefix: None,
            first: first.to_string(),
            middle: None,
            last: last.to_string(),
            suffix: None,
        }
    }

    pub fn with_prefix(self, prefix: impl ToString) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
            ..self
        }
    }

    pub fn with_middle(self, middle: impl ToString) -> Self {
        Self {
            middle: Some(middle.to_string()),
            ..self
        }
    }

    pub fn with_suffix(self, suffix: impl ToString) -> Self {
        Self {
            suffix: Some(suffix.to_string()),
            ..self
        }
    }

    pub fn first(&self) -> &str {
        &self.first
    }

    pub fn last(&self) -> &str {
        &self.last
    }

    pub fn full_name(&self) -> String {
        let mut name = String::new();
        if let Some(prefix) = &self.prefix {
            name.push_str(prefix);
            name.push(' ');
        }
        name.push_str(&self.first);
        name.push(' ');
        if let Some(middle) = &self.middle {
            name.push_str(middle);
            name.push(' ');
        }
        name.push_str(&self.last);
        if let Some(suffix) = &self.suffix {
            name.push(' ');
            name.push_str(suffix);
        }
        name
    }
}

impl core::str::FromStr for HumanName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Name must have at least a first and last name".to_string());
        }
        let n = parts.len();
        if n == 1 {
            Ok(HumanName {
                first: parts[0].to_string(),
                last: String::new(),
                ..Default::default()
            })
        } else if n == 2 {
            Ok(HumanName {
                first: parts[0].to_string(),
                last: parts[1].to_string(),
                ..Default::default()
            })
        } else if n == 3 {
            Ok(HumanName {
                first: parts[0].to_string(),
                middle: Some(parts[1].to_string()),
                last: parts[2].to_string(),
                ..Default::default()
            })
        } else if n == 4 {
            Ok(HumanName {
                prefix: Some(parts[0].to_string()),
                first: parts[1].to_string(),
                middle: Some(parts[2].to_string()),
                last: parts[3].to_string(),
                ..Default::default()
            })
        } else if n == 5 {
            Ok(HumanName {
                prefix: Some(parts[0].to_string()),
                first: parts[1].to_string(),
                middle: Some(parts[2].to_string()),
                last: parts[3].to_string(),
                suffix: Some(parts[4].to_string()),
            })
        } else {
            Err("Name must have at most a prefix, first, middle, last, and suffix".to_string())
        }
    }
}
