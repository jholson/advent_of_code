import re
import sys

from collections import defaultdict
from dataclasses import dataclass
from typing import DefaultDict, Set, Sequence, Tuple

def main():
    # result = part1()
    result = part2()

    print(result)


@dataclass(frozen=True)
class Symbol(object):
    # Single character string
    symbol: str
    
    # (index of line, index of symbol within line)
    pos: Tuple[int, int]


def find_symbols_adjacent_to_digits(
    lines: Sequence[str],
    line_idx: int,
    start_idx: int,
    end_idx: int,
) -> Sequence[Symbol]:
    """
    Given a string of digits within a given line, spanning from start_idx to end_idx within the
    line, return whether or not the string of digits is adjacent to a symbol (excluding periods)
    within the greater sequence of lines

    start_idx is inclusive, end_idx is exclusive
    """

    # (index of line, inclusive start index within line, exclusive end index within line)
    strs_to_check: List[Tuple[int, int, int]] = []

    # Find start/end index of bounding box for string checks
    # box_start_idx is inclusive, box_end_idx is exclusive
    line_len = len(lines[0])
    box_start_idx = max(0, start_idx - 1)
    box_end_idx = min(line_len, end_idx + 1)

    # Line before
    if line_idx > 0:
        strs_to_check.append((line_idx - 1, box_start_idx, box_end_idx))

    # Line after
    if line_idx < line_len - 1:
        strs_to_check.append((line_idx + 1, box_start_idx, box_end_idx))

    # Same line, character before
    if box_start_idx < start_idx:
        strs_to_check.append((line_idx, box_start_idx, box_start_idx + 1))

    # Same line, character after
    if box_end_idx > end_idx:
        strs_to_check.append((line_idx, box_end_idx - 1, box_end_idx))

    symbols = []
    pattern = re.compile(r'[^0-9.]')
    for check_line_idx, check_start_idx, check_end_idx in strs_to_check:
        line = lines[check_line_idx]
        for match in pattern.finditer(line, check_start_idx, check_end_idx):
            pos = (check_line_idx, match.span()[0])
            symbol = Symbol(symbol=match.group(), pos=pos)
            symbols.append(symbol)

    return symbols
    

def part1() -> int:
    """
    1. Locate a string of digits within a line (start and end indexes)
    2. Look for symbols in the "bounding box" of the digit string, ignoring the ones that don't
        exit due to boundary conditions:
        - Line before, from start-1 to end+1
        - Line after, from start-1 to end+1
        - Single-character string immediately before digit string
        - Single-character string immediately after digit string
    3. Existence of symbols in any of those means this is a part number
    """
    lines = [line.strip() for line in sys.stdin]

    total = 0

    for line_idx, line in enumerate(lines):
        for match in re.finditer(r'\d+', line):
            start_idx, end_idx = match.span()
            number = int(match.group())
            if find_symbols_adjacent_to_digits(lines, line_idx, start_idx, end_idx):
                total += number

    return total


def part2() -> int:
    lines = [line.strip() for line in sys.stdin]

    gears_to_numbers: DefaultDict[Symbol, List[int]] = defaultdict(list)
    for line_idx, line in enumerate(lines):
        for match in re.finditer(r'\d+', line):
            start_idx, end_idx = match.span()
            number = int(match.group())
            symbols = find_symbols_adjacent_to_digits(lines, line_idx, start_idx, end_idx)
            
            gears = [s for s in symbols if s.symbol == '*']
            for gear in gears:
                gears_to_numbers[gear].append(number)

    total = 0
    for numbers in gears_to_numbers.values():
        if len(numbers) == 2:
            total += numbers[0] * numbers[1]

    return total

if __name__ == '__main__':
    main()
