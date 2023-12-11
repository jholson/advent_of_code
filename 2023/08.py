import math
import sys

from dataclasses import dataclass
from typing import Mapping, Tuple

def main() -> None:
    # result = part1()
    result = part2()

    print(result)


@dataclass()
class Map(object):
    directions: str
    children: Mapping[str, Tuple[str, str]]

def parse_input() -> Map:
    lines = [l.strip() for l in sys.stdin]

    directions = lines[0]

    children = {}
    for line in lines[2:]:
        if not line:
            continue

        parent, line = line.split(' = ', 1)
        left, right = line.strip('()').split(', ', 1)

        children[parent] = (left, right)

    return Map(directions=directions, children=children)

def part1() -> int:
    m = parse_input()

    here = 'MSA'
    dir_idx = 0
    steps = 0
    print(f'{steps}: {here}')
    # while here != 'ZZZ':
    while not here.endswith('Z'):
        left, right = m.children[here]
        direction = m.directions[dir_idx]
        if direction == 'L':
            here = left
        else:
            here = right

        dir_idx = (dir_idx + 1) % len(m.directions)
        steps += 1

        # if steps % 100000 == 0:
        print(f'{steps}: {direction} -> {here}')

    return steps

def part2() -> int:
    m = parse_input()

    """
    Hypothesis/assumption: From each start node, eventually a cycle is entered after some initial delay
    
    Identifying a cycle: A "seen" node is the node label plus the dir_idx that led you there
    Keep a map from (node, dir_idx) -> step_seen

    Also need to factor in that the Z node in a given cycle is some distance into the cycle

    So we need to find the least common multiples of the cycles, but account for both the initial offset and the Z offset?
    """

    starts = [n for n in m.children if n.endswith('A')]
    cycle_lengths = []

    for start in starts:
        # print(f'Starting from {start}')

        dir_idx = 0
        steps = 0

        # Map from (node, dir_idx) -> steps
        seen_at_step: Dict[Tuple[str, int], int] = {}
        z_step: int = None

        here = start
        seen_at_step[here] = 0

        while True:
            left, right = m.children[here]
            direction = m.directions[dir_idx]
            if direction == 'L':
                here = left
            else:
                here = right

            # print(here)

            dir_idx = (dir_idx + 1) % len(m.directions)
            steps += 1
    
            key = (here, dir_idx)
            if key in seen_at_step:
                # Encountered a same node/dir_idx that was seen before
                break

            seen_at_step[key] = steps

            if z_step is None and here.endswith('Z'):
                z_step = steps

        cycle_length = steps - seen_at_step[key]
        if cycle_length != z_step:
            raise ValueError("Simple LCM won't work here")

        # print(f' Initial offset to cycle: {seen_at_step[key]}')
        # print(f' Z node seen at: {z_step}')
        # print(f' Cycle started at node {here} on step {seen_at_step[key]}, encountered second time on step {steps}')
        # print(f' Cycle length: {cycle_length}')

        cycle_lengths.append(cycle_length)

    return math.lcm(*cycle_lengths)

if __name__ == '__main__':
    main()
