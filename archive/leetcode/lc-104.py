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
            # print(node.val)
            if node.left is not None:
                nodes.append(node.left)
            if node.right is not None:
                nodes.append(node.right)


class Solution:
    # Technically an Iterative DFS (right->right->right)
    # def maxDepth(self, root: Optional[TreeNode]) -> int:
    #     depth = 0
    #     if root is None:
    #         return depth
    #     nodes = []
    #     nodes.append((depth + 1, root))
    #     maxdepth = 0

    #     while nodes:
    #         depth, node = nodes.pop()
    #         maxdepth = max(depth, maxdepth)
    #         if node.left is not None:
    #             nodes.append((depth + 1, node.left))
    #         if node.right is not None:
    #             nodes.append((depth + 1, node.right))
    #     return maxdepth

    # Neetcode: recusive DFS also possible!
    # def maxDepth(self, root: Optional[TreeNode]) -> int:
    #     if root is None:
    #         return 0
    #     return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right))

    # NeetCode: if BFS should not need "depth" in the stack!
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        depth = 0
        nodes = deque()
        nodes.append(root)
        while nodes:
            # clever: consume the whole "level" while adding
            for _ in range(len(nodes)):
                node = nodes.popleft()
                if node.left:
                    nodes.append(node.left)
                if node.right:
                    nodes.append(node.right)
            depth += 1
        return depth


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
f.right = g
print(list(f))

s = Solution()
print(s.maxDepth(a))
print(s.maxDepth(f))
