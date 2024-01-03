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
            if all(x is None for x in nodes):
                return
            node = nodes.popleft()
            if node:
                yield (node.val)
                nodes.append(node.left)
                nodes.append(node.right)
            else:
                yield "null"


class Solution:
    # OK but slooow
    # def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
    #     if root and subRoot:
    #         if self.isSameTree(root, subRoot):
    #             return True
    #         else:
    #             return self.isSubtree(root.left, subRoot) or self.isSubtree(
    #                 root.right, subRoot
    #             )
    #     elif (not root) and (not subRoot):
    #         return True
    #     else:
    #         return False

    # def isSameTree(self, left, right):
    #     if left and right and left.val == right.val:
    #         return self.isSameTree(left.left, right.left) and self.isSameTree(
    #             left.right, right.right
    #         )
    #     elif (not left) and (not right):
    #         return True
    #     else:
    #         return False

    # Optimize a bit
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        # Neetcode's neet observation: None subroot is subtree of anything, but if not and root is None,
        # then it is not subtreeing anything!
        if not subRoot:
            return True
        if not root:
            return False

        if self.isSameTree(root, subRoot):
            return True
        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)

    def isSameTree(self, left, right):
        if left and right and left.val == right.val:
            return self.isSameTree(left.left, right.left) and self.isSameTree(
                left.right, right.right
            )
        elif (not left) and (not right):
            return True
        else:
            return False


a = TreeNode(3)
b = TreeNode(4)
c = TreeNode(5)
a.left = b
a.right = c
d = TreeNode(1)
e = TreeNode(2)
b.left = d
b.right = e
print(list(a))

f = TreeNode(4)
g = TreeNode(1)
h = TreeNode(2)
f.left = g
f.right = h
print(list(f))

i = TreeNode(3)
j = TreeNode(4)
k = TreeNode(5)
i.left = j
i.right = k
l = TreeNode(1)  # noqa: E741
m = TreeNode(2)
n = TreeNode(0)
j.left = l
j.right = m
m.left = n
print(list(i))

s = Solution()
print(s.isSubtree(a, f))
print(s.isSubtree(i, f))
