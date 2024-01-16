from typing import Optional, Self
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
                yield node
                nodes.append(node.left)
                nodes.append(node.right)
            else:
                yield None

    def __getitem__(self, key: int) -> Optional[Self]:
        self_as_list = list(self)
        return self_as_list[key]

    @classmethod
    def from_list(cls, node_list: list[int | None]) -> Optional[Self]:
        # No don't use this kind of structure, instead count actively?
        # for e in source_list:
        #    node = cls(e)
        # Because I don't want to build every example!
        # pointer order
        # root
        # left
        # right
        nodes = [cls(x) if x is not None else None for x in node_list]
        length = len(nodes)
        for i, e in enumerate(nodes):
            if e is None:
                continue

            left_index, right_index = 2 * i + 1, 2 * i + 2
            if left_index < length and nodes[left_index] is not None:
                e.left = nodes[left_index]
            if right_index < length and nodes[right_index] is not None:
                e.right = nodes[right_index]
        return nodes[0]

    def __str__(self):
        return str([x.val if x is not None else "null" for x in list(self)])


class Solution:
    # Technically correct but I missed a trick: ordering of node values
    # in a -binary search tree-. This solution is for more general tree
    # regardless of node value
    # Anyway
    # def lowestCommonAncestor(
    #     self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    # ) -> "TreeNode":
    #     if root.left and self.isSubTree(root.left, p) and self.isSubTree(root.left, q):
    #         return self.lowestCommonAncestor(root.left, p, q)
    #     elif (
    #         root.right
    #         and self.isSubTree(root.right, p)
    #         and self.isSubTree(root.right, q)
    #     ):
    #         return self.lowestCommonAncestor(root.right, p, q)
    #     else:
    #         return root

    # def isSubTree(self, root, node):
    #     if not root:
    #         return False
    #     if not node:
    #         return True
    #     if self.isSameTree(root, node):
    #         return True
    #     else:
    #         return self.isSubTree(root.left, node) or self.isSubTree(root.right, node)

    # def isSameTree(self, left, right):
    #     if left and right and left.val == right.val:
    #         return self.isSameTree(left.left, right.left) and self.isSameTree(
    #             left.right, right.right
    #         )
    #     elif (not left) and (not right):
    #         return True
    #     else:
    #         return False

    # Now here's a BST-specific solution
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        while True:
            if root.val > p.val and root.val > q.val:
                root = root.left
            elif root.val < p.val and root.val < q.val:
                root = root.right
            else:
                break
        return root


s = Solution()

a = TreeNode.from_list([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])
print(a)
if a is not None:
    p = a[1]
    q = a[2]
    if p is not None and q is not None:
        print(s.lowestCommonAncestor(a, p, q).val)

print("=" * 88)
b = TreeNode.from_list([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])
print(b)
if b is not None:
    p = b[1]
    q = b[4]
    if p is not None and q is not None:
        print(s.lowestCommonAncestor(b, p, q).val)

print("=" * 88)
c = TreeNode.from_list([2, 1])
print(c)
if c is not None:
    p = c[0]
    q = c[1]
    if p is not None and q is not None:
        print(s.lowestCommonAncestor(c, p, q).val)
