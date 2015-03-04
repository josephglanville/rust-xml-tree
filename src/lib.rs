#![feature(old_io, alloc)]

extern crate xml;

pub use builder::build;
pub use dom::{Document, RcNode, RcElement};

pub use xml::EventReader;
pub use xml::attribute::OwnedAttribute;
pub use xml::common::Error as ParserError;
pub use xml::common::XmlVersion;
pub use xml::name::OwnedName;
pub use xml::namespace::Namespace;
pub use xml::reader::config::ParserConfig;
pub use xml::reader::events::XmlEvent;

mod builder;
mod dom;
mod error;
