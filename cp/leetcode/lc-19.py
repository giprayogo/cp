from typing import Optional, TypeVar, Type, Iterable, Iterator

T = TypeVar("T", bound="ListNode")


class ListNode:
    """
    LeetCode's definition for singly-linked list.
    Plus I added some methods for easier debugging.
    """

    def __init__(self, val: int = 0, next: Optional[Type[T]] = None):
        self.val = val
        self.next = next

    def __iter__(self) -> Iterator[int]:
        cursor = self
        yield cursor.val
        while cursor.next is not None:
            cursor = cursor.next
            yield cursor.val

    @classmethod
    def try_from_iterable(cls: Type[T], elements: Iterable) -> Optional[T]:
        dummy_head = ListNode(0)
        cursor = dummy_head
        for e in elements:
            cursor.next = ListNode(e)
            cursor = cursor.next
        return dummy_head.next


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy_head = ListNode(0)
        dummy_head.next = head
        left = right = dummy_head
        for i in range(n):
            right = right.next
        while right and right.next:
            left = left.next
            right = right.next
        left.next = left.next.next
        return dummy_head.next


def main():
    l1 = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    l1b = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    l1c = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    l1d = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    l1e = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    l2 = ListNode.try_from_iterable([1])
    l3 = ListNode.try_from_iterable([1, 2])

    s = Solution()
    l1 = s.removeNthFromEnd(l1, 1)
    l1b = s.removeNthFromEnd(l1b, 2)
    l1c = s.removeNthFromEnd(l1c, 3)
    l1d = s.removeNthFromEnd(l1d, 4)
    l1e = s.removeNthFromEnd(l1e, 5)
    l2 = s.removeNthFromEnd(l2, 1)
    l3 = s.removeNthFromEnd(l3, 1)
    print(list(l1))
    print(list(l1b))
    print(list(l1c))
    print(list(l1d))
    print(list(l1e))
    print(l2)
    print(list(l3))


if __name__ == "__main__":
    main()
