#!/usr/bin/env python3
"""
Interactive exploration script for MTGJSON Rust classes
"""

import sys
from pathlib import Path

def test_all_printings():
    """Test MtgjsonAllPrintings functionality"""
    print("\n🧪 Testing MtgjsonAllPrintings:")
    
    try:
        import mtgjson_rust
        
        # Create instance
        all_printings = mtgjson_rust.MtgjsonAllPrintings()
        print(f"  ✓ Created MtgjsonAllPrintings instance")
        
        # Test basic methods
        print(f"  • Length: {all_printings.len()}")
        print(f"  • Is empty: {all_printings.is_empty()}")
        
        # Test from_path if we have a file
        test_files = [
            r"C:\Users\rprat\Downloads\betamtgban\allprintings5.json",
            "AllPrintings.json",
            "test_data.json"
        ]
        
        for test_file in test_files:
            if Path(test_file).exists():
                print(f"  • Found test file: {test_file}")
                try:
                    loaded = mtgjson_rust.MtgjsonAllPrintings.from_path(test_file)
                    print(f"    ✓ Loaded from {test_file}")
                    print(f"    • Contains {loaded.len()} sets")
                    if not loaded.is_empty():
                        codes = loaded.list_set_codes()
                        print(f"    • First few set codes: {codes[:5] if codes else 'None'}")
                    break
                except Exception as e:
                    print(f"    ✗ Failed to load {test_file}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonAllPrintings: {e}")

def test_card_class():
    """Test MtgjsonCard functionality"""
    print("\n🧪 Testing MtgjsonCard:")
    
    try:
        import mtgjson_rust
        
        # Create instance
        card = mtgjson_rust.MtgjsonCard()
        print(f"  ✓ Created MtgjsonCard instance")
        
        # Test properties (get/set)
        properties_to_test = ['name', 'mana_cost', 'type_line', 'power', 'toughness', 'rarity']
        
        for prop in properties_to_test:
            try:
                if hasattr(card, prop):
                    value = getattr(card, prop)
                    print(f"  • {prop}: {value} (type: {type(value).__name__})")
                    
                    # Try setting a test value
                    if isinstance(value, str):
                        setattr(card, prop, f"test_{prop}")
                        new_value = getattr(card, prop)
                        print(f"    └─ Set to: {new_value}")
            except Exception as e:
                print(f"  ✗ Error with {prop}: {e}")
        
        # Test methods
        methods_to_test = ['to_json', '__str__', '__repr__']
        for method in methods_to_test:
            try:
                if hasattr(card, method):
                    result = getattr(card, method)()
                    print(f"  • {method}(): {str(result)[:100]}{'...' if len(str(result)) > 100 else ''}")
            except Exception as e:
                print(f"  ✗ Error with {method}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonCard: {e}")

def test_set_class():
    """Test MtgjsonSet functionality"""
    print("\n🧪 Testing MtgjsonSet:")
    
    try:
        import mtgjson_rust
        
        # Create instance
        mtgjson_set = mtgjson_rust.MtgjsonSet()
        print(f"  ✓ Created MtgjsonSet instance")
        
        # Test basic properties
        basic_props = ['name', 'code', 'type_', 'release_date']
        for prop in basic_props:
            try:
                if hasattr(mtgjson_set, prop):
                    value = getattr(mtgjson_set, prop)
                    print(f"  • {prop}: '{value}' (type: {type(value).__name__})")
            except Exception as e:
                print(f"  ✗ Error with {prop}: {e}")
        
        # Test collection properties
        collection_props = ['cards', 'tokens', 'decks']
        for prop in collection_props:
            try:
                if hasattr(mtgjson_set, prop):
                    value = getattr(mtgjson_set, prop)
                    print(f"  • {prop}: {len(value) if value else 0} items")
            except Exception as e:
                print(f"  ✗ Error with {prop}: {e}")
        
        # Test methods
        methods_to_test = ['get_total_cards', 'has_foil_cards', 'has_non_foil_cards']
        for method in methods_to_test:
            try:
                if hasattr(mtgjson_set, method):
                    result = getattr(mtgjson_set, method)()
                    print(f"  • {method}(): {result}")
            except Exception as e:
                print(f"  ✗ Error with {method}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonSet: {e}")

def test_game_formats():
    """Test MtgjsonGameFormats functionality"""
    print("\n🧪 Testing MtgjsonGameFormats:")
    
    try:
        import mtgjson_rust
        
        # Create instance
        formats = mtgjson_rust.MtgjsonGameFormats()
        print(f"  ✓ Created MtgjsonGameFormats instance")
        
        # Test availability properties
        availability_props = ['paper', 'mtgo', 'arena', 'shandalar', 'dreamcast']
        for prop in availability_props:
            try:
                if hasattr(formats, prop):
                    value = getattr(formats, prop)
                    print(f"  • {prop}: {value}")
                    
                    # Try setting to True
                    setattr(formats, prop, True)
                    new_value = getattr(formats, prop)
                    print(f"    └─ Set to: {new_value}")
            except Exception as e:
                print(f"  ✗ Error with {prop}: {e}")
        
        # Test methods
        methods_to_test = ['to_json', 'get_available_formats']
        for method in methods_to_test:
            try:
                if hasattr(formats, method):
                    result = getattr(formats, method)()
                    print(f"  • {method}(): {result}")
            except Exception as e:
                print(f"  ✗ Error with {method}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonGameFormats: {e}")

def test_identifiers():
    """Test MtgjsonIdentifiers functionality"""
    print("\n🧪 Testing MtgjsonIdentifiers:")
    
    try:
        import mtgjson_rust
        
        # Create instance
        identifiers = mtgjson_rust.MtgjsonIdentifiers()
        print(f"  ✓ Created MtgjsonIdentifiers instance")
        
        # Test ID properties (they should be Optional[String])
        id_props = ['mtgo_id', 'mtg_arena_id', 'multiverse_id', 'scryfall_id', 'tcgplayer_product_id']
        for prop in id_props:
            try:
                if hasattr(identifiers, prop):
                    value = getattr(identifiers, prop)
                    print(f"  • {prop}: {value}")
                    
                    # Try setting a test ID
                    setattr(identifiers, prop, f"test_{prop}_123")
                    new_value = getattr(identifiers, prop)
                    print(f"    └─ Set to: {new_value}")
            except Exception as e:
                print(f"  ✗ Error with {prop}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonIdentifiers: {e}")

def list_available_classes():
    """List all available classes in the module"""
    print("\n📋 Available Classes:")
    
    try:
        import mtgjson_rust
        
        # Get all attributes from the module
        all_attrs = dir(mtgjson_rust)
        
        # Filter for classes (usually start with uppercase)
        classes = [attr for attr in all_attrs if attr[0].isupper()]
        
        print(f"  Found {len(classes)} classes:")
        for i, cls_name in enumerate(sorted(classes), 1):
            try:
                cls = getattr(mtgjson_rust, cls_name)
                if hasattr(cls, '__doc__'):
                    doc = cls.__doc__ or "No documentation"
                    doc_short = doc.split('\n')[0][:60] + '...' if len(doc) > 60 else doc.split('\n')[0]
                else:
                    doc_short = "No documentation"
                print(f"    {i:2d}. {cls_name:<25} - {doc_short}")
            except Exception as e:
                print(f"    {i:2d}. {cls_name:<25} - Error: {e}")
        
        return classes
        
    except ImportError as e:
        print(f"  ✗ Failed to import mtgjson_rust: {e}")
        return []

def interactive_class_explorer(classes):
    """Interactive class exploration"""
    print(f"\n🔍 Interactive Class Explorer")
    print(f"Enter class name to explore (or 'quit' to exit):")
    
    while True:
        try:
            choice = input(f"\n> ").strip()
            
            if choice.lower() in ['quit', 'exit', 'q']:
                break
            
            if not choice:
                continue
            
            if choice in classes:
                explore_single_class(choice)
            else:
                print(f"Class '{choice}' not found. Available classes:")
                for cls in sorted(classes)[:10]:  # Show first 10
                    print(f"  • {cls}")
                if len(classes) > 10:
                    print(f"  ... and {len(classes) - 10} more")
        
        except KeyboardInterrupt:
            print(f"\n👋 Goodbye!")
            break
        except Exception as e:
            print(f"Error: {e}")

def explore_single_class(class_name):
    """Explore a single class interactively"""
    try:
        import mtgjson_rust
        cls = getattr(mtgjson_rust, class_name)
        
        print(f"\n🔬 Exploring {class_name}:")
        
        # Create instance
        try:
            instance = cls()
            print(f"  ✓ Created instance: {instance}")
        except Exception as e:
            print(f"  ✗ Cannot create instance: {e}")
            instance = None
        
        # List methods and properties
        members = [attr for attr in dir(cls) if not attr.startswith('_')]
        methods = []
        properties = []
        
        for member in members:
            try:
                obj = getattr(cls, member)
                if callable(obj):
                    methods.append(member)
                else:
                    properties.append(member)
            except:
                pass
        
        if methods:
            print(f"  📝 Methods ({len(methods)}): {', '.join(methods[:5])}{'...' if len(methods) > 5 else ''}")
        
        if properties:
            print(f"  📊 Properties ({len(properties)}): {', '.join(properties[:5])}{'...' if len(properties) > 5 else ''}")
        
        # Test a few things if we have an instance
        if instance:
            # Try string representation
            try:
                str_repr = str(instance)
                print(f"  📄 String representation: {str_repr[:100]}{'...' if len(str_repr) > 100 else ''}")
            except:
                pass
            
            # Try some common methods
            common_methods = ['to_json', 'len', 'is_empty']
            for method in common_methods:
                if hasattr(instance, method):
                    try:
                        result = getattr(instance, method)()
                        print(f"  🔧 {method}(): {result}")
                    except Exception as e:
                        print(f"  ⚠️  {method}(): Error - {e}")
    
    except Exception as e:
        print(f"  ✗ Error exploring {class_name}: {e}")

def main():
    """Main function"""
    
    if '--help' in sys.argv or '-h' in sys.argv:
        print("""
Interactive MTGJSON Rust Class Explorer

Usage: python interactive_explore.py [options]

Options:
  -h, --help     Show this help message
  --test-all     Run all test functions
  --list-only    Just list available classes
  --class=NAME   Test specific class only

Examples:
  python interactive_explore.py                    # Interactive mode
  python interactive_explore.py --test-all         # Run all tests
  python interactive_explore.py --list-only        # List classes only
  python interactive_explore.py --class=MtgjsonCard # Test specific class
        """)
        return
    
    print("🚀 MTGJSON Rust Interactive Explorer")
    print("="*50)
    
    # List available classes first
    classes = list_available_classes()
    
    if '--list-only' in sys.argv:
        return
    
    # Check for specific class test
    class_filter = None
    for arg in sys.argv[1:]:
        if arg.startswith('--class='):
            class_filter = arg.split('=', 1)[1]
    
    if class_filter:
        if class_filter in classes:
            explore_single_class(class_filter)
        else:
            print(f"Class '{class_filter}' not found")
        return
    
    # Run specific tests
    if '--test-all' in sys.argv:
        test_all_printings()
        test_card_class()
        test_set_class()
        test_game_formats()
        test_identifiers()
        return
    
    # Default: run some basic tests then go interactive
    print("\n🧪 Running basic tests...")
    test_all_printings()
    test_game_formats()
    
    # Interactive mode
    print(f"\n" + "="*50)
    interactive_class_explorer(classes)

if __name__ == "__main__":
    main() 