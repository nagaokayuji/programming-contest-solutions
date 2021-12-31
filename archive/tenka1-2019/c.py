import sys
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    S = input()
    cb = [0]
    cw = [0]
    for x in S:
        if x == '.':
            cw.append(cw[-1]+1)
            cb.append(cb[-1])
        else:
            cw.append(cw[-1])
            cb.append(cb[-1]+1)
    ans = min(cb[-1], cw[-1])
    # wwbb
    for pos in range(N):
        ans = min(ans, cb[pos] + cw[-1]-cw[pos])
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
