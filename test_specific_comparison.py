import mtgjson_rust

# Test specific comparison that's failing
card1 = mtgjson_rust.MtgjsonCard()
card1.number = "00a"
card1.side = None

card2 = mtgjson_rust.MtgjsonCard()
card2.number = "ap0a" 
card2.side = None

print(f"Testing: '00a' < 'ap0a'")
print(f"Result: {card1 < card2}")
print(f"Should be: False (00a should come after ap0a)")

print(f"\nTesting: 'ap0a' < '00a'")
print(f"Result: {card2 < card1}")
print(f"Should be: True (ap0a should come before 00a)")

# Test with more alphanumeric cards
cards_test = []
test_numbers = ["00", "00a", "ap0a", "gn0a", "ml0b", "mlp0a", "1"]

for num in test_numbers:
    card = mtgjson_rust.MtgjsonCard()
    card.number = num
    card.side = None
    cards_test.append((num, card))

print(f"\nSorting test with: {[num for num, _ in cards_test]}")
sorted_cards = sorted(cards_test, key=lambda x: x[1])
sorted_numbers = [num for num, _ in sorted_cards]
print(f"Result: {sorted_numbers}")
print(f"Expected: ['00', 'ap0a', 'gn0a', 'ml0b', 'mlp0a', '00a', '1']")

# Test individual comparisons to debug
pairs = [
    ("00", "ap0a"),
    ("ap0a", "00a"),
    ("00a", "1"),
]

for num1, num2 in pairs:
    c1 = mtgjson_rust.MtgjsonCard()
    c1.number = num1
    c2 = mtgjson_rust.MtgjsonCard()
    c2.number = num2
    result = c1 < c2
    print(f"'{num1}' < '{num2}': {result}")