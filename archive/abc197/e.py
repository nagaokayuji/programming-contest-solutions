from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    XC = [readti() for _ in range(N)]
    XC.sort()
    cs = [[] for _ in range(N+2)]
    for x, c in XC:
        cs[c].append(x)
    # for i in range(N):
    #     cs[i].sort()
    cs[N+1].append(0)
    cs[0].append(0)
    assert len(cs[0]) == 1
    assert len(cs[N+1]) == 1

    dp = [0]*2
    for color in range(N+1):
        ndp = [INF, INF]
        nx = cs[color+1]
        if not nx:
            cs[color+1] = cs[color]
            continue
        nxl = nx[0]
        nxr = nx[-1]
        assert nxl <= nxr

        for i, colind in enumerate((0, -1)):
            nowx = cs[color][colind]
            nowc = dp[i]

            if nxr < nowx:
                ndp[0] = min(ndp[0], nowc + (nowx-nxl))
                ndp[1] = min(ndp[1],
                             nowc + (nowx-nxl + nxr-nxl))
            elif nowx < nxl:
                ndp[1] = min(ndp[1], nowc + (nxr-nowx))
                ndp[0] = min(ndp[0],
                             nowc + (nxr-nxl)*2 + nxl-nowx)

            elif nxl <= nowx <= nxr:
                ndp[0] = min(ndp[0], nowc + nxr-nxl + nxr-nowx)
                ndp[1] = min(ndp[1], nowc + nowx-nxl + nxr-nxl)
        dp = ndp

    assert dp[0] == dp[1]
    print(dp[0])
    return


if __name__ == '__main__':
    _solve()
