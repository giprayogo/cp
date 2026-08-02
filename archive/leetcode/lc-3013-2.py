class Container:
    def __init__(self, k: int, dist: int):
        self.min_k = set()
        self.other = set()
        self.k = k
        self.dist = dist

    def add(self, value: int): ...

    def remove(self, value: int): ...

    def cost(self) -> int: ...


class Solution:
    def minimumCost(self, nums: list[int], k: int, dist: int) -> int:
        window = Container(k, dist)

        l = 0
        r = 1
        vl = nums[l]
        vr = nums[r]
        for _ in range(dist + 1):
            vr = nums[r]
            window.add(vr)

            r += 1

        l += 1
        min_cost = window.cost()

        while r < len(nums):
            vl = nums[l]
            vr = nums[r]

            min_cost = min(min_cost, window.cost())
            window.remove(vl)
            window.add(vr)

            l += 1
            r += 1

        return nums[0] + min_cost


# nums = [10, 1, 2, 2, 2, 1]
# nums = [10, 8, 18, 9]
nums = [1, 6, 3, 5]
# k = 4
# k = 3
k = 3
# dist = 3
# dist = 1
dist = 2
print(Solution().minimumCost(nums, k, dist))
