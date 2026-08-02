class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        # Two integer arrays nums1 and nums2, each sorted in ascending order
        # Return median of the two arrays
        # O(log(m+n)) time
        #
        # Joining the array would cost at least O(m+n), soit is out of question.
        # The complexity suggest some kind of binary search spanning both arrays.
        # How should it be implemented?
        #
        # Assuming that len() is a O(1) operation given the list implementation (commonly it is),
        # with |nums1| = A and |nums2| = B,
        # what I should do is find elements a and b from nums1 and nums2, respectively,
        # such that the combined elements to their left are the
        # (A+B)/2-th (odd) or (A+B+1)/2-th (even) smallest elements of them combined.
        #
        # Suppose a and b are index for [:i] ranges to each array
        # Initially a = A and b = B, so that it covers [0, A) and [0, B]
        #
        # (Erase) Yea I'm still suck with the relationship between math and code
        #
        # Rule:
        #
        # - If a+b > (A+B)/2 (odd) or (A+B+1)/2 (even)
        #   - If nums1[a-1] < nums2[b-1]
        #     - Reduce
        # - If a+b < (A+B)/2 (odd) or (A+B+1)/2 (even)
        #   - If nums1[a] < nums2[b], move a
        pass
