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
    # TLE Naive solution
    # def reorderList(self, head: Optional[ListNode]) -> None:
    #     """
    #     Do not return anything, modify head in-place instead.
    #     """
    #     if head is None:
    #         return

    #     left = head
    #     right = head
    #     while left.next is not None:
    #         while right.next.next is not None:
    #             right = right.next
    #         if left == right:
    #             break
    #         temp = left.next
    #         left.next = right.next
    #         right.next = None
    #         left.next.next = temp
    #         right = left = temp

    def reorderList(self, head: Optional[ListNode]) -> None:
        if head is None or head.next is None:
            return
        slow = head
        fast = head.next
        while fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next
        middle = slow.next

        slow.next = None
        current = middle
        prev = None
        while current is not None:
            next = current.next
            current.next = prev
            prev = current
            current = next

        dummy_head = ListNode(0)
        cursor = dummy_head
        left = True
        while cursor is not None:
            if head is None:
                cursor.next = prev
                break
            elif prev is None:
                cursor.next = head
                break
            elif left:
                cursor.next = head
                head = head.next
                cursor = cursor.next
                left = False
            else:
                cursor.next = prev
                prev = prev.next
                cursor = cursor.next
                left = True
        head = dummy_head.next

    def zip_list(self, head_a: Optional[ListNode], head_b=Optional[ListNode]):
        dummy_head = ListNode(0)
        cursor = dummy_head
        left = True
        while cursor is not None:
            if head_a is None:
                cursor.next = head_b
                break
            if head_b is None:
                cursor.next = head_a
                break
            # NOTE: zip, genki this is wrong
            # if head_a.val < head_b.val:
            if left:
                cursor.next = head_a
                head_a = head_a.next
                cursor = cursor.next
                left = False
            else:
                cursor.next = head_b
                head_b = head_b.next
                cursor = cursor.next
                left = True
        return dummy_head.next


def main():
    s = Solution()

    ln0 = ListNode.try_from_iterable([])
    ln1 = ListNode.try_from_iterable([1])
    ln2 = ListNode.try_from_iterable([1, 2, 3, 4])
    ln3 = ListNode.try_from_iterable([1, 2, 3, 4, 5])
    ln4 = ListNode.try_from_iterable(range(11))
    print(ln0)
    print(list(ln1))
    print(list(ln2))
    print(list(ln3))
    print(list(ln4))
    s.reorderList2(ln0)
    s.reorderList2(ln1)
    s.reorderList2(ln2)
    s.reorderList2(ln3)
    s.reorderList2(ln4)
    print(ln0)
    print(list(ln1))
    print(list(ln2))
    print(list(ln3))
    print(list(ln4))


if __name__ == "__main__":
    main()
