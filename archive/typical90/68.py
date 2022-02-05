from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


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


N = int(input())
Q = int(input())
dsu = DSU(N)
TXYV = [li() for _ in range(Q)]
cnts = [0]*(N-1)
for i in range(Q):
    TXYV[i][1] -= 1
    TXYV[i][2] -= 1
    if TXYV[i][0] == 0:
        cnts[TXYV[i][1]] = TXYV[i][3]

ps = [0]*N
for i in range(N-1):
    ps[i+1] = cnts[i]-ps[i]

for t, x, y, v in TXYV:
    if t == 0:
        dsu.merge(x, y)
    else:
        if not dsu.same(x, y):
            print("Ambiguous")
            continue
        if (x ^ y) & 1 == 0:
            print(v + ps[y]-ps[x])
        else:
            print(v+ps[x]+ps[y]-2*v)
