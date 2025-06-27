import mtgjson_rust

# Test the actual Rust implementation directly
card1 = mtgjson_rust.MtgjsonCard()
card1.number = "00a"
card1.side = None

card2 = mtgjson_rust.MtgjsonCard()
card2.number = "ap0a"
card2.side = None

print(f"Actual Rust: '00a' < 'ap0a' = {card1 < card2}")
print(f"Actual Rust: 'ap0a' < '00a' = {card2 < card1}")

# Test the full sorting with actual Rust objects
correct_order = [
    ("0", None),
    ("00", None),
    ("ap0a", None),
    ("gn0a", None),
    ("ml0b", None),
    ("mlp0a", None),
    ("00a", None),
    ("1", None),
    ("2", None),
]

test_group = []
for number, side in correct_order:
    card = mtgjson_rust.MtgjsonCard()
    card.number = number
    card.side = side
    test_group.append(card)

# Shuffle them
import random
random.shuffle(test_group)

# Sort them
test_group.sort()

# Get the resulting order
result_order = [(card.number, card.side) for card in test_group]

print(f"\nExpected: {[f'({num}, {side})' for num, side in correct_order]}")
print(f"Got:      {[f'({card.number}, {card.side})' for card in test_group]}")
print(f"Match:    {result_order == correct_order}")

# Test the specific problematic comparison
card_00a = mtgjson_rust.MtgjsonCard()
card_00a.number = "00a"
card_00a.side = None

card_ap0a = mtgjson_rust.MtgjsonCard()
card_ap0a.number = "ap0a"
card_ap0a.side = None

print(f"\nDirect comparison:")
print(f"card_00a < card_ap0a: {card_00a < card_ap0a}")
print(f"card_ap0a < card_00a: {card_ap0a < card_00a}")

# Test with a list of just these two
two_cards = [card_00a, card_ap0a]
two_cards.sort()
print(f"Sorted order of ['00a', 'ap0a']: {[card.number for card in two_cards]}")
print(f"Should be: ['ap0a', '00a']")