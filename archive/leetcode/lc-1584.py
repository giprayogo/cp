import heapq


class DisjointSet:
    def __init__(self, value):
        self.value = value
        self.parent = self
        self.size = 1

    def find(self):
        root = self
        while root.parent != root:
            root = root.parent

        x = self
        while x.parent != root:
            parent = x.parent
            x.parent = root
            x = parent

        return root

    def union(self, other):
        x = self.find()
        y = other.find()

        if x != y:
            if x.size < y.size:
                x.parent = y
                y.size += x.size
            else:
                y.parent = x
                x.size += y.size

    def __str__(self):
        return f"{self.value} {self.size}"


class Solution:
    def minCostConnectPoints(self, points: list[list[int]]) -> int:
        edges = []
        for i in range(len(points)):
            for j in range(0, i):
                cost = abs(points[i][0] - points[j][0]) + abs(
                    points[i][1] - points[j][1]
                )
                edges.append((i, j, cost))

        # Join
        p = list(range(len(points)))
        edges.sort(key=lambda x: x[2])
        cost = 0
        for i, j, _cost in edges:
            pi = p[i]
            pj = p[j]
            if pi != pj:
                cost += _cost
                pij = min(pi, pj)
                for k in range(len(p)):
                    if p[k] == pi or p[k] == pj:
                        p[k] = pij
        return cost

    def minCostConnectPoints2(self, points: list[list[int]]) -> int:
        edges = []
        for i in range(len(points)):
            for j in range(0, i):
                cost = abs(points[i][0] - points[j][0]) + abs(
                    points[i][1] - points[j][1]
                )
                edges.append((i, j, cost))

        # Join
        edges.sort(key=lambda x: x[2])
        p = [DisjointSet(i) for i in range(len(points))]
        cost = 0
        for i, j, _cost in edges:
            pi = p[i].find()
            pj = p[j].find()
            if pi != pj:
                cost += _cost
                pi.union(pj)
        return cost

    def minCostConnectPoints3(self, points: list[list[int]]) -> int:
        adj_list = {}
        for i in range(len(points)):
            for j in range(0, i):
                cost = abs(points[i][0] - points[j][0]) + abs(
                    points[i][1] - points[j][1]
                )
                adj_list.setdefault(i, [])
                adj_list.setdefault(j, [])
                adj_list[i].append((cost, j))
                adj_list[j].append((cost, i))

        h = []
        for e in adj_list.get(0, []):
            heapq.heappush(h, e)
        f = {0}
        cost = 0
        while h:
            _cost, _v = heapq.heappop(h)
            if _v in f:
                continue
            f.add(_v)
            cost += _cost
            for e in adj_list[_v]:
                heapq.heappush(h, e)
        return cost


s = Solution()
for f in (s.minCostConnectPoints, s.minCostConnectPoints2, s.minCostConnectPoints3):
    assert f([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]) == 20
    assert f([[3, 12], [-2, 5], [-4, 1]]) == 18
    assert f([[2, -3], [-17, -8], [13, 8], [-17, -15]]) == 53
    assert f([[0, 0]]) == 0
