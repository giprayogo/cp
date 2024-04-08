from typing import List


# Actually not bad tho not the best... do it faster! Debug faster!
class Solution:
    def visibleMountains(self, peaks: List[List[int]]) -> int:
        peaks = [(y, x) for x, y in peaks]
        peaks.sort()
        vp = [peaks[-1]]
        dups = set()

        for y, x in peaks[:-1][::-1]:
            for my, mx in vp:
                dy = my - y
                dx = abs(mx - x)
                if y != my and dy >= dx:
                    break
                elif y == my:
                    if x == mx:
                        dups.add((y, x))
                        break
                    else:
                        vp.append((y, x))
                        break
            else:
                vp.append((y, x))
        return len(vp) - len(dups)


s = Solution()
try:
    r = s.visibleMountains([[2, 2], [2, 2], [3, 1]])
    assert r == 0
    r = s.visibleMountains([[2, 2], [6, 3], [5, 4]])
    assert r == 2
    r = s.visibleMountains([[1, 3], [1, 3]])
    assert r == 0
    r = s.visibleMountains(
        [
            [12, 10],
            [37, 25],
            [8, 12],
            [8, 36],
            [39, 4],
            [22, 3],
            [36, 19],
            [3, 17],
            [10, 19],
            [19, 38],
            [6, 36],
            [27, 23],
            [4, 29],
            [36, 27],
            [21, 28],
            [9, 11],
            [24, 1],
            [16, 17],
            [16, 9],
            [22, 23],
            [37, 31],
            [34, 17],
            [19, 2],
            [33, 3],
            [12, 14],
            [23, 7],
            [20, 36],
            [7, 36],
            [16, 7],
            [24, 38],
        ]
    )
    assert r == 6
except AssertionError:
    print(r)
