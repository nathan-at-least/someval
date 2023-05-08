//! Groups of optional values where at least one value is present
//!
//!
//! # Motivation
//!
//! Sometimes applications need to ensure at least one value is present, while also allowing
//! multiple values of distinct types. A common pattern in this case, is to track multiple `Option`
//! values and require at least one contains a value:
//!
//! ```
//! /// Represent distinct optional values for a couple of types:
//! struct NameId {
//!     id: Option<u64>,
//!     name: Option<String>,
//! }
//!
//! impl NameId {
//!     fn ensure_value_is_present(&self) -> Result<(), &'static str> {
//!         if self.id.is_some() || self.name.is_some() {
//!             Ok(())
//!         } else {
//!             Err("either an id or a name or both must be present")
//!         }
//!     }
//! }
//!
//! let good_value = NameId { id: Some(42), name: None };
//! assert!(good_value.ensure_value_is_present().is_ok());
//!
//! let bad_value = NameId { id: None, name: None };
//! assert!(bad_value.ensure_value_is_present().is_err());
//! ```
//!
//! # Overview
//!
//! This crate provides types to represent this pattern:
//!
//! Sometimes applications need to represent 1 or more values which may have distinct types. This
//! is where `someval` comes in to "make illegal states unrepresentable" for this pattern.
//!
//! A "someval" is a type similar to a set of `Option` values except it guarantees at type-checking
//! time that _at least one_ value is always present at runtime.
//!
//! ## The "someval" types
//!
//! The "someval" types in this crate follow common structure, varying in the number of generics
//! they support. They are all named `SomeN` with the suffix `N` indicating how many distinct values
//! are possible, e.g. [Some2], [Some3], etc… So [Some2] is generic over two types: `Some2<A, B>`,
//! while [Some3] is generic over three: `Some3<A, B, C>`, etc…
//!
//! Each "someval" type is an enum. Both type parameters and enum variants use the uppercase
//! English alphabet as placeholders, e.g.:
//!
//! ```
//! use someval::Some2;
//!
//! type NameId = Some2<u64, String>;
//!
//! let nid = NameId::A(42);
//! ```
//!
//! There is an enum variant for every combination of values present:
//!
//! ```
//! use someval::Some3;
//!
//! type Triple = Some3<i64, &'static str, bool>;
//!
//! let val = Triple::AC(42, false);
//! ```
//!
//! ## Constructing "somevals"
//!
//! Values can be constructed with the enum variants:
//!
//! ```
//! use someval::Some2;
//!
//! type NameId = Some2<u64, String>;
//!
//! let nid1 = NameId::A(42);
//! let nid2 = NameId::B("Alice".to_string());
//! let nid3 = NameId::AB(13, "Bob".to_string());
//! ```
//!
//! A value with all types present can be converted from a tuple of values via `From`:
//!
//! ```
//! # use someval::Some2;
//! # type NameId = Some2<u64, String>;
//! # let nid3 = NameId::AB(13, "Bob".to_string());
//! let nid4 = NameId::from((13, "Bob".to_string()));
//! assert_eq!(nid3, nid4);
//! ```
//!
//! A value can by fallibly converted from a tuple of `Option` values via `TryFrom`:
//!
//! ```
//! # use someval::Some2;
//! # type NameId = Some2<u64, String>;
//! let res1 = NameId::try_from((Some(42), None));
//! assert!(res1.is_ok());
//! let res2 = NameId::try_from((None, None));
//! assert!(res2.is_err());
//! ```
//!
//! A "someval" like [Some2] can always be converted to a tuple of `Option` values:
//!
//! ```
//! # use someval::Some2;
//!
//! type NameId = Some2<u64, String>;
//!
//! let nid = NameId::A(42);
//! let (optid, optname) = nid.into();
//! assert_eq!(optid, Some(42));
//! assert_eq!(optname, None);
//! ```
//!
//! Most methods on a "someval" use a copy/move receiver and each provides an `as_ref` method to
//! convert to references (similar to [Option::as_ref] and [Result::as_ref], not to be confused
//! with [AsRef::as_ref]):
//!
//! ```
//! # use someval::Some2;
//!
//! type NameId = Some2<u64, String>;
//!
//! let nid = NameId::A(42);
//! let idref: &u64 = match nid.as_ref() {
//!     Some2::A(x) => x,
//!     _ => panic!(),
//! };
//! assert_eq!(*idref, 42);
//! ```
//!
//! Individual accessor methods give an `Option` for components (similar to [Result::ok] and
//! [Result::err]):
//!
//! ```
//! # use someval::Some2;
//!
//! type NameId = Some2<u64, String>;
//!
//! let nid = NameId::A(42);
//! assert_eq!(nid.as_ref().a(), Some(&42));
//! assert_eq!(nid.as_ref().b(), None);
//! ```
mod some2;
mod some3;

pub use self::some2::Some2;
pub use self::some3::Some3;
