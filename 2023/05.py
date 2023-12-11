import bisect
import sys

from collections import deque
from collections.abc import Mapping
from dataclasses import dataclass
from typing import Deque, Dict, Iterable, List, Sequence, Tuple


def main() -> None:
    # result = part1()
    result = part2()

    print(result)


class RangeMap(object):
    def __init__(self) -> None:
        # (key_range_start, value_range_start, range_len) sorted by key_range_start
        self.key_to_value = []  # type: List[Tuple[int, int, int]]

    def get_value_for_key(self, key: int) -> int:
        # print(f'get value for {key}')

        # Find the entry index to the right of the key in question, then look at the entry just
        #  before that
        entry_idx = bisect.bisect_right(self.key_to_value, key, key=lambda e: e[0])
        if entry_idx > 0:
            key_range_start, value_range_start, range_len = self.key_to_value[entry_idx - 1]
            # print(f'found entry starting at {key_range_start}')
            if key_range_start <= key < key_range_start + range_len:
                return value_range_start + (key - key_range_start)
        
        return key
            
    def add_mapping(self, key_range_start: int, value_range_start: int, range_len: int) -> None:
        entry = (key_range_start, value_range_start, range_len)
        bisect.insort(self.key_to_value, entry, key=lambda e: e[0])

    def flatten_with_next_map(self, next_map: RangeMap) -> RangeMap:
        """
        This map's values are next_map's keys
        """
        new_list: List[Tuple[int, int, int]] = []

        for this_key_start, this_value_start, this_range_len in self.key_to_value:
            # Find the overlapping range(s) from next_map that apply to this range entry
            next_overlaps = [
                next_key_start, next_value_start, next_range_len
                for (next_key_start, next_value_start, next_range_len) in next_map.key_to_value
                # Check for range overlap
                if (
                    this_value_start + this_range_len - 1 >= next_key_start and
                    next_key_start + next_range_len - 1 >= this_value_start
                )
            ]

            idx = 0
            while idx < len(next_overlaps):
                next_overlap = next_overlaps[i]

                # Insert a range *not* covered by next_overlap, if necessary

@dataclass()
class SeedMap(object):
    # Old value, left so Part 1 still works
    seeds: Sequence[int]

    seed_ranges: RangeMap
    seed_to_soil: RangeMap
    soil_to_fertilizer: RangeMap
    fertilizer_to_water: RangeMap
    water_to_light: RangeMap
    light_to_temperature: RangeMap
    temperature_to_humidity: RangeMap
    humidity_to_location: RangeMap

    def get_all_values_for_seed(self, seed: int) -> Tuple[int, int, int, int, int, int, int]:
        soil = self.seed_to_soil.get_value_for_key(seed)
        fertilizer = self.soil_to_fertilizer.get_value_for_key(soil)
        water = self.fertilizer_to_water.get_value_for_key(fertilizer)
        light = self.water_to_light.get_value_for_key(water)
        temperature = self.light_to_temperature.get_value_for_key(light)
        humidity = self.temperature_to_humidity.get_value_for_key(temperature)
        location = self.humidity_to_location.get_value_for_key(humidity)
        
        return soil, fertilizer, water, light, temperature, humidity, location

def parse_seed_map() -> SeedMap:
    """
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2

    fertilizer-to-water map:
    49 53 8
    0 11 42
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    """

    # Put lines into deque, removing newlines and filtering out empty lines
    lines = deque(y for y in [x.strip() for x in sys.stdin] if y)
    
    # seeds
    line = lines.popleft()
    line = line.removeprefix("seeds: ")
    seeds = deque(int(s) for s in line.split(" "))
    seeds_copy = list(seeds)
    # Producing sort of a seed -> seed map, just for easier code later
    seed_range_map = RangeMap()
    while seeds:
        seed_range_start = seeds.popleft()
        seed_range_len = seeds.popleft()
        seed_range_map.add_mapping(seed_range_start, seed_range_start, seed_range_len)

    seed_to_soil = parse_range_map(lines)
    soil_to_fertilizer = parse_range_map(lines)
    fertilizer_to_water = parse_range_map(lines)
    water_to_light = parse_range_map(lines)
    light_to_temperature = parse_range_map(lines)
    temperature_to_humidity = parse_range_map(lines)
    humidity_to_location = parse_range_map(lines)

    return SeedMap(
        seeds=seeds_copy,
        seed_ranges=seed_range_map,
        seed_to_soil=seed_to_soil,
        soil_to_fertilizer=soil_to_fertilizer,
        fertilizer_to_water=fertilizer_to_water,
        water_to_light=water_to_light,
        light_to_temperature=light_to_temperature,
        temperature_to_humidity=temperature_to_humidity,
        humidity_to_location=humidity_to_location,
    )

def parse_range_map(lines: Deque[str]) -> RangeMap:
    line = lines.popleft()
    if not line.endswith("map:"):
        raise ValueError()

    range_map = RangeMap()
    while lines and not lines[0].endswith("map:"):
        line = lines.popleft()
        value_range_start, key_range_start, range_len = [int(s) for s in line.split(" ")]
        range_map.add_mapping(key_range_start, value_range_start, range_len)

    return range_map

def part1() -> int:
    seed_map = parse_seed_map()

    locations = []
    for seed in seed_map.seeds:
        (soil, fertilizer, water, light,
         temperature, humidity, location) = seed_map.get_all_values_for_seed(seed)

        locations.append(location)

        # print(f'{seed} {soil} {fertilizer} {water} {light} {temperature} {humidity} {location}')

    return min(locations)


def part2() -> int:
    seed_map = parse_seed_map()

    # Initialize this to the seed -> seed map, we'll successively flatten maps onto it until
    #  it's a seed -> location map
    seed_location_map = seed_map.seed_ranges

    return min_location

if __name__ == '__main__':
    main()
