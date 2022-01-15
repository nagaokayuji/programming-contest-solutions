import sys
from bisect import bisect_left, bisect_right
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    s = input()
    n = len(s)
    t = input()

    s = s+s
    poss = [[] for _ in range(26)]

    for i, c in enumerate(s):
        cind = ord(c) - ord('a')
        poss[cind].append(i)

    prev = -1
    count = 0
    for c in t:
        cind = ord(c) - ord('a')
        if not poss[cind]:
            print(-1)
            return
        i = bisect_left(poss[cind], prev+1)
        prev = poss[cind][i]
        if prev >= n:
            prev %= n
            count += 1
    print(prev+1 + n*count)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
