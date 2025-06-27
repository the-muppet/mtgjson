#!/usr/bin/env python3
"""
Simple class explorer for MTGJSON5 classes
"""

import sys
import inspect
from typing import Any, Dict, List

def explore_class_simple(cls, name: str):
    """Simple class exploration"""
    print(f"\n{'='*60}")
    print(f"CLASS: {name}")
    print(f"{'='*60}")
    
    try:
        # Create instance
        instance = cls()
        print(f"✓ Successfully created {name} instance")
        
        # Get all public methods and attributes
        methods = []
        properties = []
        attributes = []
        
        for attr_name in dir(instance):
            if attr_name.startswith('_'):
                continue
                
            try:
                attr = getattr(instance, attr_name)
                if callable(attr):
                    try:
                        sig = str(inspect.signature(attr))
                        methods.append((attr_name, sig))
                    except:
                        methods.append((attr_name, "()"))
                else:
                    attr_type = type(attr).__name__
                    attr_value = str(attr)[:50] + ('...' if len(str(attr)) > 50 else '')
                    attributes.append((attr_name, attr_type, attr_value))
            except Exception as e:
                attributes.append((attr_name, "error", str(e)[:50]))
        
        # Print methods
        if methods:
            print(f"\nMETHODS ({len(methods)}):")
            for method_name, sig in sorted(methods):
                print(f"  • {method_name}{sig}")
        
        # Print attributes
        if attributes:
            print(f"\nATTRIBUTES ({len(attributes)}):")
            for attr_name, attr_type, attr_value in sorted(attributes):
                print(f"  • {attr_name} ({attr_type}): {attr_value}")
        
        # Test some common operations
        print(f"\nTEST OPERATIONS:")
        
        # Test string representation
        try:
            str_repr = str(instance)
            print(f"  • str(): {str_repr[:100]}{'...' if len(str_repr) > 100 else ''}")
        except Exception as e:
            print(f"  • str(): Error - {e}")
        
        # Test common methods
        common_methods = ['to_json', '__dict__', 'build_keys_to_skip']
        for method in common_methods:
            if hasattr(instance, method):
                try:
                    result = getattr(instance, method)()
                    if isinstance(result, (dict, list)):
                        print(f"  • {method}(): {type(result).__name__} with {len(result)} items")
                    else:
                        result_str = str(result)[:100] + ('...' if len(str(result)) > 100 else '')
                        print(f"  • {method}(): {result_str}")
                except Exception as e:
                    print(f"  • {method}(): Error - {e}")
        
        return True
        
    except Exception as e:
        print(f"✗ Failed to create {name} instance: {e}")
        return False

def main():
    """Main function"""
    
    if '--help' in sys.argv or '-h' in sys.argv:
        print("""
Simple MTGJSON Class Explorer

Usage: python simple_class_explorer.py [class_name]

Examples:
  python simple_class_explorer.py                    # Explore all known classes
  python simple_class_explorer.py MtgjsonCardObject  # Explore specific class
        """)
        return
    
    print("🔍 Simple MTGJSON Class Explorer")
    print("="*50)
    
    # Classes to test
    classes_to_test = [
        ("MtgjsonCardObject", "mtgjson5.classes.mtgjson_card"),
        ("MtgjsonSetObject", "mtgjson5.classes.mtgjson_set"), 
        ("MtgjsonDeckObject", "mtgjson5.classes.mtgjson_deck"),
        ("MtgjsonGameFormatsObject", "mtgjson5.classes.mtgjson_game_formats"),
        ("MtgjsonIdentifiersObject", "mtgjson5.classes.mtgjson_identifiers"),
        ("MtgjsonLegalitiesObject", "mtgjson5.classes.mtgjson_legalities"),
        ("MtgjsonPricesObject", "mtgjson5.classes.mtgjson_prices"),
        ("MtgjsonAllPrintingsObject", "mtgjson5.compiled_classes.mtgjson_all_printings"),
        ("MtgjsonStructuresObject", "mtgjson5.compiled_classes.mtgjson_structures"),
        ("MtgjsonEnumValuesObject", "mtgjson5.compiled_classes.mtgjson_enum_values"),
    ]
    
    # Check if user wants specific class
    if len(sys.argv) > 1:
        target_class = sys.argv[1]
        classes_to_test = [(name, module) for name, module in classes_to_test 
                          if target_class.lower() in name.lower()]
        if not classes_to_test:
            print(f"No classes found matching '{target_class}'")
            return
    
    successful = 0
    total = len(classes_to_test)
    
    for class_name, module_path in classes_to_test:
        try:
            # Import the module and get the class
            module = __import__(module_path, fromlist=[class_name])
            cls = getattr(module, class_name)
            
            if explore_class_simple(cls, class_name):
                successful += 1
                
        except ImportError as e:
            print(f"\n❌ Could not import {class_name} from {module_path}: {e}")
        except AttributeError as e:
            print(f"\n❌ Could not find {class_name} in {module_path}: {e}")
        except Exception as e:
            print(f"\n❌ Error with {class_name}: {e}")
    
    print(f"\n📊 RESULTS:")
    print(f"  • Successfully explored: {successful}/{total} classes")
    print(f"  • Failed: {total - successful}/{total} classes")
    
    if successful > 0:
        print(f"\n✅ Exploration complete! You can now use these classes in your code.")
    else:
        print(f"\n⚠️  No classes could be explored. Check dependencies and imports.")

if __name__ == "__main__":
    main() 