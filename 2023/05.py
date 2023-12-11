import bisect
import functools
import sys

from collections import deque
from collections.abc import Mapping
from dataclasses import dataclass
from typing import Deque, Dict, Iterable, List, Optional, Sequence, Tuple


def main() -> None:
    # result = part1()
    result = part2()

    print(result)


@dataclass(frozen=True, slots=True)
class RangeMapEntry(object):
    key_start: int
    val_start: int
    range_len: int

    def key_within_range(self, key: int) -> bool:
        return self.key_start <= key < self.key_end_exclusive()

    def key_end_exclusive(self) -> int:
        return self.key_start + self.range_len

    def val_end_exclusive(self) -> int:
        return self.val_start + self.range_len

class RangeMap(object):
    def __init__(self, key_to_val: Optional[List[RangeMapEntry]] = None) -> None:
        # RangeMapEntry's sorted by key_start field (ascending)
        self.key_to_val: List[RangeMapEntry] = key_to_val or []

    def get_value_for_key(self, key: int) -> int:
        # Find the entry index to the right of the key in question, then look at the entry just
        #  before that
        entry_idx = bisect.bisect_right(self.key_to_val, key, key=lambda e: e.key_start)
        if entry_idx > 0:
            entry = self.key_to_val[entry_idx - 1]
            if entry.key_within_range(key):
                return entry.val_start + (key - entry.key_start)
        
        return key
            
    def add_mapping(self, key_start: int, val_start: int, range_len: int) -> None:
        entry = RangeMapEntry(key_start=key_start, val_start=val_start, range_len=range_len)
        bisect.insort(self.key_to_val, entry, key=lambda e: e.key_start)

    def flatten_with_next_map(self, next_map: 'RangeMap') -> 'RangeMap':
        """
        This map's values are next_map's keys
        """
        new_list: List[RangeMapEntry] = []

        # Running index into next_map, which is already sorted by key
        next_idx = 0
        next_e = next_map.key_to_val[next_idx]

        # Iterate over this map's entries sorted by value (so this map and next
        #  map will be ordered by the same unit basically
        for this_e in sorted(self.key_to_val, key=lambda e: e.val_start):
            # this_val_so_far indicates that the range [this_e.val_start, this_val_so_far) is
            #  accounted for
            this_val_so_far = this_e.val_start
            while this_val_so_far < this_e.val_end_exclusive():
                # Iterate through next_e until we're past any ranges that are completely before
                #  this_val_so_far
                while next_e and next_e.key_end_exclusive() <= this_val_so_far:
                    next_idx += 1
                    if next_idx < len(next_map.key_to_val):
                        next_e = next_map.key_to_val[next_idx]
                    else:
                        next_e = None

                if next_e and next_e.key_start <= this_val_so_far:
                    # next_e overlaps with this_val_so_far

                    # ...but by how much?
                    this_val_end_exclusive = min(
                        next_e.key_end_exclusive(), this_e.val_end_exclusive()
                    )
                    new_range_len = this_val_end_exclusive - this_val_so_far
                    new_val_start = next_e.val_start + (this_val_so_far - next_e.key_start)
                    new_key_start = this_e.key_start + (this_val_so_far - this_e.val_start)

                    new_list.append(RangeMapEntry(
                        # TODO this is wrong, needs to be key and not value
                        key_start=new_key_start,

                        val_start=new_val_start,
                        range_len=new_range_len,
                    ))

                    this_val_so_far = this_val_end_exclusive
                else:
                    # next_e starts after this_val_so_far, so we need a filler range (just the
                    #  identity map)

                    if next_e:
                        new_val_end_exclusive = min(next_e.key_start, this_e.val_end_exclusive())
                    else:
                        new_val_end_exclusive = this_e.val_end_exclusive()
                    new_range_len = new_val_end_exclusive - this_val_so_far
                    new_key_start = this_e.key_start + (this_val_so_far - this_e.val_start)

                    new_list.append(RangeMapEntry(
                        # TODO this is wrong, needs to be key and not value
                        key_start=new_key_start,

                        val_start=this_val_so_far,
                        range_len=new_range_len,
                    ))

                    this_val_so_far = new_val_end_exclusive

        return RangeMap(new_list)

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
        val_start, key_start, range_len = [int(s) for s in line.split(" ")]
        range_map.add_mapping(key_start, val_start, range_len)

    return range_map

def part1() -> int:
    seed_map = parse_seed_map()

    locations = []
    for seed in seed_map.seeds:
        (soil, fertilizer, water, light,
         temperature, humidity, location) = seed_map.get_all_values_for_seed(seed)

        locations.append(location)

    return min(locations)


def part2() -> int:
    seed_map = parse_seed_map()

    # Initialize this to the seed -> seed map, we'll successively flatten maps onto it until
    #  it's a seed -> location map
    seed_to_location_map = functools.reduce(
        lambda r1, r2: r1.flatten_with_next_map(r2),
        [
            seed_map.seed_ranges,
            seed_map.seed_to_soil,
            seed_map.soil_to_fertilizer,
            seed_map.fertilizer_to_water,
            seed_map.water_to_light,
            seed_map.light_to_temperature,
            seed_map.temperature_to_humidity,
            seed_map.humidity_to_location,
        ])

    # Grab minimum val_start value from seed_to_location_map
    return min(e.val_start for e in seed_to_location_map.key_to_val)


if __name__ == '__main__':
    main()
