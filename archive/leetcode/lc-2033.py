class Solution:
    def minOperations(self, grid: list[list[int]], x: int) -> int:
        # grid is m-rows (outer dim) and n-columns
        # single operation add/substract _some_ x from a _single_ element
        # I want to make all elements equal
        #
        # feels like this is some algebra theory... or are there programmatic approach?
        # oh x is fixed!
        #
        # Metric. Metric is symmetric. If, for any two number in the element, a b,
        # there's such that a - b != n * x for some integer n, then it is impossible.
        #
        # Constraint
        # a + n1 * x
        # = b + n2 * x
        # ...
        # = z + nN * x
        #
        # objective function
        # min_{n1, n2, ..., nN} n1 + n2 + ... + N
        #
        # Brute force way one: select random element as the "center point",
        # Then calculate delta to all elements.
        # If any non integer nX, then return -1
        # Repeat for all elements
        # O((m x n)^2)  (because m x n for each m x n elements)
        #
        # Difference is symmetric tho, a - b = b - a. But removing this duplicate
        # won't change the overall scaling...
        #
        # The array shape is a distraction. It might as well a vector and it is the
        # same problem
        #
        # One element
        # a -> 0? Base case
        # Two element case
        # a, b -> easy, just do a-b once and divide by x
        # Three element
        # a, b, c ->
        # 0   | b-a | c-a
        # a-b | 0   | c-b
        # a-c | b-c | 0
        #
        # choose whichever row has minimal sum!
        # wait I should add the absolute
        #
        # |a-b| -> sometimes a-b sometimes b-a
        # one heuristic is to choose from mean but... it is not precise
        #
        # But hey, does it matter though?
        #
        # Say 0 4 8, x = 2, select pivot
        # 0 -> 2+4 = 6
        # 4 -> 2+2 = 4
        # 8 -> 2+4 = 6
        # Easy
        #
        # 1 5 9
        #
        # Testing impossiblity is easy; if one row is impossible, all will be (since symmetric)
        #
        # Stop thinking in algebra... how to do this algorithmically?
        # Oh hey algebra *is* the correct solution!
        #
        pass
