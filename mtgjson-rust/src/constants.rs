/// MTGJSON Constants that cannot be changed and are hardcoded intentionally

use std::collections::HashSet;
use std::path::PathBuf;
use once_cell::sync::Lazy;

/// MTGJSON Version
pub const MTGJSON_VERSION: &str = "5.2.0";

/// MTGJSON Build Date
pub const MTGJSON_BUILD_DATE: &str = env!("CARGO_PKG_VERSION");

/// Supported format outputs for compiled files
pub const SUPPORTED_FORMAT_OUTPUTS: &[&str] = &[
    "standard",
    "pioneer", 
    "modern",
    "legacy",
    "vintage",
    "pauper",
    "commander",
    "historic",
    "alchemy",
    "explorer",
    "brawl",
    "future",
    "timeless",
];

/// Supported set types for normal sets
pub const SUPPORTED_SET_TYPES: &[&str] = &[
    "core",
    "expansion", 
    "draft_innovation",
    "masters",
    "commander",
    "planechase",
    "archenemy",
    "vanguard",
    "from_the_vault",
    "premium_deck",
    "duel_deck",
    "starter",
    "box",
    "promo",
    "token",
    "memorabilia",
    "treasure_chest",
    "spellbook",
    "arsenal",
    "funny",
    "un",
    "minigame",
];

/// Basic land names
pub const BASIC_LAND_NAMES: &[&str] = &[
    "Plains",
    "Island", 
    "Swamp",
    "Mountain",
    "Forest",
    "Wastes",
];

/// Multi-word sub types that need special handling
pub const MULTI_WORD_SUB_TYPES: &[&str] = &[
    "Aura Curse",
    "Legendary Creature",
    "Snow Land",
    "Basic Land",
    "Artifact Creature",
    "Enchantment Creature",
    "Tribal Instant",
    "Tribal Sorcery",
    "Tribal Enchantment",
    "Tribal Artifact",
];

/// Magic supertypes
pub const SUPER_TYPES: &[&str] = &[
    "Basic",
    "Legendary",
    "Ongoing",
    "Snow",
    "World",
    "Elite",
    "Host",
];

/// Language mapping from Scryfall codes to MTGJSON language names
pub static LANGUAGE_MAP: Lazy<std::collections::HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("en", "English");
    map.insert("es", "Spanish");
    map.insert("fr", "French");
    map.insert("de", "German");
    map.insert("it", "Italian");
    map.insert("pt", "Portuguese (Brazil)");
    map.insert("ja", "Japanese");
    map.insert("ko", "Korean");
    map.insert("ru", "Russian");
    map.insert("zhs", "Chinese Simplified");
    map.insert("zht", "Chinese Traditional");
    map.insert("he", "Hebrew");
    map.insert("la", "Latin");
    map.insert("grc", "Ancient Greek");
    map.insert("ar", "Arabic");
    map.insert("sa", "Sanskrit");
    map.insert("ph", "Phyrexian");
    map
});

/// Hash algorithm to use for file integrity
pub enum HashToGenerate {
    Sha256,
}

impl HashToGenerate {
    pub fn name(&self) -> &'static str {
        match self {
            HashToGenerate::Sha256 => "sha256",
        }
    }
}

pub const HASH_TO_GENERATE: HashToGenerate = HashToGenerate::Sha256;

/// Default paths
pub static CACHE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("MTGJSON_CACHE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("cache"))
});

pub static OUTPUT_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("MTGJSON_OUTPUT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("output"))
});

pub static RESOURCE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("MTGJSON_RESOURCE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("mtgjson5/resources"))
});

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("mtgjson.properties")
});

/// Provider rate limits (requests per second)
pub const SCRYFALL_RATE_LIMIT: f64 = 10.0;
pub const TCGPLAYER_RATE_LIMIT: f64 = 5.0;
pub const CARDMARKET_RATE_LIMIT: f64 = 1.0;
pub const CARDKINGDOM_RATE_LIMIT: f64 = 2.0;

/// Maximum retries for failed requests
pub const MAX_RETRIES: u32 = 3;

/// Request timeout in seconds
pub const REQUEST_TIMEOUT: u64 = 30;

/// Maximum concurrent requests
pub const MAX_CONCURRENT_REQUESTS: usize = 10;

/// Default compression level for archives
pub const COMPRESSION_LEVEL: u32 = 6;

/// File size thresholds
pub const LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100MB
pub const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024 * 1024; // 2GB

/// Card face identifiers for split/double-faced cards
pub const FRONT_FACE_ID: u8 = 0;
pub const BACK_FACE_ID: u8 = 1;

/// Price provider identifiers
pub const PRICE_PROVIDERS: &[&str] = &[
    "cardkingdom",
    "cardmarket", 
    "tcgplayer",
    "cardhoarder",
    "mtgban",
];

/// Booster pack types
pub const BOOSTER_TYPES: &[&str] = &[
    "default",
    "draft",
    "set",
    "theme",
    "tournament",
    "arena",
];

/// Frame effects
pub const FRAME_EFFECTS: &[&str] = &[
    "legendary",
    "miracle",
    "nyxtouched",
    "draft",
    "devoid",
    "tombstone",
    "colorshifted",
    "sunmoondfc",
    "compasslanddfc",
    "originpwdfc",
    "mooneldrazidfc",
    "waxingandwaningmoondfc",
    "showcase",
    "extendedart",
    "companion",
    "etched",
    "snow",
    "lesson",
    "shatteredglass",
    "convertdfc",
    "fandfc",
    "upsidedowndfc",
];

/// Border colors
pub const BORDER_COLORS: &[&str] = &[
    "black",
    "white", 
    "borderless",
    "silver",
    "gold",
];

/// Rarities
pub const RARITIES: &[&str] = &[
    "common",
    "uncommon",
    "rare", 
    "mythic",
    "special",
    "bonus",
];

/// Finishes
pub const FINISHES: &[&str] = &[
    "nonfoil",
    "foil",
    "etched",
    "glossy",
];

/// Security stamps
pub const SECURITY_STAMPS: &[&str] = &[
    "oval",
    "triangle",
    "acorn",
    "circle",
    "arena",
    "heart",
];

/// Watermarks (common ones)
pub const COMMON_WATERMARKS: &[&str] = &[
    "abzan",
    "agentsofsneak",
    "arena",
    "atarka",
    "azorius",
    "boros",
    "colorpie",
    "conspiracy",
    "dci",
    "dimir",
    "dromoka",
    "fnm",
    "golgari",
    "gruul",
    "izzet",
    "jeskai",
    "junior",
    "kolaghan",
    "mardu",
    "mirran",
    "mtg",
    "nerf",
    "ojutai",
    "orzhov",
    "phyrexian",
    "planeswalker",
    "promo",
    "rakdos",
    "scholarship",
    "selesnya",
    "set",
    "silumgar",
    "simic",
    "sultai",
    "temur",
    "wotc",
];

/// Promo types
pub const PROMO_TYPES: &[&str] = &[
    "alchemy",
    "arena",
    "boosterfun",
    "boxtopper",
    "brawldeck",
    "bundle",
    "buyabox",
    "convention",
    "datestamped",
    "drafter",
    "draftweekend",
    "duels",
    "event",
    "fnm",
    "gameday",
    "gateway",
    "gift",
    "giftbox",
    "glossy",
    "godzillaseries",
    "intropack",
    "jpwalker",
    "judgegift",
    "league",
    "mediainsert",
    "openhouse",
    "planeswalkerdeck",
    "playerrewards",
    "playpromo",
    "premiereshop",
    "prerelease",
    "promopack",
    "publicevents",
    "release",
    "rebalanced",
    "scholarship",
    "serialized",
    "setpromo",
    "stamped",
    "starterdeck",
    "storechampionship",
    "surgefoil",
    "textless",
    "thick",
    "tournament",
    "wizardsplaynetwork",
];

/// Card layouts
pub const CARD_LAYOUTS: &[&str] = &[
    "normal",
    "split",
    "flip",
    "transform",
    "modal_dfc",
    "meld",
    "leveler",
    "saga",
    "adventure",
    "planar",
    "scheme", 
    "vanguard",
    "token",
    "double_faced_token",
    "emblem",
    "augment",
    "host",
    "art_series",
    "reversible_card",
    "class",
    "prototype",
    "battle",
    "mutate",
    "case",
];

/// Color identity mapping
pub const COLOR_IDENTITY_MAP: &[(&str, &str)] = &[
    ("W", "White"),
    ("U", "Blue"),
    ("B", "Black"),
    ("R", "Red"),
    ("G", "Green"),
];

/// Mana symbols
pub const MANA_SYMBOLS: &[&str] = &[
    "W", "U", "B", "R", "G", "C",
    "X", "Y", "Z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", 
    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
    "W/U", "W/B", "U/B", "U/R", "B/R", "B/G", "R/G", "R/W", "G/W", "G/U",
    "2/W", "2/U", "2/B", "2/R", "2/G",
    "W/P", "U/P", "B/P", "R/P", "G/P",
    "HW", "HR",
    "∞",
];

/// API endpoints and URLs
pub const SCRYFALL_API_BASE: &str = "https://api.scryfall.com";
pub const SCRYFALL_SETS_URL: &str = "https://api.scryfall.com/sets/";
pub const SCRYFALL_CARDS_URL: &str = "https://api.scryfall.com/cards/";
pub const SCRYFALL_BULK_DATA_URL: &str = "https://api.scryfall.com/bulk-data";

pub const TCGPLAYER_API_BASE: &str = "https://api.tcgplayer.com";
pub const CARDMARKET_API_BASE: &str = "https://api.cardmarket.com";
pub const CARDKINGDOM_API_BASE: &str = "https://api.cardkingdom.com";

/// User agent for HTTP requests
pub const USER_AGENT: &str = "MTGJSON Rust/5.2.0 (https://mtgjson.com/)";

/// Thread pool configuration
pub const MIN_THREADS: usize = 2;
pub const MAX_THREADS: usize = 16;

/// Cache settings
pub const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour
pub const MAX_CACHE_SIZE: usize = 1000;

/// Validation constants
pub const MIN_SET_CODE_LENGTH: usize = 3;
pub const MAX_SET_CODE_LENGTH: usize = 6;
pub const MIN_CARD_NAME_LENGTH: usize = 1;
pub const MAX_CARD_NAME_LENGTH: usize = 200;

/// File extensions
pub const JSON_EXTENSION: &str = ".json";
pub const XZ_EXTENSION: &str = ".xz";
pub const GZ_EXTENSION: &str = ".gz";
pub const ZIP_EXTENSION: &str = ".zip";

/// Date formats
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const ISO_DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// Error messages
pub const ERROR_SET_NOT_FOUND: &str = "Set not found";
pub const ERROR_CARD_NOT_FOUND: &str = "Card not found";
pub const ERROR_INVALID_SET_CODE: &str = "Invalid set code";
pub const ERROR_NETWORK_TIMEOUT: &str = "Network request timeout";
pub const ERROR_RATE_LIMITED: &str = "Rate limit exceeded";
pub const ERROR_INVALID_JSON: &str = "Invalid JSON data";
pub const ERROR_FILE_NOT_FOUND: &str = "File not found";
pub const ERROR_PERMISSION_DENIED: &str = "Permission denied";

/// Default configuration values
pub const DEFAULT_OUTPUT_PRETTY: bool = false;
pub const DEFAULT_COMPRESSION_ENABLED: bool = true;
pub const DEFAULT_PARALLEL_PROCESSING: bool = true;
pub const DEFAULT_CACHE_ENABLED: bool = true;
pub const DEFAULT_RETRY_ENABLED: bool = true;

/// Memory limits
pub const MAX_JSON_SIZE: usize = 500 * 1024 * 1024; // 500MB
pub const MAX_STRING_LENGTH: usize = 10 * 1024; // 10KB
pub const MAX_ARRAY_LENGTH: usize = 100_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_defined() {
        assert!(!SUPPORTED_FORMAT_OUTPUTS.is_empty());
        assert!(!SUPPORTED_SET_TYPES.is_empty());
        assert!(!BASIC_LAND_NAMES.is_empty());
        assert!(!LANGUAGE_MAP.is_empty());
        assert_eq!(MTGJSON_VERSION, "5.2.0");
    }

    #[test]
    fn test_paths() {
        assert!(CACHE_PATH.is_relative() || CACHE_PATH.is_absolute());
        assert!(OUTPUT_PATH.is_relative() || OUTPUT_PATH.is_absolute());
        assert!(RESOURCE_PATH.is_relative() || RESOURCE_PATH.is_absolute());
        assert!(CONFIG_PATH.is_relative() || CONFIG_PATH.is_absolute());
    }

    #[test]
    fn test_language_map() {
        assert_eq!(LANGUAGE_MAP.get("en"), Some(&"English"));
        assert_eq!(LANGUAGE_MAP.get("ja"), Some(&"Japanese"));
        assert_eq!(LANGUAGE_MAP.get("zhs"), Some(&"Chinese Simplified"));
    }

    #[test]
    fn test_supported_formats() {
        assert!(SUPPORTED_FORMAT_OUTPUTS.contains(&"standard"));
        assert!(SUPPORTED_FORMAT_OUTPUTS.contains(&"modern"));
        assert!(SUPPORTED_FORMAT_OUTPUTS.contains(&"legacy"));
        assert!(SUPPORTED_FORMAT_OUTPUTS.contains(&"vintage"));
    }

    #[test]
    fn test_rate_limits() {
        assert!(SCRYFALL_RATE_LIMIT > 0.0);
        assert!(TCGPLAYER_RATE_LIMIT > 0.0);
        assert!(CARDMARKET_RATE_LIMIT > 0.0);
        assert!(CARDKINGDOM_RATE_LIMIT > 0.0);
    }

    #[test]
    fn test_urls() {
        assert!(SCRYFALL_API_BASE.starts_with("https://"));
        assert!(TCGPLAYER_API_BASE.starts_with("https://"));
        assert!(CARDMARKET_API_BASE.starts_with("https://"));
        assert!(CARDKINGDOM_API_BASE.starts_with("https://"));
    }
}