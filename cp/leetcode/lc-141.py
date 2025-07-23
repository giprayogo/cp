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
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head is None or head.next is None:
            return False
        slow = head
        fast = head.next
        while fast and fast.next:
            if slow == fast:
                return True
            slow = slow.next
            fast = fast.next.next
        return False


def main():
    list1 = ListNode.try_from_iterable([3, 2, 0, -4])
    print(list(list1))
    second = list1.next
    fourth = list1.next.next.next
    fourth.next = second
    list2 = ListNode.try_from_iterable([1, 2])
    print(list(list2))
    list2.next.next = list2
    list3 = ListNode.try_from_iterable([1])
    print(list(list3))

    # Infinite loops
    # print(list(list1))
    # print(list(list2))

    s = Solution()
    print(s.hasCycle(list1))
    print(s.hasCycle(list2))
    print(s.hasCycle(list3))


if __name__ == "__main__":
    main()
