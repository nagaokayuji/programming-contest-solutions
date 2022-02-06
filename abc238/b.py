import sys
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, = mi()
    A = list(mi())

    s = 0
    cs = list(np.cumsum([0]+A) % 360)
    cs.sort()
    cs.append(360)
    ans = 0
    for i in range(N+1):
        ans = max(ans, cs[i+1] - cs[i])
    print(ans)


if __name__ == '__main__':
    _solve()
