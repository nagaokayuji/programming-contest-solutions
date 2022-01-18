import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


class FenwickTree:
    def __init__(self, n: int = 10**6):
        self._n = n
        self.data = [0] * n

    def add(self, p: int, x):
        p += 1
        while p <= self._n:
            self.data[p-1] = max(self.data[p-1], x)
            p += p & -p

    def sum(self, left: int, right: int):
        return self._sum(right) - self._sum(left)

    def _sum(self, r: int):
        s = 0
        while r > 0:
            s = max(s, self.data[r-1])
            r -= r & -r
        return s


def _solve():
    N = int(input())
    h = list(mi())
    hi = [(x, i) for i, x in enumerate(h)]
    hi.sort()
    a = list(mi())
    dp = FenwickTree(N+2)
    ans = 0
    for _, i in hi:
        val = dp._sum(i)+a[i]
        dp.add(i, val)
        ans = max(ans, val)
    print(dp._sum(N+1))


if __name__ == '__main__':
    _solve()
