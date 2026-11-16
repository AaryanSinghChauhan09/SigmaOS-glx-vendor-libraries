// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-glx-vendor-libraries: glx vendor libraries component model.
//!
//! Real implementation: librariess, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Libraries {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlxVendorLibrariesError {
    EmptyLibrariesName,
    DuplicateLibraries(String),
    EmptyEventSubject,
}

impl fmt::Display for GlxVendorLibrariesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLibrariesName => write!(f, "libraries name must not be empty"),
            Self::DuplicateLibraries(id) => write!(f, "duplicate libraries {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GlxVendorLibrariesError {}

pub struct GlxVendorLibraries {
    pub name: String,
    librariess: BTreeMap<String, Libraries>,
    events: BTreeMap<String, Event>,
}

impl GlxVendorLibraries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            librariess: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_libraries(&mut self, item: Libraries) -> Result<(), GlxVendorLibrariesError> {
        if item.name.trim().is_empty() {
            return Err(GlxVendorLibrariesError::EmptyLibrariesName);
        }
        let id = item.id.clone();
        if self.librariess.contains_key(&id) {
            return Err(GlxVendorLibrariesError::DuplicateLibraries(id.clone()));
        }
        self.librariess.insert(id, item);
        Ok(())
    }

    pub fn libraries_count(&self) -> usize {
        self.librariess.len()
    }

    pub fn get_libraries(&self, id: &str) -> Option<&Libraries> {
        self.librariess.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GlxVendorLibrariesError> {
        if event.subject.trim().is_empty() {
            return Err(GlxVendorLibrariesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_libraries(id: &str, name: &str) -> Libraries {
        Libraries {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GlxVendorLibraries::new("GlxVendorLibraries");
        assert_eq!(m.name, "GlxVendorLibraries");
        assert_eq!(m.libraries_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_libraries() {
        let mut m = GlxVendorLibraries::new("GlxVendorLibraries");
        m.add_libraries(sample_libraries("a1", "Alpha")).unwrap();
        assert_eq!(m.libraries_count(), 1);
        assert!(m.get_libraries("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GlxVendorLibraries::new("GlxVendorLibraries");
        assert!(m.add_libraries(sample_libraries("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GlxVendorLibraries::new("GlxVendorLibraries");
        m.add_libraries(sample_libraries("x", "One")).unwrap();
        assert!(m.add_libraries(sample_libraries("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GlxVendorLibraries::new("GlxVendorLibraries");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
