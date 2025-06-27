def original_card_lt(self_number, self_side, other_number, other_side):
    """Original Python logic from mtgjson_card.py"""
    if self_number == other_number:
        return (self_side or "") < (other_side or "")

    self_side = self_side or ""
    other_side = other_side or ""

    self_number_clean = "".join(x for x in self_number if x.isdigit()) or "100000"
    self_number_clean_int = int(self_number_clean)

    other_number_clean = "".join(x for x in other_number if x.isdigit()) or "100000"
    other_number_clean_int = int(other_number_clean)

    # Check if both numbers are pure digits
    self_is_digit = self_number == self_number_clean
    other_is_digit = other_number == other_number_clean

    if self_is_digit and other_is_digit:
        if self_number_clean_int == other_number_clean_int:
            if len(self_number_clean) != len(other_number_clean):
                return len(self_number_clean) < len(other_number_clean)
            return self_side < other_side
        return self_number_clean_int < other_number_clean_int

    if self_is_digit:
        if self_number_clean_int == other_number_clean_int:
            return True
        return self_number_clean_int < other_number_clean_int

    if other_is_digit:
        if self_number_clean_int == other_number_clean_int:
            return False
        return self_number_clean_int < other_number_clean_int

    # Case 4: Neither is pure digit
    # First check if digit strings are identical
    if self_number_clean == other_number_clean:
        if not self_side and not other_side:
            return self_number < other_number
        return self_side < other_side

    # Then check if integer values are the same but digit strings differ
    if self_number_clean_int == other_number_clean_int:
        if len(self_number_clean) != len(other_number_clean):
            return len(self_number_clean) < len(other_number_clean)
        return self_side < other_side

    return self_number_clean_int < other_number_clean_int

# Test the critical comparison
test_cases = [
    ("00a", None, "ap0a", None),
    ("ap0a", None, "00a", None),
    ("00", None, "ap0a", None),
    ("ap0a", None, "1", None),
]

print("Testing embedded Python logic:")
for self_num, self_side, other_num, other_side in test_cases:
    result = original_card_lt(self_num, self_side, other_num, other_side)
    print(f"'{self_num}' < '{other_num}': {result}")

# Now test against our Rust implementation
import mtgjson_rust

print("\nTesting Rust implementation:")
for self_num, self_side, other_num, other_side in test_cases:
    card1 = mtgjson_rust.MtgjsonCard()
    card1.number = self_num
    card1.side = self_side
    
    card2 = mtgjson_rust.MtgjsonCard()
    card2.number = other_num
    card2.side = other_side
    
    result = card1.__lt__(card2)
    print(f"'{self_num}' < '{other_num}': {result}")

print("\nComparison match:")
for self_num, self_side, other_num, other_side in test_cases:
    python_result = original_card_lt(self_num, self_side, other_num, other_side)
    
    card1 = mtgjson_rust.MtgjsonCard()
    card1.number = self_num
    card1.side = self_side
    
    card2 = mtgjson_rust.MtgjsonCard()
    card2.number = other_num
    card2.side = other_side
    
    rust_result = card1.__lt__(card2)
    match = python_result == rust_result
    
    print(f"'{self_num}' < '{other_num}': Python={python_result}, Rust={rust_result}, Match={match}")