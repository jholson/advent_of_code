import sys

from dataclasses import dataclass
from functools import reduce
from typing import Mapping, Sequence

def main():
    # result = part1()
    result = part2()

    print(result)


@dataclass()
class Game(object):
    game_id: int
    turns: Sequence[Mapping[str, int]]


def parse_game_from_line(line: str) -> Game:
    """
    Format:

    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    """
    line = line.strip()
    line = line.removeprefix("Game ")
    game_id_str, line = line.split(": ", 1)
    game_id = int(game_id_str)

    turns = []
    for turn_str in line.split("; "):
        colors: Dict[str, int] = {}
        for color_str in turn_str.split(", "):
            number_str, color = color_str.split(" ")
            number = int(number_str)

            colors[color] = number
        
        turns.append(colors)

    return Game(game_id=game_id, turns=turns)

def game_is_possible(game: Game) -> bool:
    return all(
        turn.get("red", 0) <= 12 and turn.get("green", 0) <= 13 and turn.get("blue", 0) <= 14
        for turn in game.turns
    )

def part1() -> int:
    games = [parse_game_from_line(line) for line in sys.stdin if line.strip() != ""]

    total = 0
    for game in games:
        if game_is_possible(game):
            total += game.game_id
    
    return total


def game_power(game: Game) -> int:
    max_cubes_seen = {
        "red": 0,
        "green": 0,
        "blue": 0,
    }

    for turn in game.turns:
        for color, number in turn.items():
            max_cubes_seen[color] = max(max_cubes_seen[color], number)

    return reduce(lambda a, b: a*b, max_cubes_seen.values())


def part2() -> int:
    games = [parse_game_from_line(line) for line in sys.stdin if line.strip() != ""]

    total = 0
    for game in games:
        power = game_power(game)
        total += power

    return total

if __name__ == '__main__':
    main()
