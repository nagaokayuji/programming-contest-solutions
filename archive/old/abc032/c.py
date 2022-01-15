import sys
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, K = readmi()
    S = [readi() for _ in range(N)]
    zero_c = S.count(0)
    if zero_c:
        print(N)
        return
    left = 0
    right = 0
    prod = 1
    ans = 0
    for left in range(N):
        right = max(right, left)
        while right < N and prod*S[right] <= K:
            prod *= S[right]
            right += 1
        ans = max(ans, right-left)
        if left < right:
            prod //= S[left]
    print(ans)


if __name__ == '__main__':
    _solve()
