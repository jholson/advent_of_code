import sys

def main():
    # result = part1()
    result = part2()

    print(result)

def part1():
    total = 0

    for line in sys.stdin:
        digits = [c for c in line if c.isdigit()]
        calibration = int(digits[0] + digits[-1])
        total += calibration

    return total


DIGIT_STRS = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

for x in range(1, 10):
    DIGIT_STRS[str(x)] = x


def part2():
    total = 0

    for line in sys.stdin:
        first_idx = len(line)
        last_idx = -1

        first_digit: int = None
        last_digit: int = None

        for digit_str, digit in DIGIT_STRS.items():
            # Search from beginning
            idx = line.find(digit_str)
            if idx != -1 and idx < first_idx:
                first_idx = idx
                first_digit = digit

            # Search from end
            idx = line.rfind(digit_str)
            if idx != -1 and idx > last_idx:
                last_idx = idx
                last_digit = digit

        calibration = int(str(first_digit) + str(last_digit))
        total += calibration

    return total

if __name__ == '__main__':
    main()
