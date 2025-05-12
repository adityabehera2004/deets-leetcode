class Solution:
    def valid_anagram(self, s: str, t: str) -> bool:
        return sorted(s) == sorted(t)

if __name__ == '__main__':
    solution = Solution()
    assert solution.valid_anagram("racecar", "carrace") == True, "Example 1 failed"
    assert solution.valid_anagram("jar", "jam") == False, "Example 2 failed"
    print("All test cases passed!")