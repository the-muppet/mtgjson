def python_card_lt(self_number, self_side, other_number, other_side):
    """
    Exact port of Python __lt__ logic for debugging
    """
    print(f"Comparing ({self_number}, {self_side}) < ({other_number}, {other_side})")
    
    if self_number == other_number:
        result = (self_side or "") < (other_side or "")
        print(f"  Same number, comparing sides: {result}")
        return result

    self_side = self_side or ""
    other_side = other_side or ""

    self_number_clean = "".join(x for x in self_number if x.isdigit()) or "100000"
    self_number_clean_int = int(self_number_clean)

    other_number_clean = "".join(x for x in other_number if x.isdigit()) or "100000"
    other_number_clean_int = int(other_number_clean)

    print(f"  self_number_clean: '{self_number_clean}' ({self_number_clean_int})")
    print(f"  other_number_clean: '{other_number_clean}' ({other_number_clean_int})")

    if self_number == self_number_clean and other_number == other_number_clean:
        print("  Both are pure digits")
        if self_number_clean_int == other_number_clean_int:
            if len(self_number_clean) != len(other_number_clean):
                result = len(self_number_clean) < len(other_number_clean)
                print(f"  Same int, different lengths: {result}")
                return result
            result = self_side < other_side
            print(f"  Same int and length, comparing sides: {result}")
            return result
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different ints: {result}")
        return result

    if self_number == self_number_clean:
        print("  Self is pure digit, other is not")
        if self_number_clean_int == other_number_clean_int:
            print(f"  Same int value, self wins: True")
            return True
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different int values: {result}")
        return result

    if other_number == other_number_clean:
        print("  Other is pure digit, self is not")
        if self_number_clean_int == other_number_clean_int:
            print(f"  Same int value, other wins: False")
            return False
        result = self_number_clean_int < other_number_clean_int
        print(f"  Different int values: {result}")
        return result

    print("  Neither is pure digit")
    if self_number_clean == other_number_clean:
        if not self_side and not other_side:
            result = self_number < other_number
            print(f"  Same digits, no sides, lexical: {result}")
            return result
        result = self_side < other_side
        print(f"  Same digits, comparing sides: {result}")
        return result

    if self_number_clean_int == other_number_clean_int:
        if len(self_number_clean) != len(other_number_clean):
            result = len(self_number_clean) < len(other_number_clean)
            print(f"  Same int, different digit lengths: {result}")
            return result
        result = self_side < other_side
        print(f"  Same int and digit length, comparing sides: {result}")
        return result

    result = self_number_clean_int < other_number_clean_int
    print(f"  Different int values: {result}")
    return result

# Test the specific case that's failing
print("Testing '00a' < 'ap0a':")
result1 = python_card_lt("00a", None, "ap0a", None)
print(f"Result: {result1}")
print()

print("Testing 'ap0a' < '00a':")
result2 = python_card_lt("ap0a", None, "00a", None)
print(f"Result: {result2}")
print()

# Test a few more cases
print("Testing '0' < 'ap0a':")
result3 = python_card_lt("0", None, "ap0a", None)
print(f"Result: {result3}")
print()

print("Testing 'ap0a' < '1':")
result4 = python_card_lt("ap0a", None, "1", None)
print(f"Result: {result4}")
print()