#!/usr/bin/env python
import sys
from functools import reduce

max = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

sum = 0  # part 1
power = 0  # part 2
for line in sys.stdin.readlines():
    try:
        left, right = [x.strip() for x in line.strip().split(":")]
    except ValueError:
        print("WRONG", line)
        continue
    game_id = left.split()[1]
    shows = [
        [cubeset.strip().split() for cubeset in show.strip().split(",")]
        for show in right.split(";")
    ]
    possible = True
    least = {"red": 0, "green": 0, "blue": 0}
    for show in shows:
        show = dict([(y, int(x)) for x, y in show])
        for k, v in show.items():
            maxval = max[k]
            if v > maxval:
                possible = False
            # part 2
            least[k] = least[k] if least[k] >= v else v
    local_power = reduce(lambda x,y: x*y, least.values())
    power += local_power

    if possible:
        sum += int(game_id)

print(sum)
print(power)
