import mtgjson_rust

# Create test cards
card_00a = mtgjson_rust.MtgjsonCard()
card_00a.number = "00a"
card_00a.side = None

card_ap0a = mtgjson_rust.MtgjsonCard()
card_ap0a.number = "ap0a" 
card_ap0a.side = None

print("=== DEBUGGING RUST COMPARISON ===")
print(f"card_00a.number: '{card_00a.number}'")
print(f"card_ap0a.number: '{card_ap0a.number}'")
print()

print(f"card_00a < card_ap0a: {card_00a < card_ap0a}")
print(f"card_ap0a < card_00a: {card_ap0a < card_00a}")
print()

# Test the digit extraction manually in Python to verify
def extract_digits(number):
    return "".join(c for c in number if c.isdigit()) or "100000"

def test_python_logic():
    self_number = "00a"
    other_number = "ap0a"
    
    self_number_clean = extract_digits(self_number)
    other_number_clean = extract_digits(other_number)
    
    print(f"Python digit extraction:")
    print(f"  '{self_number}' -> '{self_number_clean}' (len: {len(self_number_clean)})")
    print(f"  '{other_number}' -> '{other_number_clean}' (len: {len(other_number_clean)})")
    
    self_int = int(self_number_clean)
    other_int = int(other_number_clean)
    
    print(f"  Integer values: {self_int} vs {other_int}")
    
    if self_int == other_int:
        print(f"  Same integer value, comparing lengths: {len(self_number_clean)} vs {len(other_number_clean)}")
        length_comparison = len(self_number_clean) < len(other_number_clean)
        print(f"  Length comparison result: {len(self_number_clean)} < {len(other_number_clean)} = {length_comparison}")
        return length_comparison
    
    return self_int < other_int

test_python_logic()

# Test more cases to see the pattern
test_cases = [
    ("00a", "ap0a"),
    ("ap0a", "00a"), 
    ("00", "ap0a"),
    ("ap0a", "1"),
    ("1", "2"),
    ("10a", "10b"),
]

print(f"\n=== COMPREHENSIVE RUST COMPARISON TESTS ===")
for num1, num2 in test_cases:
    card1 = mtgjson_rust.MtgjsonCard()
    card1.number = num1
    card1.side = None
    
    card2 = mtgjson_rust.MtgjsonCard()
    card2.number = num2
    card2.side = None
    
    result = card1 < card2
    print(f"'{num1}' < '{num2}': {result}")