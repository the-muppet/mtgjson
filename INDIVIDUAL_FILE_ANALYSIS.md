# MTGJSON Rust - Individual File Analysis

This document provides detailed analysis of individual Rust files and their PyO3 bindings, including complete method signatures, properties, and functionality documentation.

---

## Core Data Structure Files

### legalities.rs - MtgjsonLegalities

**Status:** ⚠️ **FIELD TYPE COMPATIBILITY ISSUE**

```python
class MtgjsonLegalities:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def get_legal_formats(self) -> list[str]: ...
    def to_dict(self) -> dict[str, str]: ...
    
    # Properties (all have get/set)
    # ⚠️ WARNING: Python expects str, Rust provides Option<String>
    brawl: str | None
    commander: str | None  
    duel: str | None
    future: str | None
    frontier: str | None
    legacy: str | None
    modern: str | None
    pauper: str | None
    penny: str | None
    pioneer: str | None
    standard: str | None
    vintage: str | None
```

**Critical Issues:**
- **Missing formats**: `alchemy`, `explorer`, `gladiator`, `historic`, `oldschool`, `paupercommander`, `predh`, `premodern`, `timeless` (mentioned in Python API reference)
- **Type incompatibility**: Python expects required `str` fields, Rust provides `Option<String>`
- **Partial implementation**: Only 12 of ~20 expected format fields

**Methods Analysis:**
- `get_legal_formats()`: Returns list of formats where status is "Legal"
- `to_dict()`: Converts to HashMap filtering out None values
- `to_json()`: Standard JSON serialization

---

### leadership_skills.rs - MtgjsonLeadershipSkills

**Status:** ✅ **WORKING CORRECTLY**

```python
class MtgjsonLeadershipSkills:
    def __init__(self, brawl: bool, commander: bool, oathbreaker: bool) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def has_any_skills(self) -> bool: ...
    def get_available_skills(self) -> list[str]: ...
    
    # Properties (all have get/set)
    brawl: bool
    commander: bool  
    oathbreaker: bool
```

**Analysis:**
- **Simple and complete**: All three leadership skills properly represented
- **Type safety**: Uses bool fields matching expected behavior
- **Utility methods**: Provides both check and enumeration methods
- **No compatibility issues**: Should work seamlessly with Python code

---

### game_formats.rs - MtgjsonGameFormats

**Status:** ✅ **WORKING CORRECTLY**

```python
class MtgjsonGameFormats:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json(self) -> list[str]: ...  # Note: Returns list, not JSON string
    def get_available_formats(self) -> list[str]: ...
    
    # Properties (all have get/set)
    paper: bool
    mtgo: bool
    arena: bool
    shandalar: bool
    dreamcast: bool
```

**Analysis:**
- **Complete format coverage**: All major game formats represented
- **Efficient representation**: Boolean fields for each format
- **Utility methods**: Provides list conversion and format checking
- **From trait**: Implements `From<&[&str]>` for easy construction
- **Note**: `to_json()` returns `Vec<String>` not JSON string (unusual but functional)

---

### foreign_data.rs - MtgjsonForeignData

**Status:** ✅ **WORKING WITH MINOR ISSUES**

```python
class MtgjsonForeignData:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json_string(self) -> str: ...
    def to_dict(self) -> str: ...  # Note: Returns JSON string, not dict
    def has_content(self) -> bool: ...
    def get_display_name(self) -> str | None: ...
    
    # Properties (all have get/set)
    language: str
    multiverse_id: int | None  # Deprecated - Remove in 5.4.0
    identifiers: MtgjsonIdentifiers
    face_name: str | None
    flavor_text: str | None
    name: str | None
    text: str | None
    type_: str | None
```

**Analysis:**
- **Comprehensive fields**: All major foreign data fields included
- **Deprecation handling**: `multiverse_id` marked for removal
- **Embedded objects**: Includes `MtgjsonIdentifiers` for foreign identifiers
- **Utility methods**: Content validation and display name resolution
- **Method naming issue**: `to_dict()` returns JSON string, not Python dict

---

### deck.rs - MtgjsonDeck & MtgjsonDeckHeader

**Status:** ✅ **WORKING CORRECTLY**

```python
class MtgjsonDeck:
    def __init__(self, deck_name: str = "", sealed_product_uuids: list[str] | None = None) -> None: ...
    
    # Core Methods
    def to_json(self) -> str: ...
    def set_sanitized_name(self, name: str) -> None: ...
    def add_sealed_product_uuids(self, mtgjson_set_sealed_products: list[MtgjsonSealedProduct]) -> None: ...
    def populate_deck_from_api(self, mtgjson_deck_header: MtgjsonDeckHeader, mtgjson_set_sealed_products: list[MtgjsonSealedProduct]) -> None: ...
    
    # Card Management
    def add_main_board_card(self, card_json: str) -> None: ...
    def add_side_board_card(self, card_json: str) -> None: ...
    def add_commander_card(self, card_json: str) -> None: ...
    def get_total_cards(self) -> int: ...
    def get_main_board_count(self) -> int: ...
    def get_side_board_count(self) -> int: ...
    def has_cards(self) -> bool: ...
    def clear_all_cards(self) -> None: ...
    
    # Properties (all have get/set)
    main_board: list[str]  # JSON strings, not card objects
    side_board: list[str]
    display_commander: list[str]
    commander: list[str]
    planes: list[str]
    schemes: list[str]
    code: str
    name: str
    release_date: str
    sealed_product_uuids: list[str] | None
    type_: str
    file_name: str  # Internal field

class MtgjsonDeckHeader:
    def __init__(self, output_deck: MtgjsonDeck) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def from_deck_data(code: str, name: str, release_date: str, type_: str, file_name: str) -> MtgjsonDeckHeader: ...  # @staticmethod
    def get_display_info(self) -> dict[str, str]: ...
    
    # Properties (all have get/set)
    code: str
    file_name: str
    name: str
    release_date: str
    type_: str
```

**Analysis:**
- **Complete deck structure**: All major deck components represented
- **Card storage**: Uses JSON strings for cards (memory efficient)
- **Sealed product integration**: Links to sealed products via UUIDs
- **Filename sanitization**: Built-in safe filename generation
- **Commander support**: Full support for commander variants
- **Deck header**: Separate header class for metadata management

---

### translations.rs - MtgjsonTranslations

**Status:** ✅ **WORKING CORRECTLY**

```python
class MtgjsonTranslations:
    def __init__(self, active_dict: dict[str, str] | None = None) -> None: ...
    
    # Methods
    def parse_key(key: str) -> str: ...  # @staticmethod
    def to_json(self) -> str: ...
    def to_dict(self) -> dict[str, str]: ...
    def get_available_languages(self) -> list[str]: ...
    def has_translations(self) -> bool: ...
    
    # Properties (all have get/set)
    chinese_simplified: str | None
    chinese_traditional: str | None
    french: str | None
    german: str | None
    italian: str | None
    japanese: str | None
    korean: str | None
    portuguese_brazil: str | None
    russian: str | None
    spanish: str | None
```

**Analysis:**
- **Complete language coverage**: All major MTG languages supported
- **Flexible initialization**: Can initialize from dictionary or start empty
- **Key parsing**: Static method for handling underscore-to-space conversion
- **Language enumeration**: Methods to check available translations
- **Dictionary integration**: Clean mapping to/from hash maps

---

## High-Performance Modules

### parallel_call.rs - ParallelProcessor & ParallelIterator

**Status:** 🚫 **COMPLETELY DIFFERENT API**

```python
class ParallelProcessor:
    def __init__(self, pool_size: int | None = None) -> None: ...
    
    # Core Methods (DIFFERENT FROM PYTHON VERSION)
    def parallel_call_batch(self, tasks: list[str]) -> list[str]: ...
    def parallel_api_calls(self, urls: list[str]) -> list[str]: ...
    def parallel_transform_fold(self, data: list[str], fold_list: bool, fold_dict: bool) -> list[str]: ...
    def parallel_card_processing(self, card_data: list[str]) -> list[MtgjsonCard]: ...
    def parallel_price_processing(self, providers: list[str]) -> str: ...  # Returns JSON string
    
    # Properties
    pool_size: int

class ParallelIterator:
    def __init__(self, chunk_size: int | None = None, pool_size: int | None = None) -> None: ...
    
    # Methods
    def process_chunks(self, data: list[str]) -> list[str]: ...
    
    # Properties
    chunk_size: int
    pool_size: int
```

**Critical Differences:**
- **Technology**: Uses Tokio async runtime instead of gevent
- **API**: Completely different method signatures from Python version
- **Error handling**: Uses PyResult instead of Python exception patterns
- **Concurrency model**: True parallelism via Rust threads vs Python's async
- **Integration**: Designed for Rust ecosystem, not Python compatibility

**Python Version Expected:**
```python
# Python expects:
def parallel_call(function, arguments, fold_list=False, fold_dict=False): ...
# Rust provides:
def parallel_call_batch(self, tasks: list[str]) -> list[str]: ...
```

---

## Compiled Classes

### compiled_classes/keywords.rs - MtgjsonKeywords

**Status:** ✅ **WORKING WITH COMPREHENSIVE FUNCTIONALITY**

```python
class MtgjsonKeywords:
    def __init__(self) -> None: ...
    
    # Creation Methods
    def from_lists(ability_words: list[str], keyword_actions: list[str], keyword_abilities: list[str]) -> MtgjsonKeywords: ...  # @staticmethod
    
    # Modification Methods
    def add_ability_word(self, ability_word: str) -> None: ...
    def add_keyword_action(self, keyword_action: str) -> None: ...
    def add_keyword_ability(self, keyword_ability: str) -> None: ...
    
    # Query Methods
    def is_ability_word(self, word: str) -> bool: ...
    def is_keyword_action(self, word: str) -> bool: ...
    def is_keyword_ability(self, word: str) -> bool: ...
    def search_keywords(self, substring: str) -> list[str]: ...
    def get_all_keywords(self) -> list[str]: ...
    def total_count(self) -> int: ...
    def to_json(self) -> str: ...
    
    # Properties (all have get/set)
    ability_words: list[str]
    keyword_actions: list[str]
    keyword_abilities: list[str]
```

**Analysis:**
- **Comprehensive functionality**: Complete keyword management system
- **Default data**: Includes extensive default keyword lists
- **Search capabilities**: Case-insensitive search and filtering
- **Modification support**: Add new keywords with automatic deduplication
- **Type checking**: Methods to identify keyword types
- **Performance**: Efficient list operations with sorting

**Default Data Included:**
- **Ability Words**: 41 default entries (Adamant, Addendum, Alliance, etc.)
- **Keyword Actions**: 22 default entries (Abandon, Activate, Adapt, etc.)  
- **Keyword Abilities**: 50+ default entries (Deathtouch, Flying, etc.)

---

## Property Access Patterns Analysis

### Get/Set Annotations

All analyzed classes follow consistent PyO3 property patterns:

```rust
#[pyo3(get, set)]  // Read-write property
pub field: Type,

#[pyo3(get)]       // Read-only property  
pub field: Type,

// Python usage:
obj.field = value  // Set (if allowed)
value = obj.field  // Get
```

### Serialization Patterns

Most classes implement similar serialization patterns:

```rust
#[serde(skip_serializing_if = "skip_if_empty_optional_string")]
#[serde(skip_serializing_if = "Option::is_none")]
#[serde(skip_serializing_if = "skip_if_empty_vec")]
```

### Method Naming Conventions

- `to_json()` - Convert to JSON string
- `to_dict()` - Convert to HashMap/dictionary  
- `new()` - Constructor method
- `get_*()` - Getter methods for computed properties
- `add_*()` - Methods to add items to collections
- `has_*()` - Boolean check methods
- `is_*()` - Type/state checking methods

---

## Compatibility Assessment by File

| File | Status | Issues | Compatibility Score |
|------|---------|---------|---------------------|
| `legalities.rs` | 🚫 | Missing formats, wrong types | 40% |
| `leadership_skills.rs` | ✅ | None | 100% |
| `game_formats.rs` | ✅ | Minor: to_json() returns list | 95% |
| `foreign_data.rs` | ✅ | Minor: to_dict() returns string | 90% |
| `deck.rs` | ✅ | None | 100% |
| `translations.rs` | ✅ | None | 100% |
| `parallel_call.rs` | 🚫 | Completely different API | 10% |
| `keywords.rs` | ✅ | None | 100% |

---

## Missing Files Analysis

The following files still need individual analysis:

### Core Files
- `sealed_product.rs` - SealedProduct classes with enum types
- `related_cards.rs` - Card relationship management
- `purchase_urls.rs` - Purchase link management  
- `meta.rs` - Metadata information
- `price_builder.rs` - Price processing module (incompatible)

### Additional Compiled Classes
- `compiled_classes/all_printings.rs` - Master card collection
- `compiled_classes/atomic_cards.rs` - Unique card aggregation
- `compiled_classes/all_identifiers.rs` - Identifier compilation
- `compiled_classes/card_types.rs` - Card type enumeration
- `compiled_classes/compiled_list.rs` - Compiled metadata
- `compiled_classes/deck_list.rs` - Deck collection
- `compiled_classes/enum_values.rs` - Enumerated values
- `compiled_classes/set_list.rs` - Set collection
- `compiled_classes/structures.rs` - Data structures
- `compiled_classes/tcgplayer_skus.rs` - TCGPlayer integration

---

## Recommendations for Remaining Files

### Priority 1 - Critical Fixes
1. **`legalities.rs`**: Add missing format fields and fix type compatibility
2. **`price_builder.rs`**: Align constructor and method signatures with Python version
3. **`parallel_call.rs`**: Consider Python-compatible wrapper methods

### Priority 2 - Complete Analysis  
1. **`sealed_product.rs`**: Large file with enum types - needs comprehensive analysis
2. **`output_generator.rs`**: Complete analysis of high-performance methods
3. All remaining compiled classes for complete API documentation

### Priority 3 - Documentation Enhancement
1. Add comprehensive examples for all working classes
2. Document performance characteristics and memory usage
3. Create migration guide for incompatible classes

This analysis provides the foundation for completing the individual file documentation and addressing compatibility issues identified in the comprehensive functionality comparison.