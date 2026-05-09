class Solution:
    def minOperations(self, s: str) -> int:
        num = int(s, 2)

        two_shifts = (len(s) - 1) // 2
        one_shift = (len(s) - 1) % 2

        compare = 1
        for _ in range(two_shifts):
            compare = (compare << 2) + 1
        if one_shift:
            compare <<= 1

        ops = (num ^ compare).bit_count()
        return min(ops, len(s) - ops)


print(Solution().minOperations("0100"))
print(Solution().minOperations("10"))
print(Solution().minOperations("1111"))
