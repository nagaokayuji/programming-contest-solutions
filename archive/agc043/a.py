import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    H, W = readmi()
    S = [input() for _ in range(H)]

    q = deque()
    dp = [[INF]*W for _ in range(H)]
    q.append((0, 0, 0 if S[0][0] == '.' else 1, S[0][0] == '#'))
    while q:
        nowi, nowj, count, breaking = q.popleft()
        if dp[nowi][nowj] < count:
            continue
        dp[nowi][nowj] = min(dp[nowi][nowj], count)
        for di, dj in [(1, 0), (0, 1)]:
            nxi, nxj = nowi+di, nowj+dj
            if not (0 <= nxi < H and 0 <= nxj < W):
                continue
            if dp[nxi][nxj] <= count:
                continue
            if not breaking:
                if S[nxi][nxj] == '#':
                    q.append((nxi, nxj, count+1, True))
                    dp[nxi][nxj] = min(dp[nxi][nxj], count+1)
                else:
                    q.appendleft((nxi, nxj, count, False))
                    dp[nxi][nxj] = min(dp[nxi][nxj], count)
            else:  # breaking
                if S[nxi][nxj] == '#':
                    q.appendleft((nxi, nxj, count, True))
                    dp[nxi][nxj] = min(dp[nxi][nxj], count)
                else:
                    q.appendleft((nxi, nxj, count, False))
                    dp[nxi][nxj] = min(dp[nxi][nxj], count)

    print(dp[H-1][W-1])


if __name__ == '__main__':
    _solve()
