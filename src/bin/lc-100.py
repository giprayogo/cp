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
            if node:
                yield (node.val)
                nodes.append(node.left)
                nodes.append(node.right)
            else:
                yield "null"


class Solution:
    # BFS; kind of sloow tho
    # def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
    #     nodes_a = deque()
    #     nodes_b = deque()
    #     if p:
    #         if q:
    #             nodes_a.append(p)
    #             nodes_b.append(q)
    #         else:
    #             return False
    #     elif q:
    #         return False

    #     while nodes_a:
    #         node_a = nodes_a.popleft()
    #         node_b = nodes_b.popleft()

    #         if node_a.val != node_b.val:
    #             return False

    #         if node_a.left:
    #             if node_b.left:
    #                 nodes_a.append(node_a.left)
    #                 nodes_b.append(node_b.left)
    #             else:
    #                 return False
    #         elif node_b.left:
    #             return False
    #         if node_a.right:
    #             if node_b.right:
    #                 nodes_a.append(node_a.right)
    #                 nodes_b.append(node_b.right)
    #             else:
    #                 return False
    #         elif node_b.right:
    #             return False

    #     return True

    # DFS recursive; should be much simpler!
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p:
            if q:
                if p.val != q.val:
                    return False
                return self.isSameTree(p.left, q.left) and self.isSameTree(
                    p.right, q.right
                )
            else:
                return False
        elif q:
            return False
        else:
            return True


s = Solution()

a = TreeNode(1)
b = TreeNode(2)
c = TreeNode(3)
a.left = b
a.right = c
d = TreeNode(1)
e = TreeNode(2)
f = TreeNode(3)
d.left = b
d.right = c
print(list(a), list(d), s.isSameTree(a, d))

g = TreeNode(1)
h = TreeNode(2)
g.left = h
i = TreeNode(1)
j = TreeNode(2)
i.right = j
print(list(g), list(i), s.isSameTree(g, i))

k = TreeNode(1)
l = TreeNode(2)  # noqa: E741
m = TreeNode(1)
k.left = l
k.right = m
n = TreeNode(1)
o = TreeNode(1)
p = TreeNode(2)
n.left = o
n.right = p
print(list(k), list(n), s.isSameTree(k, n))

print(s.isSameTree(k, None))
print(s.isSameTree(None, None))
