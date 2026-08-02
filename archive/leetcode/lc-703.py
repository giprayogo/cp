import heapq


class KthLargest:
    def __init__(self, k: int, nums: list[int]):
        self.k = k
        heapq.heapify(nums)
        self.heap = heapq.nlargest(self.k, nums)
        heapq.heapify(self.heap)

    def add(self, val: int) -> int:
        if len(self.heap) >= self.k:
            heapq.heappushpop(self.heap, val)
        else:
            heapq.heappush(self.heap, val)
        # I miss a trick: it's on the top of the heap! because I trim it already. thanks neetcode
        return self.heap[0]


obj = KthLargest(3, [4, 5, 8, 2])
print(obj.add(3))
print(obj.add(5))
print(obj.add(10))
print(obj.add(9))
print(obj.add(4))

obj = KthLargest(1, [])
print(obj.add(-3))
print(obj.add(-2))
print(obj.add(-4))
print(obj.add(0))
print(obj.add(4))

obj = KthLargest(3, [5, -1])
print(obj.add(2))
print(obj.add(1))
print(obj.add(-1))
print(obj.add(3))
print(obj.add(4))
