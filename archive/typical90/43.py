from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


H, W = mi()
sr, sc = ti()
gr, gc = ti()
sr -= 1
sc -= 1
gr -= 1
gc -= 1
S = [input() for _ in range(H)]


dp = [[[INF for _ in range(4)] for _ in range(W)] for _ in range(H)]

dx = [-1, 0, 0, 1]
dy = [0, -1, 1, 0]
dp[sr][sc][0] = 0
dp[sr][sc][1] = 0
dp[sr][sc][2] = 0
dp[sr][sc][3] = 0
dq = deque()
# for i in range(4):
#     dq.append((sr, sc, i))
dq.append((sr, sc, -1))
while dq:
    i, j, dir = dq.popleft()
    if i == gr and j == gc:
        break

    for k in range(4):
        ni, nj = i+dx[k], j+dy[k]
        if 0 <= ni < H and 0 <= nj < W and S[ni][nj] == "." and dp[ni][nj][k] > dp[i][j][k]:
            if k == dir:
                dq.appendleft((ni, nj, k))
            else:
                dq.append((ni, nj, k))

            dp[ni][nj][k] = dp[i][j][k]
            for _i in range(4):
                if k != _i:
                    dp[ni][nj][_i] = min(dp[i][j][k]+1, dp[ni][nj][_i])

print(min(dp[gr][gc]))
