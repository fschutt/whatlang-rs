//! Whatlang is a Rust library to detect(regonize) natural languages.
//! Apart from it, the library also recognizes scripts (writing system).
//! Every language and script are represented by determined list of enums.
//!
//! # Examples
//!
//! Using `detect` function:
//!
//! ```
//! use whatlang::{detect, Lang, Script};
//!
//! let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
//! let info = detect(text).unwrap();
//! assert_eq!(info.lang(), Lang::Epo);
//! assert_eq!(info.script(), Script::Latin);
//!
//! // TODO:
//! // Confidence is in the range from 0 to 1.
//! // assert_eq!(info.confidence(), 1.0);
//! // assert!(info.is_reliable());
//! ```
//!
//! Using `Detector` with specified blacklist or whitelist:
//!
//! ```
//! // TODO:
//! // use whatlang::{Detector, Lang};
//!
//! // let whitelist = vec![Lang::Eng, Lang::Rus];
//!
//! // // You can also create detector using with_blacklist function
//! // let detector = Detector::with_whitelist(whitelist);
//! // let lang = detector.detect_lang("There is no reason not to learn Esperanto.");
//! // assert_eq!(lang, Some(Lang::Eng));
//!
// mod detect;
// mod detector;
mod error;
// mod info;
mod lang;
mod scripts;
mod trigrams;
mod utils;
mod core;
mod alphabets;
mod combined;

// pub use crate::detector::Detector;
// pub use crate::info::Info;
pub use crate::lang::Lang;
pub use crate::scripts::{detect_script, Script};
pub use crate::core::{detect, detect_lang, detect_with_options, Output, Method, Options};
