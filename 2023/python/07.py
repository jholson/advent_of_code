import sys

from collections import defaultdict
from dataclasses import dataclass
from typing import DefaultDict, Sequence, Tuple

def main() -> None:
    # result = part1()
    result = part2()

    print(result)


@dataclass(frozen=True, slots=True)
class Hand(object):
    cards: str
    bid: int

    """
    First item of tuple, highest wins:
    7 = five of a kind
    6 = four of a kind
    5 = full house
    4 = three of a kind
    3 = two pair
    2 = one pair
    1 = high card

    Remaining 5 items of tuple, card ranks, highest wins:
    14 = A
    13 = K
    12 = Q
    11 = J
    10 = T
    9 = 9
    ...
    2 = 2
    """
    sort_key: Tuple[int, int, int, int, int, int]

def parse_input(jokers: bool) -> Sequence[Hand]:
    hands = []
    for line in sys.stdin:
        line = line.strip()

        cards = line[:5]
        bid = int(line[6:])
        sort_key = get_sort_key(cards, jokers=jokers)

        hands.append(Hand(cards=cards, bid=bid, sort_key=sort_key))

    return hands


def get_sort_key(cards: str, jokers: bool) -> Tuple[int, int, int, int, int, int]:
    """
    Return a sort key for the cards
    """
    hand_type = get_hand_type(cards, jokers=jokers)
    
    return (hand_type, ) + tuple(get_card_rank(c, jokers=jokers) for c in cards)

def get_hand_type(cards: str, jokers: bool) -> int:
    """
    jokers: consider J as jokers for hand ranking
    """
    dupes: DefaultDict[str, int] = defaultdict(int)

    for card in cards:
        dupes[card] += 1

    if jokers:
        jokers = dupes["J"]
        del dupes["J"]

    vals_desc = list(dupes.values())
    vals_desc.sort(reverse=True)
    
    if jokers:
        if len(vals_desc) == 0:
            # Five of a kind (five jokers)
            return 7

        # Just add the jokers onto the largest group, this is the best outcome for any hand
        vals_desc[0] += jokers

    if vals_desc[0] == 5:
        # Five of a kind
        return 7
    elif vals_desc[0] == 4:
        # Four of a kind
        return 6
    elif vals_desc[0] == 3:
        if vals_desc[1] == 2:
            # Full house
            return 5
        else:
            # Three of a kind
            return 4
    elif vals_desc[0] == 2:
        if vals_desc[1] == 2:
            # Two pair
            return 3
        else:
            # One pair
            return 2
    else:
        # High card
        return 1

CARD_RANK = {
    'A': 14,
    'K': 13,
    'Q': 12,
    'J': 11,
    'T': 10,
}

def get_card_rank(card: str, jokers: bool) -> int:
    if jokers and card == 'J':
        return 1
    return CARD_RANK.get(card) or int(card)

def get_reward_total(hands: Sequence[Hand]) -> int:
    hands.sort(key=lambda h: h.sort_key)

    total = 0
    for idx, hand in enumerate(hands):
        rank = idx + 1
        total += rank * hand.bid

    return total

def part1() -> int:
    hands = parse_input(jokers=False)

    return get_reward_total(hands)

def part2() -> int:
    hands = parse_input(jokers=True)

    return get_reward_total(hands)


if __name__ == '__main__':
    main()
