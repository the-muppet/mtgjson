#!/usr/bin/env python3
"""
Usage examples for MTGJSON classes (once working)
"""

def example_card_exploration():
    """Example of exploring a card object"""
    print("🎴 Card Object Exploration Example:")
    print("="*50)
    
    try:
        # This would work once the implementation is fixed
        from mtgjson5.classes.mtgjson_card import MtgjsonCardObject
        
        card = MtgjsonCardObject()
        
        # Set some basic properties
        card.name = "Lightning Bolt"
        card.mana_cost = "{R}"
        card.type = "Instant"
        card.power = ""
        card.toughness = ""
        card.rarity = "common"
        
        print(f"Created card: {card.name}")
        print(f"Mana cost: {card.mana_cost}")
        print(f"Type: {card.type}")
        print(f"Rarity: {card.rarity}")
        
        # Test availability
        if hasattr(card, 'availability'):
            card.availability.paper = True
            card.availability.mtgo = True
            card.availability.arena = False
            print(f"Available on: paper={card.availability.paper}, mtgo={card.availability.mtgo}")
        
        # Test identifiers
        if hasattr(card, 'identifiers'):
            card.identifiers.mtgo_id = "12345"
            card.identifiers.scryfall_id = "abcd-1234"
            print(f"MTGO ID: {card.identifiers.mtgo_id}")
        
        return True
        
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        print("💡 Install missing dependencies first")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def example_set_exploration():
    """Example of exploring a set object"""
    print("\n📦 Set Object Exploration Example:")
    print("="*50)
    
    try:
        from mtgjson5.classes.mtgjson_set import MtgjsonSetObject
        
        mtgjson_set = MtgjsonSetObject()
        
        # Set basic properties
        mtgjson_set.name = "Core Set 2021"
        mtgjson_set.code = "M21"
        mtgjson_set.type = "core"
        mtgjson_set.release_date = "2020-07-03"
        
        print(f"Created set: {mtgjson_set.name} ({mtgjson_set.code})")
        print(f"Type: {mtgjson_set.type}")
        print(f"Release date: {mtgjson_set.release_date}")
        print(f"Cards: {len(mtgjson_set.cards)} cards")
        print(f"Tokens: {len(mtgjson_set.tokens)} tokens")
        
        return True
        
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def example_all_printings_exploration():
    """Example of using AllPrintings object"""
    print("\n🗂️ AllPrintings Object Exploration Example:")
    print("="*50)
    
    try:
        from mtgjson5.compiled_classes.mtgjson_all_printings import MtgjsonAllPrintingsObject
        
        all_printings = MtgjsonAllPrintingsObject()
        
        print(f"Created AllPrintings object")
        print(f"Sets loaded: {len(all_printings.all_sets_dict) if hasattr(all_printings, 'all_sets_dict') else 'unknown'}")
        
        # Try to load from file (this is the pattern you'd use)
        test_file = r"C:\Users\rprat\Downloads\betamtgban\allprintings5.json"
        print(f"Would attempt to load from: {test_file}")
        
        # This would be the actual usage:
        # loaded_data = MtgjsonAllPrintingsObject.from_path(test_file)
        # print(f"Loaded {loaded_data.len()} sets")
        
        return True
        
    except ImportError as e:
        print(f"❌ Import failed: {e}")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def example_exploration_workflow():
    """Example workflow for exploring classes"""
    print("\n🔬 Class Exploration Workflow Example:")
    print("="*50)
    
    # Step 1: List available methods
    print("1. Explore class methods:")
    print("   python explore_classes.py --class=MtgjsonCard --detailed")
    
    # Step 2: Test specific class
    print("\n2. Test specific functionality:")
    print("   python interactive_explore.py --class=MtgjsonCard")
    
    # Step 3: Run comprehensive tests
    print("\n3. Run all tests:")
    print("   python simple_class_explorer.py")
    
    # Step 4: Real data testing
    print("\n4. Test with real data:")
    print("   # In Python REPL:")
    print("   from mtgjson_rust import MtgjsonAllPrintings")
    print("   all_printings = MtgjsonAllPrintings.from_path(r'C:\\path\\to\\allprintings5.json')")
    print("   print(f'Loaded {all_printings.len()} sets')")
    print("   codes = all_printings.list_set_codes()")
    print("   print(f'First 10 sets: {codes[:10]}')")

def main():
    """Main function demonstrating usage patterns"""
    
    print("🚀 MTGJSON Class Usage Examples")
    print("="*60)
    print("These examples show how you'd use the classes once they're working.\n")
    
    # Try each example
    examples = [
        example_card_exploration,
        example_set_exploration,
        example_all_printings_exploration,
        example_exploration_workflow
    ]
    
    for example in examples:
        try:
            example()
        except Exception as e:
            print(f"❌ Example failed: {e}")
    
    print("\n" + "="*60)
    print("📝 Summary:")
    print("- These examples will work once dependencies are installed")
    print("- Use the exploration scripts to discover available methods")
    print("- Start with simple classes before moving to complex ones")
    print("- Test with real MTGJSON data files for practical usage")
    
    print("\n🔧 To get started:")
    print("1. Fix Python dependencies: pip install boto3")
    print("2. OR fix Rust compilation errors")
    print("3. Run: python simple_class_explorer.py")
    print("4. Use the working classes in your projects!")

if __name__ == "__main__":
    main() 