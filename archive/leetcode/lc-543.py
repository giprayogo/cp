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
    # Work but slooooooow. Need to learn the theory
    # def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
    #     maxLength = 0
    #     nodes = []
    #     if root:
    #         nodes.append(root)
    #     while nodes:
    #         node = nodes.pop()
    #         maxLength = max(
    #             self.maxDepth(node.left) + self.maxDepth(node.right), maxLength
    #         )
    #         if node.left:
    #             nodes.append(node.left)
    #         if node.right:
    #             nodes.append(node.right)
    #     return maxLength

    # def maxDepth(self, root: Optional[TreeNode]) -> int:
    #     if root is None:
    #         return 0
    #     return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))

    # NeetCode is basically the same but just with specializing the DFS? Try?
    # So function call is -that- expensive; I wonder if there are other solutions tho
    # Naah all bsically the same. problem closed.
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        maxLength = 0

        def dfs(node) -> int:
            nonlocal maxLength
            if not node:
                return 0
            else:
                left = dfs(node.left)
                right = dfs(node.right)
                maxLength = max(maxLength, left + right)
                return 1 + max(left, right)

        dfs(root)
        return maxLength


a = TreeNode(1)
b = TreeNode(2)
c = TreeNode(3)
a.left = b
a.right = c
d = TreeNode(4)
e = TreeNode(5)
b.left = d
b.right = e
print(list(a))

f = TreeNode(1)
g = TreeNode(2)
f.left = g
print(list(f))

s = Solution()
print(s.diameterOfBinaryTree(a))
print(s.diameterOfBinaryTree(f))
