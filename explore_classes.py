#!/usr/bin/env python3
"""
Script to explore all methods and attributes of MTGJSON Rust classes
"""

import inspect
import sys
from typing import Any, Dict, List, Tuple

def explore_class(cls: type, class_name: str) -> Dict[str, Any]:
    """Explore a single class and return information about its methods and attributes"""
    
    info = {
        'name': class_name,
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
                method_info = {
                    'name': name,
                    'signature': str(inspect.signature(obj)) if hasattr(inspect, 'signature') else "signature unavailable",
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
                    'value': str(obj) if not callable(obj) else "callable"
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
    print(f"CLASS: {info['name']}")
    print(f"{'='*80}")
    
    print(f"\nDOCSTRING:")
    print(f"  {info['docstring']}")
    
    # Methods
    if info['methods']:
        print(f"\nMETHODS ({len(info['methods'])}):")
        for method in sorted(info['methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring":
                    print(f"    └─ {method['docstring']}")
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
                if prop['docstring'] != "No docstring":
                    print(f"    └─ {prop['docstring']}")
            else:
                print(f"  • {prop['name']} ({access_str})")
    
    # Class methods
    if info['class_methods']:
        print(f"\nCLASS METHODS ({len(info['class_methods'])}):")
        for method in sorted(info['class_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • @classmethod {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring":
                    print(f"    └─ {method['docstring']}")
            else:
                print(f"  • @classmethod {method['name']}")
    
    # Static methods
    if info['static_methods']:
        print(f"\nSTATIC METHODS ({len(info['static_methods'])}):")
        for method in sorted(info['static_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • @staticmethod {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring":
                    print(f"    └─ {method['docstring']}")
            else:
                print(f"  • @staticmethod {method['name']}")
    
    # Special methods
    if info['special_methods']:
        print(f"\nSPECIAL METHODS ({len(info['special_methods'])}):")
        for method in sorted(info['special_methods'], key=lambda x: x['name']):
            if detailed:
                print(f"  • {method['name']}{method['signature']}")
                if method['docstring'] != "No docstring":
                    print(f"    └─ {method['docstring']}")
            else:
                print(f"  • {method['name']}")
    
    # Attributes
    if info['attributes']:
        print(f"\nATTRIBUTES ({len(info['attributes'])}):")
        for attr in sorted(info['attributes'], key=lambda x: x['name']):
            if detailed:
                print(f"  • {attr['name']} ({attr['type']}): {attr['value']}")
            else:
                print(f"  • {attr['name']} ({attr['type']})")

def create_summary_table(all_info: List[Dict[str, Any]]) -> None:
    """Create a summary table of all classes"""
    
    print(f"\n{'='*100}")
    print(f"SUMMARY TABLE")
    print(f"{'='*100}")
    
    header = f"{'Class Name':<25} {'Methods':<8} {'Props':<6} {'ClassM':<7} {'StaticM':<8} {'Special':<8} {'Attrs':<6}"
    print(header)
    print('-' * len(header))
    
    for info in sorted(all_info, key=lambda x: x['name']):
        row = (f"{info['name']:<25} "
               f"{len(info['methods']):<8} "
               f"{len(info['properties']):<6} "
               f"{len(info['class_methods']):<7} "
               f"{len(info['static_methods']):<8} "
               f"{len(info['special_methods']):<8} "
               f"{len(info['attributes']):<6}")
        print(row)

def main():
    """Main exploration function"""
    
    # Check command line arguments
    detailed = '--detailed' in sys.argv or '-d' in sys.argv
    summary_only = '--summary' in sys.argv or '-s' in sys.argv
    class_filter = None
    
    for arg in sys.argv[1:]:
        if arg.startswith('--class='):
            class_filter = arg.split('=', 1)[1]
        elif arg.startswith('-c'):
            if '=' in arg:
                class_filter = arg.split('=', 1)[1]
            else:
                # Next argument should be the class name
                try:
                    idx = sys.argv.index(arg)
                    if idx + 1 < len(sys.argv):
                        class_filter = sys.argv[idx + 1]
                except ValueError:
                    pass
    
    if '--help' in sys.argv or '-h' in sys.argv:
        print("""
MTGJSON Rust Class Explorer

Usage: python explore_classes.py [options]

Options:
  -h, --help              Show this help message
  -d, --detailed          Show detailed information (signatures, docstrings)
  -s, --summary           Show only summary table
  -c CLASS, --class=CLASS Explore only specific class
  
Examples:
  python explore_classes.py                           # Explore all classes (brief)
  python explore_classes.py --detailed                # Explore all classes (detailed)
  python explore_classes.py --summary                 # Show only summary table
  python explore_classes.py --class=MtgjsonCard       # Explore only MtgjsonCard
  python explore_classes.py -d -c MtgjsonSet          # Detailed view of MtgjsonSet
        """)
        return
    
    # List of all available classes
    class_names = [
        'JsonValue', 'MtgjsonAllIdentifiers', 'MtgjsonAllPrintings', 'MtgjsonAtomicCards', 
        'MtgjsonCard', 'MtgjsonCardTypes', 'MtgjsonCompiledList', 'MtgjsonDeck', 
        'MtgjsonDeckHeader', 'MtgjsonDeckList', 'MtgjsonEnumValues', 'MtgjsonForeignData', 
        'MtgjsonGameFormats', 'MtgjsonIdentifiers', 'MtgjsonKeywords', 'MtgjsonLeadershipSkills', 
        'MtgjsonLegalities', 'MtgjsonMeta', 'MtgjsonPrices', 'MtgjsonPurchaseUrls', 
        'MtgjsonRelatedCards', 'MtgjsonRuling', 'MtgjsonSealedProduct', 'MtgjsonSet', 
        'MtgjsonSetList', 'MtgjsonStructures', 'MtgjsonTcgplayerSkus', 'MtgjsonTranslations', 
        'OutputGenerator', 'ParallelIterator', 'ParallelProcessor', 'PriceBuilder', 
        'SealedProductCategory', 'SealedProductSubtype'
    ]
    
    if class_filter:
        if class_filter in class_names:
            class_names = [class_filter]
        else:
            print(f"Error: Class '{class_filter}' not found in available classes.")
            print(f"Available classes: {', '.join(class_names)}")
            return
    
    print("MTGJSON Rust Module Class Explorer")
    print("="*50)
    
    try:
        import mtgjson_rust
        print(f"✓ Successfully imported mtgjson_rust module")
    except ImportError as e:
        print(f"✗ Failed to import mtgjson_rust module: {e}")
        print("Make sure the module is built and in your Python path")
        return
    
    all_info = []
    available_classes = []
    missing_classes = []
    
    # Explore each class
    for class_name in class_names:
        try:
            cls = getattr(mtgjson_rust, class_name)
            available_classes.append(class_name)
            
            if not summary_only:
                print(f"\n📋 Exploring {class_name}...")
            
            info = explore_class(cls, class_name)
            all_info.append(info)
            
            if not summary_only:
                print_class_info(info, detailed)
                
        except AttributeError:
            missing_classes.append(class_name)
            if not summary_only:
                print(f"\n❌ Class {class_name} not found in mtgjson_rust module")
    
    # Print summary
    if not class_filter:
        create_summary_table(all_info)
        
        print(f"\n📊 EXPLORATION RESULTS:")
        print(f"  • Total classes explored: {len(available_classes)}")
        print(f"  • Classes found: {len(available_classes)}")
        print(f"  • Classes missing: {len(missing_classes)}")
        
        if missing_classes:
            print(f"  • Missing classes: {', '.join(missing_classes)}")
    
    print(f"\n✅ Exploration complete!")

if __name__ == "__main__":
    main() 