import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    A = readli()

    ans = 0
    right = 0
    prev = -1
    for left in range(N):
        right = max(left, right)
        if right == left:
            prev = -1
        while right < N and prev < A[right]:
            prev = A[right]
            right += 1
        ans += right-left
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
