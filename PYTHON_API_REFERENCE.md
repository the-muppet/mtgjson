# MTGJSON Rust - Python API Reference

## Overview

This document provides a comprehensive reference for all Python classes, methods, and functions exposed by the MTGJSON Rust implementation through PyO3 bindings. The module is imported as `mtgjson_rust` and provides high-performance equivalents to the Python MTGJSON classes.

## Module Import

```python
import mtgjson_rust

# All classes are available directly from the module
card = mtgjson_rust.MtgjsonCard()
prices = mtgjson_rust.MtgjsonPrices("paper", "tcgplayer", "2024-01-15", "USD")
```

---

## Core Data Classes

These classes represent the fundamental MTGJSON data structures for cards, sets, and related objects.

### JsonValue

PyO3-compatible wrapper for JSON values.

**Class:** `JsonValue`

```python
class JsonValue:
    def __init__(self, value: str) -> None: ...
    
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    
    # Properties
    value: str  # get, set
```

**Usage:**
```python
json_val = mtgjson_rust.JsonValue('{"key": "value"}')
print(json_val.value)  # {"key": "value"}
```

---

### MtgjsonCard

Represents a single Magic: The Gathering card with all its properties.

**Class:** `MtgjsonCard`

```python
class MtgjsonCard:
    def __init__(self, is_token: bool = False) -> None: ...
    
    # Core Methods
    def to_json(self) -> str: ...
    def set_illustration_ids(self, illustration_ids: list[str]) -> None: ...
    def get_illustration_ids(self) -> list[str]: ...
    def get_names(self) -> list[str]: ...
    def set_names(self, names: list[str] | None) -> None: ...
    def append_names(self, name: str) -> None: ...
    def set_watermark(self, watermark: str | None) -> None: ...
    def get_atomic_keys(self) -> list[str]: ...
    def eq(self, other: MtgjsonCard) -> bool: ...
    def compare(self, other: MtgjsonCard) -> int: ...
    
    # Properties (all have get/set)
    artist: str
    artist_ids: list[str] | None
    ascii_name: str | None
    attraction_lights: list[str] | None
    availability: MtgjsonGameFormats
    booster_types: list[str]
    border_color: str
    card_parts: list[str]
    color_identity: list[str]
    color_indicator: list[str] | None
    colors: list[str]
    converted_mana_cost: float
    count: int
    defense: str | None
    duel_deck: str | None
    edhrec_rank: int | None
    edhrec_saltiness: float | None
    face_converted_mana_cost: float
    face_flavor_name: str | None
    face_mana_value: float
    face_name: str | None
    finishes: list[str]
    first_printing: str | None
    flavor_name: str | None
    flavor_text: str | None
    foreign_data: list[MtgjsonForeignData]
    frame_effects: list[str]
    frame_version: str
    hand: str | None
    has_alternative_deck_limit: bool | None
    has_content_warning: bool | None
    has_foil: bool | None  # Deprecated
    has_non_foil: bool | None  # Deprecated
    identifiers: MtgjsonIdentifiers
    is_alternative: bool | None
    is_foil: bool | None
    is_full_art: bool | None
    is_funny: bool | None
    is_game_changer: bool | None
    is_online_only: bool | None
    is_oversized: bool | None
    is_promo: bool | None
    is_rebalanced: bool | None
    is_reprint: bool | None
    is_reserved: bool | None
    is_starter: bool | None  # Deprecated
    is_story_spotlight: bool | None
    is_textless: bool | None
    is_timeshifted: bool | None
    keywords: list[str]
    language: str
    layout: str
    leadership_skills: MtgjsonLeadershipSkills | None
    legalities: MtgjsonLegalities
    life: str | None
    loyalty: str | None
    mana_cost: str
    mana_value: float
    name: str
    number: str
    orientation: str | None
    original_printings: list[str]
    original_release_date: str | None
    original_text: str | None
    original_type: str | None
    other_face_ids: list[str]
    power: str
    prices: MtgjsonPrices
    printings: list[str]
    promo_types: list[str]
    purchase_urls: MtgjsonPurchaseUrls
    rarity: str
    rebalanced_printings: list[str]
    related_cards: MtgjsonRelatedCards | None
    reverse_related: list[str] | None
    rulings: list[MtgjsonRuling] | None
    security_stamp: str | None
    side: str | None
    signature: str | None
    source_products: dict[str, list[str]] | None
    subsets: list[str] | None
    subtypes: list[str]
    supertypes: list[str]
    text: str
    toughness: str
    type_: str
    types: list[str]
    uuid: str
    variations: list[str]
    watermark: str | None  # get only
    
    # Internal fields
    set_code: str
    is_token: bool
    raw_purchase_urls: dict[str, str]
```

---

### MtgjsonSet

Represents a Magic: The Gathering set with all its cards and metadata.

**Class:** `MtgjsonSet`

```python
class MtgjsonSet:
    def __init__(self) -> None: ...
    
    # Core Methods
    def to_json(self) -> str: ...
    def get_windows_safe_set_code(self) -> str: ...
    def add_card(self, card: MtgjsonCard) -> None: ...
    def add_token(self, token: MtgjsonCard) -> None: ...
    def add_deck(self, deck: MtgjsonDeck) -> None: ...
    def add_sealed_product(self, product: MtgjsonSealedProduct) -> None: ...
    def sort_cards(self) -> None: ...
    def sort_tokens(self) -> None: ...
    def get_total_cards(self) -> int: ...
    def get_cards_by_rarity(self) -> dict[str, int]: ...
    def get_unique_languages(self) -> list[str]: ...
    def find_card_by_name(self, name: str) -> int | None: ...
    def find_card_by_uuid(self, uuid: str) -> int | None: ...
    def get_cards_of_rarity(self, rarity: str) -> list[int]: ...
    def has_foil_cards(self) -> bool: ...
    def has_non_foil_cards(self) -> bool: ...
    def get_statistics(self) -> str: ...
    def update_set_sizes(self) -> None: ...
    def validate(self) -> list[str]: ...
    
    # Properties (all have get/set)
    base_set_size: int | None
    booster: str | None
    cards: list[MtgjsonCard]
    cardsphere_set_id: int | None
    code: str | None
    code_v3: str | None
    decks: list[MtgjsonDeck]
    is_foreign_only: bool
    is_foil_only: bool
    is_non_foil_only: bool
    is_online_only: bool
    is_partial_preview: bool
    keyrune_code: str | None
    languages: list[str]
    mcm_id: int | None
    mcm_id_extras: int | None
    mcm_name: str | None
    mtgo_code: str | None
    name: str
    parent_code: str | None
    release_date: str
    tcgplayer_group_id: int | None
    sealed_product: list[MtgjsonSealedProduct]
    tokens: list[MtgjsonCard]
    token_set_code: str | None
    total_set_size: int
    translations: MtgjsonTranslations
    type_: str
    extra_tokens: list[str]
    search_uri: str
```

---

### MtgjsonIdentifiers

Card identifiers for various platforms and services.

**Class:** `MtgjsonIdentifiers`

```python
class MtgjsonIdentifiers:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def to_dict(self) -> dict[str, str]: ...
    def has_identifiers(self) -> bool: ...
    def count_identifiers(self) -> int: ...
    
    # Properties (all have get/set)
    card_kingdom_etched_id: str | None
    card_kingdom_foil_id: str | None
    card_kingdom_id: str | None
    cardsphere_foil_id: str | None
    cardsphere_id: str | None
    mcm_id: str | None
    mcm_meta_id: str | None
    mtg_arena_id: str | None
    mtgjson_foil_version_id: str | None
    mtgjson_non_foil_version_id: str | None
    mtgjson_v4_id: str | None
    mtgo_foil_id: str | None
    mtgo_id: str | None
    multiverse_id: str | None
    scryfall_id: str | None
    scryfall_illustration_id: str | None
    scryfall_card_back_id: str | None
    scryfall_oracle_id: str | None
    tcgplayer_etched_product_id: str | None
    tcgplayer_product_id: str | None
```

---

### MtgjsonPrices

Price information for cards from various providers.

**Class:** `MtgjsonPrices`

```python
class MtgjsonPrices:
    def __init__(
        self,
        source: str,
        provider: str,
        date: str,
        currency: str,
        buy_normal: float | None = None,
        buy_foil: float | None = None,
        buy_etched: float | None = None,
        sell_normal: float | None = None,
        sell_foil: float | None = None,
        sell_etched: float | None = None
    ) -> None: ...
    
    # Methods
    def items(self) -> list[tuple[str, float | None]]: ...
    def to_json(self) -> str: ...
    def to_json_structure(self) -> str: ...
    def has_price_data(self) -> bool: ...
    def get_buy_prices(self) -> dict[str, float]: ...
    def get_sell_prices(self) -> dict[str, float]: ...
    def get_spread(self, finish: str) -> float | None: ...
    def get_price_count(self) -> int: ...
    
    # Properties (all have get/set)
    source: str
    provider: str
    date: str
    currency: str
    buy_normal: float | None
    buy_foil: float | None
    buy_etched: float | None
    sell_normal: float | None
    sell_foil: float | None
    sell_etched: float | None
```

---

### MtgjsonRuling

Official rulings and clarifications for cards.

**Class:** `MtgjsonRuling`

```python
class MtgjsonRuling:
    def __init__(self, date: str, text: str) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def is_valid(self) -> bool: ...
    def get_summary(self) -> str: ...
    def compare_by_date(self, other: MtgjsonRuling) -> int: ...
    def get_word_count(self) -> int: ...
    def contains_keyword(self, keyword: str) -> bool: ...
    def get_character_count(self) -> int: ...
    
    # Properties (all have get/set)
    date: str
    text: str
```

---

### MtgjsonLegalities

Card legality information across different formats.

**Class:** `MtgjsonLegalities`

```python
class MtgjsonLegalities:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    def get_legal_formats(self) -> list[str]: ...
    def to_dict(self) -> dict[str, str]: ...
    
    # Properties (all have get/set)
    # Note: Field types incompatible with Python version
    alchemy: str | None
    brawl: str | None
    commander: str | None
    duel: str | None
    explorer: str | None
    future: str | None
    gladiator: str | None
    historic: str | None
    legacy: str | None
    modern: str | None
    oldschool: str | None
    pauper: str | None
    paupercommander: str | None
    penny: str | None
    pioneer: str | None
    predh: str | None
    premodern: str | None
    standard: str | None
    timeless: str | None
    vintage: str | None
```

---

### MtgjsonDeck

Represents a pre-constructed deck.

**Class:** `MtgjsonDeck`

```python
class MtgjsonDeck:
    def __init__(self) -> None: ...
    
    # Methods (signatures not fully detailed in scope)
    def to_json(self) -> str: ...
    # Additional methods available...
    
    # Properties (all have get/set)
    # Note: Full property list not detailed in provided scope
```

---

### MtgjsonDeckHeader

Header information for deck collections.

**Class:** `MtgjsonDeckHeader`

```python
class MtgjsonDeckHeader:
    def __init__(self) -> None: ...
    
    # Methods
    def to_json(self) -> str: ...
    # Additional methods available...
```

---

### Other Core Classes

The following classes are available with similar patterns (exact signatures would need individual file analysis):

- **MtgjsonForeignData** - Foreign language card data
- **MtgjsonGameFormats** - Format availability information  
- **MtgjsonLeadershipSkills** - Commander format leadership skills
- **MtgjsonMeta** - Metadata information
- **MtgjsonPurchaseUrls** - Purchase links for cards
- **MtgjsonRelatedCards** - Related card relationships
- **MtgjsonSealedProduct** - Sealed product information
- **MtgjsonTranslations** - Multi-language translations

---

## Compiled Classes

These classes represent higher-level aggregated data structures for MTGJSON compilation output.

### MtgjsonAllPrintings

Master collection of all sets and cards.

**Class:** `MtgjsonAllPrintings`

```python
class MtgjsonAllPrintings:
    def __init__(self) -> None: ...
    
    # Methods
    def add_set(self, set_code: str, set_data: MtgjsonSet) -> None: ...
    
    # Properties
    all_sets_dict: dict[str, MtgjsonSet]  # get, set
```

---

### MtgjsonAtomicCards

Atomic card data (unique cards across all printings).

**Class:** `MtgjsonAtomicCards`

```python
class MtgjsonAtomicCards:
    def __init__(self, cards: dict | None) -> None: ...
    
    # Methods (signatures need individual analysis)
    # Additional methods available...
```

---

### Other Compiled Classes

The following compiled classes are available:

- **MtgjsonAllIdentifiers** - All card identifiers compilation
- **MtgjsonCardTypes** - All card types compilation  
- **MtgjsonCompiledList** - Compiled list metadata
- **MtgjsonDeckList** - All decks compilation
- **MtgjsonEnumValues** - Enumerated values compilation
- **MtgjsonKeywords** - All keywords compilation
- **MtgjsonSetList** - All sets list compilation
- **MtgjsonStructures** - Structural data compilation
- **MtgjsonTcgplayerSkus** - TCGPlayer SKU data

---

## High-Performance Modules

These classes provide high-performance computational functionality.

### OutputGenerator

High-performance file generation and JSON processing.

**Class:** `OutputGenerator`

```python
class OutputGenerator:
    def __init__(self, output_path: str, pretty_print: bool) -> None: ...
    
    # Core Methods
    def generate_compiled_output_files(self) -> None: ...
    def build_all_printings_files(self) -> None: ...
    def build_format_specific_files(self, all_printings: MtgjsonAllPrintings) -> None: ...
    def build_atomic_cards(self) -> None: ...
    def build_atomic_specific_files(self) -> None: ...
    def generate_compiled_prices_output(self) -> None: ...
    def build_compiled_list(self) -> None: ...
    def build_keywords(self) -> None: ...
    def build_card_types(self) -> None: ...
    def build_meta(self) -> None: ...
    def build_set_list(self) -> None: ...
    def build_deck_list(self) -> None: ...
    def build_enum_values(self) -> None: ...
    def create_compiled_output(self, filename: str, data_json: str) -> None: ...
    def construct_format_map(self) -> dict[str, list[str]]: ...
    def construct_atomic_cards_format_map(self) -> dict[str, list[str]]: ...
    def filter_all_printings_by_format(
        self, 
        all_printings: MtgjsonAllPrintings, 
        format_name: str
    ) -> MtgjsonAllPrintings: ...
    def generate_output_file_hashes(self) -> None: ...
    def calculate_file_hash(self, path: str) -> str: ...
    
    # Properties
    output_path: str  # get, set
    pretty_print: bool  # get, set
```

---

### PriceBuilder

High-performance price data processing.

**Class:** `PriceBuilder`

```python
class PriceBuilder:
    def __init__(self, all_printings_path: str, providers: list[str]) -> None: ...
    
    # Methods
    def build_prices(self) -> str: ...  # Returns JSON string
    # Additional methods available...
    
    # Note: Incompatible constructor and return types vs Python version
```

---

### ParallelProcessor

Parallel processing capabilities.

**Class:** `ParallelProcessor`

```python
class ParallelProcessor:
    def __init__(self) -> None: ...
    
    # Methods
    def process_parallel_chunks(self, data: str, chunk_size: int) -> str: ...
    # Additional methods available...
    
    # Note: Completely different API from Python version
```

---

### ParallelIterator

Iterator for parallel processing.

**Class:** `ParallelIterator`

```python
class ParallelIterator:
    def __init__(self) -> None: ...
    
    # Methods
    # Specific signatures need individual analysis
```

---

## Enums and Special Classes

### SealedProductCategory

Enumeration for sealed product categories.

**Class:** `SealedProductCategory`

```python
class SealedProductCategory:
    # Enum values and methods need individual analysis
```

---

### SealedProductSubtype

Enumeration for sealed product subtypes.

**Class:** `SealedProductSubtype`

```python
class SealedProductSubtype:
    # Enum values and methods need individual analysis
```

---

## Property Access Patterns

All classes follow consistent PyO3 property patterns:

```python
# Property getters and setters
card = mtgjson_rust.MtgjsonCard()

# Get property
name = card.name

# Set property  
card.name = "Lightning Bolt"

# Properties marked as 'get only' cannot be set
watermark = card.watermark  # OK
card.watermark = "New"      # Would raise AttributeError
```

---

## Error Handling

PyO3 methods raise standard Python exceptions:

```python
try:
    card = mtgjson_rust.MtgjsonCard()
    json_str = card.to_json()
except ValueError as e:
    print(f"Serialization error: {e}")

try:
    output_gen = mtgjson_rust.OutputGenerator("/invalid/path", True)
    output_gen.generate_compiled_output_files()
except IOError as e:
    print(f"File system error: {e}")
```

---

## Performance Notes

- **Memory Management**: Objects are managed by Rust, providing better memory safety
- **Serialization**: JSON operations are highly optimized through Rust's serde
- **Parallel Processing**: True parallel processing capabilities through Rust's threading
- **Type Safety**: Rust's type system prevents many runtime errors

---

## Compatibility Issues

Based on the analysis, several classes have compatibility issues with their Python counterparts:

### 🚫 Known Issues

1. **MtgjsonLegalities**: Field types incompatible (str vs Option<String>)
2. **MtgjsonSealedProduct**: Missing initialization and wrong contents type
3. **OutputGenerator**: Incompatible method signatures 
4. **PriceBuilder**: Incompatible constructor and return types
5. **ParallelProcessor**: Completely different API

### ⚠️ Minor Differences

1. **MtgjsonPrices**: `to_json()` returns different structure
2. **Parallel Processing**: Different async architecture
3. **UUID Generation**: Non-deterministic vs deterministic

### ✅ Fully Compatible

1. **MtgjsonIdentifiers**: Fully compatible with Python version
2. **MtgjsonRuling**: Basic functionality works correctly  
3. **Core PyO3 bindings**: Python integration functional
4. **Basic serialization**: JSON operations work correctly

---

## Usage Examples

### Basic Card Creation and Manipulation

```python
import mtgjson_rust

# Create a new card
card = mtgjson_rust.MtgjsonCard(is_token=False)
card.name = "Lightning Bolt"
card.mana_cost = "{R}"
card.type_ = "Instant"
card.text = "Lightning Bolt deals 3 damage to any target."

# Work with identifiers
card.identifiers.scryfall_id = "12345678-1234-1234-1234-123456789012"
card.identifiers.multiverse_id = "12345"

print(f"Card has {card.identifiers.count_identifiers()} identifiers")

# Convert to JSON
json_output = card.to_json()
```

### Price Data Management

```python
# Create price data
prices = mtgjson_rust.MtgjsonPrices(
    source="paper",
    provider="tcgplayer", 
    date="2024-01-15",
    currency="USD",
    buy_normal=2.50,
    sell_normal=3.25,
    sell_foil=6.75
)

# Check price data
if prices.has_price_data():
    print(f"Available prices: {prices.get_price_count()} out of 6 possible")
    
    # Get buy/sell prices by finish
    buy_prices = prices.get_buy_prices()
    sell_prices = prices.get_sell_prices()
    
    # Calculate spread (profit margin)
    spread = prices.get_spread("normal")
    if spread is not None:
        print(f"Dealer margin: ${spread:.2f}")
```

### Set Management

```python
# Create a set
set_obj = mtgjson_rust.MtgjsonSet()
set_obj.code = "LTR"
set_obj.name = "The Lord of the Rings: Tales of Middle-earth"
set_obj.release_date = "2023-06-23"

# Add cards to set
set_obj.add_card(card)

# Set operations
set_obj.sort_cards()
total_cards = set_obj.get_total_cards()
rarity_breakdown = set_obj.get_cards_by_rarity()

# Validation
errors = set_obj.validate()
if errors:
    print("Set validation errors:", errors)
```

### High-Performance Output Generation

```python
# Create output generator
output_gen = mtgjson_rust.OutputGenerator(
    output_path="./output",
    pretty_print=True
)

# Generate all compiled outputs
try:
    output_gen.generate_compiled_output_files()
    output_gen.generate_output_file_hashes()
    print("Output generation completed successfully")
except Exception as e:
    print(f"Output generation failed: {e}")
```

This completes the comprehensive Python API reference for the MTGJSON Rust implementation.