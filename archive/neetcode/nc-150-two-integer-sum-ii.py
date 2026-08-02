class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        # Numbers is sorted in non-decreasing order, means for any subsequent x, y in numbers,
        # x <= y
        #
        # Return indices (1-indexed) of two numbers (that is not the numbers themselves!),
        # such that sum = numbers[index1] + numbers[index2] = target and index1 < index2
        #
        # There will always be exactly one valid solution
        # And
        #
        # Max O(1) additional space
        #
        # Naive solution is checking every pair which is O(N^2) in time.
        # But given that it is non-decreasing, I can have this update rule:
        #
        # So I can use two pointers with this rule:
        # - Start index1 from most-left and index2 from most-right
        # - If sum > target, update index2 -> index2-1
        # - If sum < target, update index1 -> index1+1
        #
        # My proof (non-rigorous?)
        # - Suppose the correct indices are some a and b
        # - We need to prove that there will be no cases where the pointers ended up at a+1 or b-1,
        #   because then the update algorithm will not be able to find the correct solution
        # - Suppose that the left pointer ends up at a, and b is still at some c > b, then
        #   sum > target (not >= because there is only one valid solution), which means the
        #   algorithm will always move the right pointer, until it is at b
        # - It is not possible to have the sum < target after moving pointer b, because that would
        #   mean that c < b, which means the numbers are not sorted in non-decreasing order.
        # - Likewise for the left pointer
        i = 0
        j = len(numbers) - 1
        while (sum := numbers[i] + numbers[j]) != target:
            if sum > target:
                j -= 1
            elif sum < target:
                i += 1
        return [i + 1, j + 1]


numbers = [1, 2, 3, 4]
target = 3
assert Solution().twoSum(numbers, target) == [1, 2]
print("OK")
