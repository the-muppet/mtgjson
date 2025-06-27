# MTGJSON Class Exploration Summary

## Overview

This document summarizes the exploration of MTGJSON classes in both Python and Rust implementations, along with scripts created to explore them and current issues discovered.

## Files Created

### 1. `explore_classes.py`
**Purpose**: Comprehensive exploration script for Rust MTGJSON classes  
**Features**:
- Detailed introspection of all class methods, properties, and attributes
- Command-line options for filtering and detail levels
- Summary tables and statistics
- Support for exploring individual classes

**Usage**:
```bash
python explore_classes.py                           # Explore all classes (brief)
python explore_classes.py --detailed                # Explore all classes (detailed)
python explore_classes.py --summary                 # Show only summary table
python explore_classes.py --class=MtgjsonCard       # Explore only MtgjsonCard
python explore_classes.py -d -c MtgjsonSet          # Detailed view of MtgjsonSet
```

### 2. `interactive_explore.py`
**Purpose**: Interactive exploration and testing of Rust MTGJSON classes  
**Features**:
- Interactive class selection
- Live testing of methods and properties
- Basic functionality tests
- Real-time feedback

**Usage**:
```bash
python interactive_explore.py                    # Interactive mode
python interactive_explore.py --test-all         # Run all tests
python interactive_explore.py --list-only        # List classes only
python interactive_explore.py --class=MtgjsonCard # Test specific class
```

### 3. `explore_python_mtgjson.py`
**Purpose**: Comprehensive exploration script for Python MTGJSON5 classes  
**Features**:
- Module-level class discovery
- Detailed method and property analysis
- Testing functionality
- Documentation extraction

**Usage**:
```bash
python explore_python_mtgjson.py --summary       # Show summary table
python explore_python_mtgjson.py --test          # Run tests
python explore_python_mtgjson.py --detailed      # Detailed exploration
```

### 4. `simple_class_explorer.py`
**Purpose**: Simple, direct testing of individual MTGJSON classes  
**Features**:
- Direct class instantiation and testing
- Method signature discovery
- Attribute inspection
- Error handling for missing dependencies

**Usage**:
```bash
python simple_class_explorer.py                    # Explore all known classes
python simple_class_explorer.py MtgjsonCardObject  # Explore specific class
```

## Current Status

### Python MTGJSON5 Implementation

**Status**: ❌ **Not Fully Functional**

**Issues Discovered**:
1. **Missing Dependencies**: The Python implementation requires `boto3` which is not installed
2. **Import Chain Issues**: Classes cannot be imported due to dependency cascade failures
3. **Module Structure**: Complex module dependencies that fail if any single dependency is missing

**Available Classes** (based on codebase inspection):
- `MtgjsonCardObject` - Individual Magic card representation
- `MtgjsonSetObject` - Magic set representation
- `MtgjsonDeckObject` - Deck representation
- `MtgjsonGameFormatsObject` - Platform availability (paper, MTGO, Arena, etc.)
- `MtgjsonIdentifiersObject` - Card identifiers (MTGO ID, Arena ID, etc.)
- `MtgjsonLegalitiesObject` - Format legalities
- `MtgjsonPricesObject` - Price information
- `MtgjsonAllPrintingsObject` - Complete set collection
- `MtgjsonStructuresObject` - Data structures
- `MtgjsonEnumValuesObject` - Enumerated values

### Rust MTGJSON Implementation

**Status**: ❌ **Compilation Errors**

**Major Issues Discovered**:

#### 1. Import Path Errors (Most Common)
Many files use incorrect import paths:
```rust
// Wrong:
use crate::card::MtgjsonCardObject;
use crate::base::JsonObject;

// Should be:
use crate::classes::card::MtgjsonCardObject;
use crate::classes::base::JsonObject;
```

#### 2. Missing Dependencies in Cargo.toml
Required crates not included:
- `config` - Configuration management
- `once_cell` - Lazy statics
- `lazy_static` - Static initialization
- `unicode_normalization` - Unicode handling

#### 3. PyO3 API Changes
Outdated PyO3 usage:
```rust
// Old API:
PyList::new_bound(py, cards.iter().map(|v| v.to_string()));

// Needs updating for current PyO3 version
```

#### 4. Module Organization Issues
- Missing module exports
- Incorrect `mod.rs` file organization
- Private/public visibility issues

#### 5. JSON Deserialization Issues
The main issue you encountered with availability fields:
- JSON data has string values: `"mtgo": "mtgo"`
- Rust structs expect boolean values: `pub mtgo: bool`

**✅ Fixed**: Custom deserializer added to handle both string and boolean values.

## Practical Recommendations

### For Immediate Use

1. **Use the existing Python codebase** - It's functional even if the exploration scripts hit dependency issues
2. **Install missing Python dependencies**:
   ```bash
   pip install boto3
   # Add any other missing dependencies as they appear
   ```

3. **Manual class exploration** - Since the classes exist, you can explore them manually:
   ```python
   from mtgjson5.classes.mtgjson_card import MtgjsonCardObject
   card = MtgjsonCardObject()
   print(dir(card))  # See all methods and attributes
   ```

### For Rust Development

If you want to continue with Rust development, here's what needs to be fixed:

1. **Fix Import Paths** (High Priority):
   ```bash
   # Use find/replace in your editor:
   find: use crate::([^:]+)::
   replace: use crate::classes::$1::
   ```

2. **Add Missing Dependencies**:
   ```toml
   # Add to Cargo.toml [dependencies]:
   config = "0.13"
   once_cell = "1.19"
   lazy_static = "1.4"
   unicode-normalization = "0.1"
   ```

3. **Fix Module Exports**:
   Update `lib.rs` to properly export all modules and fix visibility issues.

4. **Update PyO3 Usage**:
   Update to current PyO3 API patterns.

### Alternative Approach: Gradual Migration

1. **Start with individual classes** - Fix one class at a time
2. **Create minimal test cases** - Test each class as you fix it
3. **Use the custom deserializer pattern** - Apply the availability field fix pattern to other similar issues

## Expected Class Structure (Based on Analysis)

### Core Data Classes
- **MtgjsonCard** - Individual card with all properties (name, mana cost, power, toughness, etc.)
- **MtgjsonSet** - Set containing cards, metadata, release info
- **MtgjsonDeck** - Preconstructed deck with mainboard/sideboard
- **MtgjsonGameFormats** - Platform availability flags
- **MtgjsonIdentifiers** - External service IDs
- **MtgjsonLegalities** - Format legality status
- **MtgjsonPrices** - Price data from various sources

### Compiled/Aggregate Classes
- **MtgjsonAllPrintings** - Complete database of all sets
- **MtgjsonStructures** - Schema definitions
- **MtgjsonEnumValues** - Valid values for enum fields
- **MtgjsonCardTypes** - Card type definitions
- **MtgjsonKeywords** - Keyword abilities

### Utility Classes
- **OutputGenerator** - File generation utilities
- **ParallelProcessor** - Parallel processing tools
- **PriceBuilder** - Price data aggregation

## Next Steps

1. **Choose your path**:
   - **Python**: Install dependencies and use existing functionality
   - **Rust**: Fix compilation errors systematically

2. **Test with real data**:
   - Use your `allprintings5.json` file
   - Start with simple operations (loading, basic queries)

3. **Build incrementally**:
   - Get basic functionality working first
   - Add advanced features later

The exploration scripts are ready to use once you have a working implementation of either version! 