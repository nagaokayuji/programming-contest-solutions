import sys
from collections import defaultdict, Counter, deque
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    H, W = mi()
    C = [input() for _ in range(H)]

    for i in range(H):
        C[i] += C[i]*100

    W = len(C[0])
    visited = [[False]*(W) for _ in range(H)]
    visited[0][0] = True
    q = deque()
    q.append((0, 0))
    while q:
        i, j = q.popleft()
        visited[i][j] = True
        if i+1 < H and C[i+1][j] == '.':
            if not visited[i+1][j]:
                visited[i+1][j] = True
                q.append((i+1, j))
        if 0 <= j+1 < W and C[i][j+1] == '.':
            if not visited[i][j+1]:
                visited[i][j+1] = True
                q.append((i, j+1))

    if visited[H-1][W-1]:
        print("Yay!")
    else:
        print(":(")


if __name__ == '__main__':
    _solve()
