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
            print(node.val)
            if node.left is not None:
                nodes.append(node.left)
            if node.right is not None:
                nodes.append(node.right)


class Solution:
    # def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
    #     if root is None:
    #         return root
    #     nodes = deque()
    #     nodes.append(root)
    #     while nodes:
    #         node = nodes.popleft()
    #         node.left, node.right = node.right, node.left
    #         if node.left is not None:
    #             nodes.append(node.left)
    #         if node.right is not None:
    #             nodes.append(node.right)
    #     return root

    # Recursive (again hint from neetcode)
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return root
        root.left, root.right = root.right, root.left
        if root.left is not None:
            self.invertTree(root.left)
        if root.right is not None:
            self.invertTree(root.right)
        return root


a = TreeNode(4)
b = TreeNode(2)
c = TreeNode(7)
a.left = b
a.right = c
d = TreeNode(1)
e = TreeNode(3)
b.left = d
b.right = e
f = TreeNode(6)
g = TreeNode(9)
c.left = f
c.right = g

print(list(a))
s = Solution()
s.invertTree(a)
print(list(a))
