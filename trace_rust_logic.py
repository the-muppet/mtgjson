def trace_rust_logic(self_number, self_side, other_number, other_side):
    """Trace our current Rust implementation logic step by step"""
    print(f"\n=== RUST LOGIC: ('{self_number}', {self_side}) < ('{other_number}', {other_side}) ===")
    
    if self_number == other_number:
        result = (self_side or "") < (other_side or "")
        print(f"Same number, comparing sides: {result}")
        return result

    self_side = self_side or ""
    other_side = other_side or ""

    self_number_clean = "".join(x for x in self_number if x.isdigit()) or "100000"
    self_number_clean_int = int(self_number_clean)

    other_number_clean = "".join(x for x in other_number if x.isdigit()) or "100000"
    other_number_clean_int = int(other_number_clean)

    print(f"self_number_clean: '{self_number_clean}' (int: {self_number_clean_int}, len: {len(self_number_clean)})")
    print(f"other_number_clean: '{other_number_clean}' (int: {other_number_clean_int}, len: {len(other_number_clean)})")
    
    self_is_digit = self_number == self_number_clean
    other_is_digit = other_number == other_number_clean
    
    print(f"self_is_digit: {self_is_digit}")
    print(f"other_is_digit: {other_is_digit}")

    # Case 1: Both are pure digits
    if self_is_digit and other_is_digit:
        print("RUST CASE 1: Both are pure digits")
        if self_number_clean_int == other_number_clean_int:
            if len(self_number_clean) != len(other_number_clean):
                result = len(self_number_clean) < len(other_number_clean)
                print(f"  Same int, different lengths: {len(self_number_clean)} < {len(other_number_clean)} = {result}")
                return result
            result = self_side < other_side
            print(f"  Same int and length, comparing sides: '{self_side}' < '{other_side}' = {result}")
            return result
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different ints: {self_number_clean_int} < {other_number_clean_int} = {result}")
        return result

    # Case 2: Self is pure digit, other is not
    if self_is_digit:
        print("RUST CASE 2: Self is pure digit, other is not")
        if self_number_clean_int == other_number_clean_int:
            print(f"  Same int value ({self_number_clean_int}), pure digit wins: True")
            return True
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different int values: {self_number_clean_int} < {other_number_clean_int} = {result}")
        return result

    # Case 3: Other is pure digit, self is not
    if other_is_digit:
        print("RUST CASE 3: Other is pure digit, self is not")
        if self_number_clean_int == other_number_clean_int:
            print(f"  Same int value ({self_number_clean_int}), pure digit wins: False")
            return False
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different int values: {self_number_clean_int} < {other_number_clean_int} = {result}")
        return result

    # Case 4: Neither is pure digit
    print("RUST CASE 4: Neither is pure digit")
    
    # Check if digit strings are identical
    if self_number_clean == other_number_clean:
        print(f"  Same digit strings: '{self_number_clean}' == '{other_number_clean}'")
        if not self_side and not other_side:
            result = self_number < other_number
            print(f"  No sides, lexical comparison: '{self_number}' < '{other_number}' = {result}")
            return result
        result = self_side < other_side
        print(f"  Comparing sides: '{self_side}' < '{other_side}' = {result}")
        return result

    # Check if integer values are the same but digit strings differ
    if self_number_clean_int == other_number_clean_int:
        print(f"  Same int values: {self_number_clean_int}")
        if len(self_number_clean) != len(other_number_clean):
            result = len(self_number_clean) < len(other_number_clean)
            print(f"  Different digit lengths: {len(self_number_clean)} < {len(other_number_clean)} = {result}")
            return result
        result = self_side < other_side
        print(f"  Same digit length, comparing sides: '{self_side}' < '{other_side}' = {result}")
        return result

    # Finally compare integer values
    result = self_number_clean_int < other_number_clean_int
    print(f"  Different int values: {self_number_clean_int} < {other_number_clean_int} = {result}")
    return result

# Test the failing case
test_cases = [
    ("00a", None, "ap0a", None),
    ("ap0a", None, "00a", None),
]

for self_num, self_side, other_num, other_side in test_cases:
    result_rust = trace_rust_logic(self_num, self_side, other_num, other_side)
    print(f"RUST RESULT: ('{self_num}', {self_side}) < ('{other_num}', {other_side}) = {result_rust}")
    print()