import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    CSF = [readti() for _ in range(N-1)]
    for i, (c, s, f) in enumerate(CSF):
        t = s+c
        for nc, ns, nf in CSF[i+1:]:
            t = max(t, ns)
            t = nf * ((t+nf-1)//nf) + nc
        print(t)

    print(0)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
