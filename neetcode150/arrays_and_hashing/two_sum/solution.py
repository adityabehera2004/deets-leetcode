from typing import List

class Solution:
    def two_sum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i in range(len(nums)):
            if target - nums[i] in seen:
                return [seen[target - nums[i]], i]
            seen[nums[i]] = i
        return []

if __name__ == '__main__':
    solution = Solution()
    assert solution.two_sum([3,4,5,6], 7) == [0,1], "Example 1 failed"
    assert solution.two_sum([4,5,6], 10) == [0,2], "Example 2 failed"
    assert solution.two_sum([5,5], 10) == [0,1], "Example 3 failed"
    print("All test cases passed!")