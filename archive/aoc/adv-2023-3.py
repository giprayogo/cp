#!/usr/bin/env python
import sys
from functools import reduce

sum = 0
lines = sys.stdin.readlines()

nlines = len(lines)
masks = [0] * nlines
numbers = []
like_gears = []
for i, line in enumerate(lines):
    line = line.strip()
    if not line:
        continue

    in_number = False
    digits = []
    j_from = None
    j_to = None
    for j, c in enumerate(line):
        if c == ".":
            in_number = False

            # parse digits
            if digits:
                j_to = j - 1
                number = reduce(lambda x, y: 10 * x + y, digits)
                numbers.append((number, i, (j_from, j_to)))
                j_from = j_to = None
                digits = []
        elif c.isnumeric():
            in_number = True
            j_from = j if j_from is None else j_from
            digits.append(int(c))
        else:
            in_number = False
            # add to mask and dilate
            for _i in list(range(-1, 2)):
                _i += i
                for _j in list(range(-1, 2)):
                    _j += j
                    if _i >= 0 and _j >= 0 and _i < nlines and _j < nlines:
                        masks[_i] = (1 << _j) | masks[_i]
            if c == "*":
                like_gears.append((i, j, []))

            # parse digits
            if digits:
                j_to = j - 1
                number = reduce(lambda x, y: 10 * x + y, digits)
                numbers.append((number, i, (j_from, j_to)))
                j_from = j_to = None
                digits = []
    # parse digits
    if digits:
        j_to = j  # type: ignore
        number = reduce(lambda x, y: 10 * x + y, digits)
        numbers.append((number, i, (j_from, j_to)))
        j_from = j_to = None
        digits = []

# for row in masks:
#     print(f"{row:10b}")
# print(numbers)
print(list(enumerate(like_gears)))

# check mask
for num, row, (column_from, column_to) in numbers:
    mask = masks[row]
    # print(num)
    for col in range(column_from, column_to + 1):
        # print(col)
        bit = (mask >> col) & 1
        if bit:
            # print(">>", num)
            sum += num
            for i, (grow, gcolumn, adjs) in enumerate(like_gears):
                if abs(grow - row) <= 1 and gcolumn in range(
                    column_from - 1, column_to + 2
                ):
                    adjs.append(num)
                    like_gears[i] = (grow, gcolumn, adjs)
            break

ratsums = 0
for _, _, adjs in like_gears:
    if len(adjs) == 2:
        ratsums += adjs[0] * adjs[1]

print(sum)
print(ratsums)
