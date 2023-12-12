import math
import re
import sys

from dataclasses import dataclass
from typing import Sequence

def main() -> None:
    result = part1()

    print(result)


@dataclass(frozen=True)
class Race(object):
    time: int
    record_distance: int


def parse_input_part1() -> Sequence[Race]:
    time_line, distance_line = [l.strip() for l in sys.stdin]
    
    time_line = time_line.removeprefix("Time:").lstrip()
    distance_line = distance_line.removeprefix("Distance:").lstrip()

    times = [int(s) for s in re.split(r' +', time_line)]
    distances = [int(s) for s in re.split(r' +', distance_line)]

    return [Race(time=time, record_distance=distance) for time, distance in zip(times, distances)]

def parse_input_part2() -> Sequence[Race]:
    time_line, distance_line = [l.strip() for l in sys.stdin]
    
    time_line = time_line.removeprefix("Time:").lstrip()
    distance_line = distance_line.removeprefix("Distance:").lstrip()

    times = [int(''.join([d for d in time_line if d != ' ']))]
    distances = [int(''.join([d for d in distance_line if d != ' ']))]

    return [Race(time=time, record_distance=distance) for time, distance in zip(times, distances)]


def calc_distance(time: int, charge_time: int) -> int:
    return (time - charge_time) * charge_time

def part1() -> int:
    """
    d: distance traveled
    t: total time
    c: charge time

    d = (t - c) * c

    trying to solve for c, given d and t...
    -c^2 + tc - d = 0
    c = (t +- sqrt(t^2 - 4d)) / 2
    """
    races = parse_input_part2()

    result = 1
    for race in races:
        sqrt_res = math.sqrt(race.time ** 2 - 4 * race.record_distance)
        # print(f'sqrt_res = {sqrt_res}')
        low_time = math.ceil((race.time - sqrt_res) / 2.0)
        high_time = math.floor((race.time + sqrt_res) / 2.0)

        low_time = max(1, low_time)
        high_time = min(race.time - 1, high_time)

        # Account for if the roots exactly match the record distance
        if calc_distance(race.time, low_time) == race.record_distance:
            low_time += 1
        if calc_distance(race.time, high_time) == race.record_distance:
            high_time -= 1

        ways_to_win = max(high_time - low_time + 1, 0)

        # print(race)
        # print(f'Hold at least {low_time} and at most {high_time}, so {ways_to_win} ways to win')

        result *= ways_to_win

    return result

if __name__ == '__main__':
    main()
