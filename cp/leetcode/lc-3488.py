class Solution:
    def solveQueries(self, nums: list[int], queries: list[int]) -> list[int]:
        index = {}
        for i, num in enumerate(nums):
            indices = index.setdefault(num, [])
            indices.append(i)

        n = len(nums)
        for i, j in enumerate(queries):
            q = nums[j]
            indices = index[q]

            if len(indices) == 1:
                queries[i] = -1
            else:
                l, r = 0, len(indices)

                while l < r:
                    m = l + (r - l) // 2
                    k = indices[m]

                    if k == j:
                        break
                    elif k > j:
                        r = m
                    elif k < j:
                        l = m + 1
                else:
                    raise RuntimeError("wrong logic somewhere")

                a, b = indices[m - 1], indices[(m + 1) % len(indices)]
                queries[i] = min(circular_distance(a, k, n), circular_distance(b, k, n))
        return queries


def circular_distance(a, b, n):
    return min(abs(max(a, b) - min(a, b)), abs(max(a, b) - min(a, b) - n))


print(Solution().solveQueries([1, 3, 1, 4, 1, 3, 2], [0, 3, 5]))
