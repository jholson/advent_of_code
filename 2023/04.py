import re
import sys

from collections import defaultdict
from dataclasses import dataclass
from typing import DefaultDict, Sequence, Set

def main():
    # result = part1()
    result = part2()

    print(result)

@dataclass()
class Card(object):
    card_id: int
    winning_nums: Set[int]
    nums: Set[int]

def parse_input() -> Sequence[Card]:
    """
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    """
    cards = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue

        line = line.removeprefix("Card ")
        card_id_str, line = line.split(": ", 1)
        card_id = int(card_id_str)

        winning_nums, nums = [
            {
                int(d)
                for d in re.split(r' +', s.strip())
            }
            for s in line.split(" | ", 1)
        ]

        cards.append(Card(card_id=card_id, winning_nums=winning_nums, nums=nums))

    return cards
        

def part1() -> int:
    cards = parse_input()

    total = 0
    for card in cards:
        # Set intersection
        matching_nums = card.winning_nums & card.nums
        if matching_nums:
            total += 2 ** (len(matching_nums) - 1)

    return total


def part2() -> int:
    cards = parse_input()

    # Calculate card_id -> number of copies
    copies: DefaultDict[int, int] = defaultdict(lambda: 1)
    for card in cards:
        # Set intersection
        matches = len(card.winning_nums & card.nums)

        # Don't worry about being out of range here, we'll check card ID range later
        for x in range(card.card_id + 1, card.card_id + 1 + matches):
            copies[x] += copies[card.card_id]

    return sum(copies[x + 1] for x in range(cards[-1].card_id))

if __name__ == '__main__':
    main()
