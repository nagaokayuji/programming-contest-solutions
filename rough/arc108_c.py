import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))


class DSU:
    def __init__(self, n: int = 0):
        self._n = n
        self.parent_or_size = [-1] * n

    def merge(self, a: int, b: int):
        x = self.leader(a)
        y = self.leader(b)

        if x == y:
            return x

        if -self.parent_or_size[x] < -self.parent_or_size[y]:
            x, y = y, x

        self.parent_or_size[x] += self.parent_or_size[y]
        self.parent_or_size[y] = x
        return x

    def same(self, a: int, b: int):
        return self.leader(a) == self.leader(b)

    def leader(self, a: int):
        parent = self.parent_or_size[a]
        while parent >= 0:
            if self.parent_or_size[parent] < 0:
                return parent
            self.parent_or_size[a], a, parent = (
                self.parent_or_size[parent],
                self.parent_or_size[parent],
                self.parent_or_size[self.parent_or_size[parent]]
            )

        return a

    def size(self, a: int):
        return -self.parent_or_size[self.leader(a)]

    def groups(self):
        leader_buf = [self.leader(i) for i in range(self._n)]
        result = [[] for _ in range(self._n)]
        for i in range(self._n):
            result[leader_buf[i]].append(i)
        return list(filter(lambda r: r, result))


def _solve():
    N, M = li()
    UVC = [tuple(li1()) for _ in range(M)]
    dsu = DSU(N)
    g = [[] for _ in range(N)]
    for u, v, c in UVC:
        if not dsu.same(u, v):
            dsu.merge(u, v)
            g[u].append((v, c))
            g[v].append((u, c))

    ans = [None]*N
    ans[0] = 0

    def dfs(v=0, prev=None):
        for to, c in g[v]:
            if to == prev:
                continue
            if ans[v] == c:
                ans[to] = 1 if c == 0 else 0
            else:
                ans[to] = c
            dfs(to, v)

    dfs()
    for x in ans:
        print(x+1)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
