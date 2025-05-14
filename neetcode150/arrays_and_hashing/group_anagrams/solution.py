from typing import List
from collections import defaultdict

class Solution:
    def group_anagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            count = [0] * 26
            for c in s:
                count[ord(c) - ord('a')] += 1
            groups[tuple(count)].append(s)
        return list(groups.values())

if __name__ == '__main__':
    solution = Solution()
    
    def to_comparable(lists):
        return set(frozenset(group) for group in lists)
    
    example1 = solution.group_anagrams(["act","pots","tops","cat","stop","hat"])
    expected1 = [["hat"],["act", "cat"],["stop", "pots", "tops"]]
    assert to_comparable(example1) == to_comparable(expected1), "Example 1 failed"
    
    example2 = solution.group_anagrams(["x"])
    expected2 = [["x"]]
    assert to_comparable(example2) == to_comparable(expected2), "Example 2 failed"
    
    example3 = solution.group_anagrams([""])
    expected3 = [["", ""]]
    assert to_comparable(example3) == to_comparable(expected3), "Example 3 failed"
    
    print("All test cases passed!")