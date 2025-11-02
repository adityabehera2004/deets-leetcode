class Solution:
    def valid_anagram(self, s: str, t: str) -> bool:
        return sorted(s) == sorted(t)