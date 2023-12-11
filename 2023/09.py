import sys

from dataclasses import dataclass
from typing import Sequence

def main() -> None:
    # result = part1()
    result = part2()

    print(result)


@dataclass()
class Thing(object):
    pass

def parse_input() -> Sequence[Sequence[int]]:
    return [[int(s) for s in l.strip().split()] for l in sys.stdin if l.strip()]

def part1() -> int:
    seqs = parse_input()

    total = 0
    for seq in seqs:
        tri: Sequence[Sequence[int]] = [seq]
        prev_seq: Sequence[int] = seq

        while not all(x == 0 for x in prev_seq):
            new_seq = [prev_seq[i + 1] - prev_seq[i] for i in range(len(prev_seq) - 1)]

            tri.append(new_seq)
            prev_seq = new_seq

        # Iterate over tri backwards ("bottom" up, from the second to bottom layer)
        tri[-1].append(0)
        for i in range(len(tri) - 2, -1, -1):
            tri[i].append(tri[i][-1] + tri[i + 1][-1])

        total += tri[0][-1]

    return total
    

def part2() -> int:
    seqs = parse_input()

    total = 0
    for seq in seqs:
        tri: Sequence[Sequence[int]] = [seq]
        prev_seq: Sequence[int] = seq

        while not all(x == 0 for x in prev_seq):
            new_seq = [prev_seq[i + 1] - prev_seq[i] for i in range(len(prev_seq) - 1)]

            tri.append(new_seq)
            prev_seq = new_seq

        # Separate list to hold the extrapolated previous numbers for each level
        prev_num: Sequence[Optional[int]] = [None] * len(tri)
        prev_num[-1] = 0
        for i in range(len(tri) - 2, -1, -1):
            prev_num[i] = tri[i][0] - prev_num[i + 1]

        total += prev_num[0]


    return total

if __name__ == '__main__':
    main()
