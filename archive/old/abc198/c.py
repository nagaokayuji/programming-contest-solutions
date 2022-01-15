import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    R, X, Y = readti()
    dist2 = X*X + Y*Y
    if R*R > dist2:
        print(2)
        return
    for count in range(10**8):
        if (R*count)**2 >= dist2:
            print(count)
            return
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
