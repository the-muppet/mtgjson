import random
import mtgjson_rust


def debug_card_sorting():
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
        ("2a", "a"),
        ("2b", "b"),
        ("3", None),
        ("10", None),
        ("10a", "a"),
        ("10b", "b"),
        ("11", None),
        ("20", None),
        ("", None),
    ]

    test_group = []
    for number, side in correct_order:
        card = mtgjson_rust.MtgjsonCard()
        card.number = number
        card.side = side
        test_group.append(card)

    print("Before sorting:")
    for i, card in enumerate(test_group):
        print(f"  {i}: ({card.number}, {card.side})")

    # Just do one sort to see what happens
    random.shuffle(test_group)
    print("\nAfter shuffle:")
    for i, card in enumerate(test_group):
        print(f"  {i}: ({card.number}, {card.side})")
    
    test_group.sort()
    print("\nAfter sort:")
    for i, card in enumerate(test_group):
        print(f"  {i}: ({card.number}, {card.side})")

    test_group_order = list(map(lambda x: (x.number, x.side), test_group))
    
    print(f"\nExpected: {correct_order}")
    print(f"Got:      {test_group_order}")
    print(f"Match:    {correct_order == test_group_order}")

if __name__ == "__main__":
    debug_card_sorting()