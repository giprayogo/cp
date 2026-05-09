from collections import OrderedDict


class Solution:
    def minimumCost(self, nums: list[int], k: int, dist: int) -> int:
        min_k = dict()

        l = 0
        r = 1
        vl = nums[l]
        vr = nums[r]
        for _ in range(dist + 1):
            vr = nums[r]

            min_k[vr] = min_k.setdefault(vr, 0) + 1

            r += 1

        l += 1
        min_cost = self.min_n_from_dictionary(k - 1, min_k)

        while r < len(nums):
            vl = nums[l]
            vr = nums[r]

            min_k[vl] -= 1
            if min_k[vl] == 0:
                del min_k[vl]
            min_k[vr] = min_k.setdefault(vr, 0) + 1

            min_cost = min(min_cost, self.min_n_from_dictionary(k - 1, min_k))

            l += 1
            r += 1

        return nums[0] + min_cost

    def min_n_from_dictionary(self, n, dictionary) -> int:
        result = 0
        for k, v in sorted(dictionary.items()):
            take = min(n, v)
            result += k * take
            n -= take
            if n == 0:
                break
        return result


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
