# Implement a time-based KV store that supports
# - Storing multiple values for the same key at specified time stamps
# - Retrieving the key's value at a specified timestamp
#
# Constraints
# - 1 <= key.length, value.length <= 100
# - key and value lowercase english letters and digits
# - 1 <= timestamp <= 1000
#
# Recommended time and space complexity
# - O(1) time set
# - O(logn) time get
# - O(m*n) space, n total values per keys, m total number of keys (i.e. no extra space)
#
# A (hash) dict store with string keys and array of (timestamp, value) tuple trivially achieves
# the set and storage requirement, but will required O(n) get since I need to check all
# values for the timestamp comparison.
#
# What I need is an ordered data structure that has O(1) insertion
#
# Missing assumption: time stamp is monotonically increasing! So I don't need to care about
# ordering the inner value array.


class TimeMap:
    def __init__(self):
        self._store = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        # Store key with value at timestamp
        values = self._store.get(key, [])
        values.append((timestamp, value))
        self._store[key] = values

    def get(self, key: str, timestamp: int) -> str:
        # Returns most recent value of key that is older than timestamp, i.e.
        # prev_timestamp <= timestamp
        if (values := self._store.get(key)) is not None:
            # Do binary search here
            value = ""

            n = len(values)
            left = 0
            right = n - 1
            while left <= right:
                m = (left + right) // 2
                if values[m][0] > timestamp:
                    right = m - 1
                else:
                    value = values[m][1]
                    left = m + 1
            return value
        return ""


time_map = TimeMap()
time_map.set("alice", "happy", 1)
assert time_map.get("alice", 1) == "happy"
assert time_map.get("alice", 2) == "happy"
time_map.set("alice", "sad", 3)
assert time_map.get("alice", 3) == "sad"

print("OK")
