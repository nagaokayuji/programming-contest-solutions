from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, M, Q = mi()
WV = sorted([tuple(mi()) for _ in range(N)], key=lambda x: x[1])
X = li()


def solve(l, r):
    l -= 1
    r -= 1
    ret = 0
    ls = sorted(X[:l] + X[r+1:])
    used = [False] * N
    for box in ls:
        mxi, mxv = None, 0
        for i, (w, v) in enumerate(WV):
            if not used[i] and box >= w and mxv < v:
                mxi = i
                mxv = v
        if mxi is not None:
            used[mxi] = True
            ret += mxv
    print(ret)


for _ in range(Q):
    l, r = mi()
    solve(l, r)
