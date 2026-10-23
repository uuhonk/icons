//! Embedded SVG icon data from the uuhonk icon library.
//!
//! This crate contains only icon names and SVG strings. Tags and the preview
//! live in the parent icon library; use [`get`] or [`ICONS`] for Rust access.
#![forbid(unsafe_code)]

/// One embedded icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Icon {
    pub name: &'static str,
    pub svg: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

/// Looks up an icon by its stable name.
pub fn get(name: &str) -> Option<&'static Icon> {
    ICONS.iter().find(|icon| icon.name == name)
}

/// Iterates over all icon names.
pub fn names() -> impl Iterator<Item = &'static str> {
    ICONS.iter().map(|icon| icon.name)
}

#[cfg(test)]
mod tests {
    use super::{ICONS, get, names};

    #[test]
    fn has_icons_and_names_are_unique() {
        assert!(!ICONS.is_empty());
        let mut names: Vec<_> = names().collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ICONS.len());
        assert!(get("folder").is_some());
        assert!(get("post-processing").is_some());
    }

    #[test]
    fn every_svg_is_embedded() {
        for icon in ICONS {
            assert!(icon.svg.contains("<svg"));
        }
    }
}
