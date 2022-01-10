import sys
from bisect import bisect_left, bisect_right
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    N = int(input())
    S = list(mi())
    Q = int(input())
    C = list(mi())
    S.sort()
    ans = 0
    for c in C:
        i = bisect_left(S, c)
        if S[i % N] == c:
            ans += 1
    print(ans)


if __name__ == '__main__':
    _solve()
