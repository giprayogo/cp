import heapq


class Solution:
    def kClosest(self, points: list[list[int]], k: int) -> list[list[int]]:
        distances = [(p[0] ** 2 + p[1] ** 2, p) for p in points]
        heapq.heapify(distances)
        return [x[1] for x in heapq.nsmallest(k, distances)]


s = Solution()
print(s.kClosest([[1, 3], [-2, 2]], 1))
print(s.kClosest([[3, 3], [5, -1], [-2, 4]], 2))
