//! The words DSA shows or says in its own interface that every host shares, one table for
//! each part, in each of the languages the interface is shown in ([`crate::ui_language`]).
//!
//! A host keeps only the words its operating system owns (a menu item the desktop names
//! itself, a shortcut key written the way that system writes it) and takes the rest from
//! here, so a translation is corrected in one place.

pub mod caption;
pub mod root_approval;
pub mod window_menu;
