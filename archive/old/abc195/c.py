import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def solve():
    N = readi()
    if N < 1000:
        print(0)
        return
    ans = 0
    for ketasu in range(1, 17):
        if len(str(N)) < ketasu:
            break
        comma = (ketasu-1)//3
        kosu = min(N, 10**ketasu - 1) - 10**(ketasu-1) + 1
        # print(ketasu, comma, kosu)
        ans += comma * kosu

    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
