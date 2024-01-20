from typing import Optional
from collections import deque


# LC Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __iter__(self):
        nodes = deque()
        nodes.append(self)
        while nodes:
            node = nodes.popleft()
            yield (node.val)
            if node.left is not None:
                nodes.append(node.left)
            if node.right is not None:
                nodes.append(node.right)


class Solution:
    # Recursive DFS
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        balanced = True

        def dfs(node) -> int:
            nonlocal balanced
            if not node:
                return 0
            left = dfs(node.left)
            right = dfs(node.right)
            if abs(left - right) > 1:
                balanced = False
            return 1 + max(left, right)

        dfs(root)
        return balanced

    # BFS: WA: I read the problem wrong; depth diff by 2 locally, not globally
    # def isBalanced(self, root: Optional[TreeNode]) -> bool:
    #     nodes = deque()
    #     if root:
    #         nodes.append(root)
    #     depth = 0
    #     balanced = True
    #     off = False
    #     while nodes:
    #         if off:
    #             balanced = False
    #             break
    #         if 2**depth != len(nodes):
    #             off = True
    #         for _ in range(len(nodes)):
    #             node = nodes.popleft()
    #             if node.left:
    #                 nodes.append(node.left)
    #             if node.right:
    #                 nodes.append(node.right)
    #         depth += 1

    #     return balanced


a = TreeNode(3)
b = TreeNode(9)
c = TreeNode(20)
a.left = b
a.right = c
d = TreeNode(15)
e = TreeNode(7)
c.left = d
c.right = e
print(list(a))

f = TreeNode(1)
g = TreeNode(2)
h = TreeNode(2)
f.left = g
f.right = h
i = TreeNode(3)
j = TreeNode(3)
g.left = i
g.right = j
k = TreeNode(4)
l = TreeNode(4)  # noqa: E741
i.left = k
i.right = l
print(list(f))

s = Solution()
print(s.isBalanced(a))
print(s.isBalanced(f))
print(s.isBalanced(None))
