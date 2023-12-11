import sys

from dataclasses import dataclass
from typing import Tuple, Sequence, Set

def main() -> None:
    # result = part1()
    result = part2()

    print(result)


@dataclass()
class Thing(object):
    pass


prev_dir_to_next_dir = {
    "u": {
        '|': "u",
        '7': "l",
        'F': "r",
    },
    "d": {
        '|': "d",
        'L': "r",
        'J': "l",
    },
    "l": {
        '-': "l",
        'F': "d",
        'L': "u",
    },
    "r": {
        '-': "r",
        '7': "d",
        'J': "u",
    },
}

def parse_input() -> Sequence[str]:
    return [k for k in [l.strip() for l in sys.stdin] if k]

def part1() -> int:
    m = parse_input()

    path = find_loop(m)

    path_len = len(path)

    # Round down. If you round up, that puts you past the midpoint and you're closer now 
    return path_len // 2

def find_loop(m: Sequence[str]) -> Set[Tuple[int, int]]:
    s_pos = find_s_pos(m)

    # First direction
    pos_i, pos_j = s_pos

    # Up
    if pos_i > 0 and m[pos_i - 1][pos_j] in ('|', '7', 'F'):
        first_step_pos = (pos_i - 1, pos_j)
        direction = "u"
    # Down
    elif pos_i < len(m) - 1 and m[pos_i + 1][pos_j] in ('|', 'L', 'J'):
        first_step_pos = (pos_i + 1, pos_j)
        direction = "d"
    # Left
    elif pos_j > 0 and m[pos_i][pos_j - 1] in ('-', 'F', 'L'):
        first_step_pos = (pos_i, pos_j - 1)
        direction = "l"
    # Right
    elif pos_j < len(m[0]) - 1 and m[pos_i][pos_j + 1] in ('-', '7', 'J'):
        first_step_pos = (pos_i, pos_j + 1)
        direction = "r"
    else:
        raise ValueError()
    
    # Follow the rest of the pipe
    pos = first_step_pos
    path = {first_step_pos}
    while pos != s_pos:
        pos_i, pos_j = pos
        next_direction = prev_dir_to_next_dir[direction][m[pos_i][pos_j]]
        if next_direction == "u":
            next_pos = (pos_i - 1, pos_j)
        elif next_direction == "d":
            next_pos = (pos_i + 1, pos_j)
        elif next_direction == "l":
            next_pos = (pos_i, pos_j - 1)
        elif next_direction == "r":
            next_pos = (pos_i, pos_j + 1)
        else:
            raise ValueError()

        path.add(next_pos)

        pos = next_pos
        direction = next_direction

    return path

def find_s_pos(m: Sequence[str]) -> Tuple[int, int]:
    # Find S
    for i in range(len(m)):
        for j in range(len(m[0])):
            if m[i][j] == 'S':
                return i, j
    raise ValueError()


def part2() -> int:
    """
    For each point not on the loop, draw a "ray" (either to the right or left), and only consider
    the characters in the loop along that ray. Count the number of times the ray crosses the loop.
    Going rightwards, an L must (eventually) be followed by either a J or a 7. A L-7 sequence counts
    as crossing the loop once, a L-J sequence counts as crossing the loop twice (or zero times).
    Each | would be a single crossing
    """
    m = parse_input()

    loop = find_loop(m)

    # Iterate over all squares not part of the loop
    num_inside_points = 0
    for i in range(len(m)):
        for j in range(len(m[0])):
            if (i, j) not in loop and is_point_on_inside(m, loop, i, j):
                num_inside_points += 1
                
    return num_inside_points


def is_point_on_inside(m: Sequence[str], loop: Set[Tuple[int, int]], i: int, j: int) -> bool:
    # Draw a "ray" to the right
    d = "r"
    ray = [m[i][c] for c in range(j + 1, len(m[0])) if (i, c) in loop]

    if 'S' not in ray:
        num_crossings = num_right_ray_crossings(ray)
    else:
        # S is hard to handle, so we'll look another direction instead (left)
        ray = [m[i][c] for c in range(j - 1, -1, -1) if (i, c) in loop]
        num_crossings = num_left_ray_crossings(ray)

    return num_crossings % 2 == 1

def num_right_ray_crossings(ray: str) -> int:
    crossings = 0
    i = 0
    ray = [e for e in ray if e not in ('.', '-')]
    while i < len(ray):
        if ray[i] == '|':
            crossings += 1
        elif ray[i] == 'L':
            i += 1
            if ray[i] == '7':
                crossings += 1
            elif ray[i] != 'J':
                raise ValueError(ray[i])
        elif ray[i] == 'F':
            i += 1
            if ray[i] == 'J':
                crossings += 1
            elif ray[i] != '7':
                raise ValueError(ray[i])
        else:
            raise ValueError(ray[i])

        i += 1

    return crossings

def num_left_ray_crossings(ray: str) -> int:
    crossings = 0
    i = 0
    ray = [e for e in ray if e not in ('.', '-')]
    while i < len(ray):
        if ray[i] == '|':
            crossings += 1
        elif ray[i] == 'J':
            i += 1
            if ray[i] == 'F':
                crossings += 1
            elif ray[i] != 'L':
                raise ValueError(ray[i])
        elif ray[i] == '7':
            i += 1
            if ray[i] == 'L':
                crossings += 1
            elif ray[i] != 'F':
                raise ValueError(ray[i])
        else:
            raise ValueError(ray[i])


        i += 1

    return crossings

if __name__ == '__main__':
    main()
