import sys
sys.path.append('.')

# Try to import the original Python implementation
try:
    from mtgjson5.classes.mtgjson_card import MtgjsonCardObject
    print("✅ Successfully imported original Python MtgjsonCardObject")
    
    # Test the original Python sorting
    card1 = MtgjsonCardObject()
    card1.number = "00a"
    card1.side = None
    
    card2 = MtgjsonCardObject()
    card2.number = "ap0a"
    card2.side = None
    
    print(f"Original Python: '00a' < 'ap0a' = {card1 < card2}")
    print(f"Original Python: 'ap0a' < '00a' = {card2 < card1}")
    
    # Test with more cards
    test_cards = []
    for num in ["00", "00a", "ap0a", "gn0a", "ml0b", "mlp0a", "1"]:
        card = MtgjsonCardObject()
        card.number = num
        card.side = None
        test_cards.append((num, card))
    
    sorted_cards = sorted(test_cards, key=lambda x: x[1])
    sorted_numbers = [num for num, _ in sorted_cards]
    print(f"Original Python sort result: {sorted_numbers}")
    
except ImportError as e:
    print(f"❌ Could not import original Python implementation: {e}")
    print("This is expected in our test environment")
    
    # Let's manually test the Python algorithm
    def test_python_lt(self_number, self_side, other_number, other_side):
        if self_number == other_number:
            return (self_side or "") < (other_side or "")

        self_side = self_side or ""
        other_side = other_side or ""

        self_number_clean = "".join(x for x in self_number if x.isdigit()) or "100000"
        self_number_clean_int = int(self_number_clean)

        other_number_clean = "".join(x for x in other_number if x.isdigit()) or "100000"
        other_number_clean_int = int(other_number_clean)

        if self_number == self_number_clean and other_number == other_number_clean:
            if self_number_clean_int == other_number_clean_int:
                if len(self_number_clean) != len(other_number_clean):
                    return len(self_number_clean) < len(other_number_clean)
                return self_side < other_side
            return self_number_clean_int < other_number_clean_int

        if self_number == self_number_clean:
            if self_number_clean_int == other_number_clean_int:
                return True
            return self_number_clean_int < other_number_clean_int

        if other_number == other_number_clean:
            if self_number_clean_int == other_number_clean_int:
                return False
            return self_number_clean_int < other_number_clean_int

        if self_number_clean == other_number_clean:
            if not self_side and not other_side:
                return self_number < other_number
            return self_side < other_side

        if self_number_clean_int == other_number_clean_int:
            if len(self_number_clean) != len(other_number_clean):
                return len(self_number_clean) < len(other_number_clean)
            return self_side < other_side

        return self_number_clean_int < other_number_clean_int
    
    print(f"Manual Python: '00a' < 'ap0a' = {test_python_lt('00a', None, 'ap0a', None)}")
    print(f"Manual Python: 'ap0a' < '00a' = {test_python_lt('ap0a', None, '00a', None)}")
    
    # Manual sort
    test_numbers = ["00", "00a", "ap0a", "gn0a", "ml0b", "mlp0a", "1"]
    
    # Simple bubble sort using our comparison
    for i in range(len(test_numbers)):
        for j in range(i + 1, len(test_numbers)):
            if not test_python_lt(test_numbers[i], None, test_numbers[j], None):
                test_numbers[i], test_numbers[j] = test_numbers[j], test_numbers[i]
    
    print(f"Manual Python sort result: {test_numbers}")