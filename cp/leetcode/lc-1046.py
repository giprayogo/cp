import heapq


class Solution:
    def lastStoneWeight(self, stones: list[int]) -> int:
        max_heap = [-x for x in stones]
        heapq.heapify(max_heap)
        a = b = None
        while True:
            try:
                a = heapq.heappop(max_heap)
                b = heapq.heappop(max_heap)
                # if a > b:  # NOTE+ Remember: heap. this is never true!
                #     heapq.heappush(max_heap, b - a)
                if a < b:
                    heapq.heappush(max_heap, a - b)
                a = b = None
            # NOTE: also I can use length as while condition instead of using exception as control flow
            except IndexError:
                if a:
                    return -a
                else:
                    return 0


s = Solution()
assert s.lastStoneWeight([2, 7, 4, 1, 8, 1]) == 1
assert s.lastStoneWeight([1]) == 1
