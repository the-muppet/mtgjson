pub mod monolith;
pub mod orientation_detector;
pub mod set_language_detector;
pub mod utils;

pub use utils::{MtgjsonConfig, build_http_header};
pub use monolith::ScryfallProvider;
pub use orientation_detector::ScryfallProviderOrientationDetector;
pub use set_language_detector::ScryfallProviderSetLanguageDetector;