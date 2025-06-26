# MTGJSON Rust Transformation Progress

## Completed Fixes ✅

### 1. **PyO3 Module Registration API Update**
- **Issue**: `add_class` method doesn't exist in PyO3 0.22
- **File**: `src/lib.rs`
- **Status**: ⚠️ PARTIALLY FIXED - Updated API calls but still need to fix the method names and module parameter type

### 2. **Missing Utils Module**
- **Issue**: `utils` module was referenced but didn't exist
- **File**: `src/utils.rs` (created)
- **Status**: ✅ COMPLETED
- **Functions implemented**:
  - `alpha_numeric_only()` - Extract alphanumeric characters
  - `sanitize_deck_name()` - Clean deck names for file safety
  - `make_windows_safe_filename()` - Windows-compatible filenames

### 3. **serde_json::Value Replacement**
- **Issue**: `serde_json::Value` not compatible with PyO3
- **Files**: `deck.rs`, `sealed_product.rs`, `set.rs`, `card.rs`
- **Status**: ⚠️ PARTIALLY FIXED - Replaced with `PyObject` but this creates serialization issues

## Critical Issues Remaining ❌

### 1. **PyObject Serialization Conflicts**
**Problem**: `PyObject` doesn't implement `Serialize`, `Deserialize`, `Clone`, or `PartialEq`

**Affected Files**:
- `deck.rs` - All card list fields (`main_board`, `side_board`, etc.)
- `sealed_product.rs` - `contents` field
- `set.rs` - `booster` and `extra_tokens` fields
- `card.rs` - `watermark_resource` field

**Solution Needed**:
- Remove derive macros for structs with `PyObject` fields
- Implement custom serialization/deserialization
- Implement manual `Clone` and `PartialEq` if needed
- Or use a different approach like custom wrapper types

### 2. **PyO3 0.22 API Compatibility**
**Problem**: Multiple API method calls are incorrect for PyO3 0.22

**Issues**:
- `m.add()` method doesn't exist, should use `m.add_class()`
- `py.get_type()` should be `py.get_type_bound()`
- Module parameter type mismatch

**Solution**: Update to correct PyO3 0.22 API calls

### 3. **Documentation Comment Format**
**Problem**: Inner doc comments (`//!`) used incorrectly throughout

**Files**: Almost all source files have this issue
**Solution**: Convert `//!` to `///` for struct/enum documentation

### 4. **Missing Field Error**
**Problem**: `multiverse_id` field missing from `MtgjsonForeignData`
**File**: `src/foreign_data.rs`
**Solution**: Add the missing field or fix the reference

### 5. **Duplicate Watermark Methods**
**Problem**: Duplicate setter methods for `watermark` field
**File**: `src/card.rs`
**Solution**: Remove duplicate method or use proper PyO3 attributes

### 6. **Return Type Compatibility**
**Problem**: Several methods return types that aren't PyO3-compatible
**Examples**:
- `std::cmp::Ordering` return types
- `HashMap<String, serde_json::Value>` return types
- Methods returning references to non-PyO3 types

## Recommended Next Steps

### Immediate Priority (Critical for Compilation)

1. **Fix PyO3 API calls in `lib.rs`**:
   ```rust
   m.add_class::<MtgjsonCard>()?;
   // instead of m.add("MtgjsonCard", py.get_type::<MtgjsonCard>())?;
   ```

2. **Remove problematic derive macros** from structs containing `PyObject`:
   ```rust
   #[derive(Debug)] // Remove Clone, Serialize, Deserialize, PartialEq
   #[pyclass(name = "MtgjsonDeck")]
   ```

3. **Implement custom serialization** for PyObject-containing structs using manual `to_json()` methods

### Medium Priority

4. **Fix documentation comments** throughout codebase
5. **Add missing fields** in `MtgjsonForeignData`
6. **Fix return type compatibility** for PyO3 methods

### Long-term Improvements

7. **Add comprehensive error handling**
8. **Implement proper Python-Rust type conversion**
9. **Add integration tests**
10. **Performance optimization**

## Alternative Approaches to Consider

### Option 1: Custom Wrapper Types
Create PyO3-compatible wrapper types instead of using `PyObject` directly:
```rust
#[pyclass]
struct PyCompatibleValue {
    // Custom implementation
}
```

### Option 2: Simplified Data Model
Use only basic Rust types that naturally support PyO3, avoiding complex nested structures.

### Option 3: Hybrid Approach
Keep some structures as pure Rust with manual Python conversion methods.

## Current Build Status
❌ **Does not compile** - 166+ compilation errors primarily due to:
- PyO3 API mismatches
- PyObject serialization conflicts
- Missing implementations

## Next Session Goals
1. Get the codebase to compile successfully
2. Fix the highest priority PyO3 compatibility issues
3. Implement basic Python interoperability testing