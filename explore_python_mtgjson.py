#!/usr/bin/env python3
"""
Script to explore Python MTGJSON5 classes and methods
"""

import inspect
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple

def explore_module_classes(module, module_name: str) -> List[Dict[str, Any]]:
    """Explore all classes in a module"""
    
    classes_info = []
    
    # Get all classes from the module
    for name, obj in inspect.getmembers(module, inspect.isclass):
        if obj.__module__.startswith(module_name):  # Only classes defined in this module
            info = explore_class(obj, name)
            classes_info.append(info)
    
    return classes_info

def explore_class(cls: type, class_name: str) -> Dict[str, Any]:
    """Explore a single class and return information about its methods and attributes"""
    
    info = {
        'name': class_name,
        'module': cls.__module__,
        'docstring': inspect.getdoc(cls) or "No docstring available",
        'methods': [],
        'properties': [],
        'attributes': [],
        'special_methods': [],
        'class_methods': [],
        'static_methods': []
    }
    
    # Get all members of the class
    members = inspect.getmembers(cls)
    
    for name, obj in members:
        # Skip private attributes that start with underscore (but keep special methods like __init__)
        if name.startswith('_') and not (name.startswith('__') and name.endswith('__')):
            continue
            
        try:
            if inspect.ismethod(obj) or inspect.isfunction(obj):
                try:
                    sig = str(inspect.signature(obj))
                except (ValueError, TypeError):
                    sig = "signature unavailable"
                
                method_info = {
                    'name': name,
                    'signature': sig,
                    'docstring': inspect.getdoc(obj) or "No docstring",
                    'is_class_method': inspect.ismethod(obj) and hasattr(obj, '__self__'),
                    'is_static_method': isinstance(inspect.getattr_static(cls, name, None), staticmethod)
                }
                
                if name.startswith('__') and name.endswith('__'):
                    info['special_methods'].append(method_info)
                elif method_info['is_class_method']:
                    info['class_methods'].append(method_info)
                elif method_info['is_static_method']:
                    info['static_methods'].append(method_info)
                else:
                    info['methods'].append(method_info)
                    
            elif isinstance(obj, property):
                prop_info = {
                    'name': name,
                    'docstring': inspect.getdoc(obj) or "No docstring",
                    'has_getter': obj.fget is not None,
                    'has_setter': obj.fset is not None,
                    'has_deleter': obj.fdel is not None
                }
                info['properties'].append(prop_info)
                
            else:
                # Regular attribute
                attr_info = {
                    'name': name,
                    'type': type(obj).__name__,
                    'value': str(obj)[:100] + ('...' if len(str(obj)) > 100 else '') if not callable(obj) else "callable"
                }
                info['attributes'].append(attr_info)
                
        except Exception as e:
            # Some objects might not be introspectable
            attr_info = {
                'name': name,
                'type': "unknown",
                'value': f"Error inspecting: {e}"
            }
            info['attributes'].append(attr_info)
    
    return info

def print_class_info(info: Dict[str, Any], detailed: bool = True) -> None:
    """Print formatted information about a class"""
    
    print(f"\n{'='*80}")
    print(f"CLASS: {info['name']} (module: {info['module']})")
    print(f"{'='*80}")
    
    print(f"\nDOCSTRING:")
    docstring = info['docstring']
    if len(docstring) > 200:
        docstring = docstring[:200] + "..."
    print(f"  {docstring}")
    
    # Methods
    if info['methods']:
        print(f"\nMETHODS ({len(info['methods'])}):")
        for method in sorted(info['methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring" and len(method['docstring']) > 0:
                    doc_short = method['docstring'].split('\n')[0][:60] + '...' if len(method['docstring']) > 60 else method['docstring'].split('\n')[0]
                    print(f"    └─ {doc_short}")
            else:
                print(f"  • {method['name']}")
    
    # Properties  
    if info['properties']:
        print(f"\nPROPERTIES ({len(info['properties'])}):")
        for prop in sorted(info['properties'], key=lambda x: x['name']):
            access = []
            if prop['has_getter']: access.append('get')
            if prop['has_setter']: access.append('set')
            if prop['has_deleter']: access.append('del')
            access_str = '/'.join(access) if access else 'none'
            
            if detailed:
                print(f"  • {prop['name']} ({access_str})")
                if prop['docstring'] != "No docstring" and len(prop['docstring']) > 0:
                    doc_short = prop['docstring'].split('\n')[0][:60] + '...' if len(prop['docstring']) > 60 else prop['docstring'].split('\n')[0]
                    print(f"    └─ {doc_short}")
            else:
                print(f"  • {prop['name']} ({access_str})")
    
    # Class methods
    if info['class_methods']:
        print(f"\nCLASS METHODS ({len(info['class_methods'])}):")
        for method in sorted(info['class_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • @classmethod {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring" and len(method['docstring']) > 0:
                    doc_short = method['docstring'].split('\n')[0][:60] + '...' if len(method['docstring']) > 60 else method['docstring'].split('\n')[0]
                    print(f"    └─ {doc_short}")
            else:
                print(f"  • @classmethod {method['name']}")
    
    # Static methods
    if info['static_methods']:
        print(f"\nSTATIC METHODS ({len(info['static_methods'])}):")
        for method in sorted(info['static_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • @staticmethod {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring" and len(method['docstring']) > 0:
                    doc_short = method['docstring'].split('\n')[0][:60] + '...' if len(method['docstring']) > 60 else method['docstring'].split('\n')[0]
                    print(f"    └─ {doc_short}")
            else:
                print(f"  • @staticmethod {method['name']}")
    
    # Special methods
    if info['special_methods']:
        print(f"\nSPECIAL METHODS ({len(info['special_methods'])}):")
        for method in sorted(info['special_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring" and len(method['docstring']) > 0:
                    doc_short = method['docstring'].split('\n')[0][:60] + '...' if len(method['docstring']) > 60 else method['docstring'].split('\n')[0]
                    print(f"    └─ {doc_short}")
            else:
                print(f"  • {method['name']}")
    
    # Attributes (limit to most interesting ones)
    if info['attributes']:
        interesting_attrs = [attr for attr in info['attributes'] if not attr['name'].startswith('__')]
        if interesting_attrs:
            print(f"\nATTRIBUTES ({len(interesting_attrs)}):")
            for attr in sorted(interesting_attrs, key=lambda x: x['name'])[:10]:  # Limit to first 10
                if detailed:
                    print(f"  • {attr['name']} ({attr['type']}): {attr['value']}")
                else:
                    print(f"  • {attr['name']} ({attr['type']})")
            if len(interesting_attrs) > 10:
                print(f"    ... and {len(interesting_attrs) - 10} more attributes")

def create_summary_table(all_info: List[Dict[str, Any]]) -> None:
    """Create a summary table of all classes"""
    
    print(f"\n{'='*100}")
    print(f"SUMMARY TABLE")
    print(f"{'='*100}")
    
    header = f"{'Class Name':<30} {'Methods':<8} {'Props':<6} {'ClassM':<7} {'StaticM':<8} {'Special':<8} {'Attrs':<6}"
    print(header)
    print('-' * len(header))
    
    for info in sorted(all_info, key=lambda x: x['name']):
        interesting_attrs = [attr for attr in info['attributes'] if not attr['name'].startswith('__')]
        row = (f"{info['name']:<30} "
               f"{len(info['methods']):<8} "
               f"{len(info['properties']):<6} "
               f"{len(info['class_methods']):<7} "
               f"{len(info['static_methods']):<8} "
               f"{len(info['special_methods']):<8} "
               f"{len(interesting_attrs):<6}")
        print(row)

def test_specific_classes():
    """Test specific MTGJSON5 classes with examples"""
    
    print(f"\n🧪 TESTING SPECIFIC CLASSES:")
    print("="*50)
    
    try:
        # Test MtgjsonCardObject
        print(f"\n📄 Testing MtgjsonCardObject:")
        from mtgjson5.classes.mtgjson_card import MtgjsonCardObject
        
        card = MtgjsonCardObject()
        print(f"  ✓ Created MtgjsonCardObject instance")
        
        # Test some properties
        test_props = ['name', 'mana_cost', 'type', 'power', 'toughness', 'rarity']
        for prop in test_props:
            if hasattr(card, prop):
                try:
                    value = getattr(card, prop)
                    print(f"  • {prop}: {value} (type: {type(value).__name__})")
                    
                    # Try setting a test value
                    if isinstance(value, str):
                        setattr(card, prop, f"test_{prop}")
                        new_value = getattr(card, prop)
                        print(f"    └─ Set to: {new_value}")
                except Exception as e:
                    print(f"  ✗ Error with {prop}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonCardObject: {e}")
    
    try:
        # Test MtgjsonSetObject
        print(f"\n📦 Testing MtgjsonSetObject:")
        from mtgjson5.classes.mtgjson_set import MtgjsonSetObject
        
        mtgjson_set = MtgjsonSetObject()
        print(f"  ✓ Created MtgjsonSetObject instance")
        
        # Test some properties
        test_props = ['name', 'code', 'type', 'release_date', 'cards', 'tokens']
        for prop in test_props:
            if hasattr(mtgjson_set, prop):
                try:
                    value = getattr(mtgjson_set, prop)
                    if isinstance(value, list):
                        print(f"  • {prop}: {len(value)} items")
                    else:
                        print(f"  • {prop}: {value} (type: {type(value).__name__})")
                except Exception as e:
                    print(f"  ✗ Error with {prop}: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonSetObject: {e}")
    
    try:
        # Test AllPrintings if available
        print(f"\n🗂️ Testing AllPrintings:")
        from mtgjson5.compiled_classes.mtgjson_all_printings import MtgjsonAllPrintingsObject
        
        all_printings = MtgjsonAllPrintingsObject()
        print(f"  ✓ Created MtgjsonAllPrintingsObject instance")
        
        # Test some methods
        test_methods = ['get_files_to_build', 'iterate_all_sets']
        for method in test_methods:
            if hasattr(all_printings, method):
                try:
                    result = getattr(all_printings, method)()
                    if isinstance(result, list):
                        print(f"  • {method}(): {len(result)} items")
                    else:
                        print(f"  • {method}(): {result}")
                except Exception as e:
                    print(f"  ✗ Error with {method}: {e}")
        
        # Try loading from path if file exists
        test_file = r"C:\Users\rprat\Downloads\betamtgban\allprintings5.json"
        if Path(test_file).exists():
            print(f"  • Found test file: {test_file}")
            try:
                loaded = MtgjsonAllPrintingsObject()
                # Note: Python version might not have from_path method
                print(f"    ✓ File exists for potential testing")
            except Exception as e:
                print(f"    ✗ Error loading file: {e}")
        
    except Exception as e:
        print(f"  ✗ Error testing MtgjsonAllPrintingsObject: {e}")

def main():
    """Main exploration function"""
    
    # Check command line arguments
    detailed = '--detailed' in sys.argv or '-d' in sys.argv
    summary_only = '--summary' in sys.argv or '-s' in sys.argv
    test_only = '--test' in sys.argv or '-t' in sys.argv
    class_filter = None
    
    for arg in sys.argv[1:]:
        if arg.startswith('--class='):
            class_filter = arg.split('=', 1)[1]
        elif arg.startswith('-c'):
            if '=' in arg:
                class_filter = arg.split('=', 1)[1]
    
    if '--help' in sys.argv or '-h' in sys.argv:
        print("""
Python MTGJSON5 Class Explorer

Usage: python explore_python_mtgjson.py [options]

Options:
  -h, --help              Show this help message
  -d, --detailed          Show detailed information (signatures, docstrings)
  -s, --summary           Show only summary table
  -t, --test              Show only test results
  -c CLASS, --class=CLASS Explore only specific class
  
Examples:
  python explore_python_mtgjson.py                           # Explore all classes (brief)
  python explore_python_mtgjson.py --detailed                # Explore all classes (detailed)
  python explore_python_mtgjson.py --summary                 # Show only summary table
  python explore_python_mtgjson.py --test                    # Show only test results
  python explore_python_mtgjson.py --class=MtgjsonCardObject # Explore only MtgjsonCardObject
        """)
        return
    
    print("🚀 Python MTGJSON5 Module Class Explorer")
    print("="*50)
    
    try:
        import mtgjson5
        print(f"✓ Successfully imported mtgjson5 module")
    except ImportError as e:
        print(f"✗ Failed to import mtgjson5 module: {e}")
        return
    
    if test_only:
        test_specific_classes()
        return
    
    # Explore classes module
    all_info = []
    
    try:
        print(f"\n📋 Exploring mtgjson5.classes...")
        import mtgjson5.classes
        classes_info = explore_module_classes(mtgjson5.classes, 'mtgjson5.classes')
        all_info.extend(classes_info)
        
        print(f"\n📋 Exploring mtgjson5.compiled_classes...")
        import mtgjson5.compiled_classes
        compiled_classes_info = explore_module_classes(mtgjson5.compiled_classes, 'mtgjson5.compiled_classes')
        all_info.extend(compiled_classes_info)
        
    except Exception as e:
        print(f"✗ Error exploring modules: {e}")
        return
    
    # Filter by class if specified
    if class_filter:
        all_info = [info for info in all_info if class_filter.lower() in info['name'].lower()]
        if not all_info:
            print(f"No classes found matching '{class_filter}'")
            return
    
    # Print class information
    if not summary_only:
        for info in sorted(all_info, key=lambda x: x['name']):
            print_class_info(info, detailed)
    
    # Print summary
    if not class_filter:
        create_summary_table(all_info)
        
        print(f"\n📊 EXPLORATION RESULTS:")
        print(f"  • Total classes explored: {len(all_info)}")
        
        # Group by module
        modules = {}
        for info in all_info:
            module = info['module'].split('.')[-2] if '.' in info['module'] else info['module']
            if module not in modules:
                modules[module] = []
            modules[module].append(info['name'])
        
        for module, classes in modules.items():
            print(f"  • {module}: {len(classes)} classes")
    
    print(f"\n🧪 Run with --test flag to see specific class testing examples")
    print(f"\n✅ Exploration complete!")

if __name__ == "__main__":
    main() 