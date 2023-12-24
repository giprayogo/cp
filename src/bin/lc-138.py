from typing import Optional, TypeVar, Type

T = TypeVar("T", bound="Node")


class Node:
    """
    LeetCode's definition for singly-linked list.
    Plus I added some methods for easier debugging.
    """

    def __init__(
        self,
        val: int = 0,
        next: Optional[Type[T]] = None,
        random: Optional[Type[T]] = None,
    ):
        self.val = val
        self.next = next
        self.random = None

    def __iter__(self):
        cursor = self
        yield [cursor.val, cursor.random.val if cursor.random else ""]
        while cursor.next is not None:
            cursor = cursor.next
            yield [cursor.val, cursor.random.val if cursor.random else ""]

    @classmethod
    def try_from_iterable(cls: Type[T], elements) -> Optional[T]:
        dummy_head = Node(0)
        cursor = dummy_head

        nodes = []
        rs = []
        for e, r in elements:
            node = Node(e)
            cursor.next = node
            cursor = cursor.next

            nodes.append(node)
            rs.append(r)
        # Connect the random edge
        for node, r in zip(nodes, rs):
            if r is not None:
                node.random = nodes[r]
        return dummy_head.next


class Solution:
    # def copyRandomList(self, head: Optional[Type[T]]) -> Optional[Type[T]]:
    #     if head is None:
    #         return None
    #     dummy_head = Node(0)

    #     cursor = head
    #     newcursor = dummy_head
    #     rands = {}
    #     oldnew = {}
    #     while cursor is not None:
    #         node = Node(cursor.val)

    #         rands[node] = cursor.random
    #         oldnew[cursor] = node
    #         newcursor.next = node

    #         cursor = cursor.next
    #         newcursor = newcursor.next

    #     for node, oldrand in rands.items():
    #         if oldrand is not None:
    #             node.random = oldnew[oldrand]

    #     return dummy_head.next
    # NeetCode's solution... much simpler!
    def copyRandomList(self, head):
        oldnew = {None: None}

        cursor = head
        while cursor:
            node = Node(cursor.val)
            oldnew[cursor] = node
            cursor = cursor.next
        cursor = head
        while cursor:
            new = oldnew[cursor]
            new.next = oldnew[cursor.next]
            new.random = oldnew[cursor.random]
            cursor = cursor.next
        return oldnew[head]


def main():
    list1 = Node.try_from_iterable([[7, None], [13, 0], [11, 4], [10, 2], [1, 0]])
    list2 = Node.try_from_iterable([[1, 1], [2, 1]])
    list3 = Node.try_from_iterable([[3, None], [3, 0], [3, None]])
    print(list(list1))
    print(list(list2))
    print(list(list3))

    s = Solution()
    list1_cp = s.copyRandomList(list1)
    list2_cp = s.copyRandomList(list2)
    list3_cp = s.copyRandomList(list3)
    print(list(list1_cp))
    print(list(list2_cp))
    print(list(list3_cp))


if __name__ == "__main__":
    main()
