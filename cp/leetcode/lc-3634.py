class Solution:
    def minRemoval_1(self, nums: list[int], k: int) -> int:
        # Greedy on sorted list.
        # Remove elements that locally maximize ratio increase.
        # O(n log n) + O(n) = O(n log n) time complexity, O(1) extra space
        # Not sure if correct!
        nums.sort()

        l = 0
        r = len(nums) - 1
        i = 0
        while l < r:
            if nums[r] / nums[l] < k or (nums[r] // nums[l] == k and nums[r] % nums[l] == 0):
                break
            if (nums[l + 1] / nums[l]) > (nums[r] / nums[r - 1]):
                l += 1
            else:
                r -= 1
            i += 1
        return i

    def minRemoval_2(self, nums: list[int], k: int) -> int:
        # DP on sorted list
        # O(n^2) time, O(1) space
        # Will it TLE? Yes!
        nums.sort()

        i = len(nums) - 1

        # For 1..n, k = 1 case: because no repeated elements -> Can have repeated element actually!
        # if k == 1:
        #     return i

        for l in range(0, len(nums)):
            for r in reversed(range(0, len(nums))):
                if nums[r] / nums[l] < k or (nums[r] // nums[l] == k and nums[r] % nums[l] == 0):
                    i = min(i, l + (len(nums) - 1 - r))
                    break
        return i

    # TODO+ Do I need joint sort-search thing?

    def minRemoval(self, nums: list[int], k: int) -> int:
        # DP on sorted list, with sliding window, because we want to find _minimum_
        # Better but won't help at all with 0...n k=1 case...
        # O(n^2) time, O(1) space
        # Will it TLE? Yes!
        nums.sort()

        i = len(nums) - 1
        for l in range(0, len(nums)):
            for r in reversed(range(0, len(nums))):
                if nums[r] / nums[l] < k or (nums[r] // nums[l] == k and nums[r] % nums[l] == 0):
                    i = min(i, l + (len(nums) - 1 - r))
                    break
        return i


# Debug Test cases
# 1. Floating point precision issue
# print(Solution().minRemoval([2, 1, 5], 2))
# 2. Incorrect example with greedy approach
print(Solution().minRemoval([466, 306, 76, 17, 60, 246, 341, 284], 2))
# 3.
