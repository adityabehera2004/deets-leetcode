from typing import List

class Solution:
    def contains_duplicate(self, nums: List[int]) -> bool:
        return len(set(nums)) != len(nums)

if __name__ == '__main__':
    solution = Solution()
    assert solution.contains_duplicate([1, 2, 3, 3]) == True, "Example 1 failed"
    assert solution.contains_duplicate([1, 2, 3, 4]) == False, "Example 2 failed"
    print("All test cases passed!")