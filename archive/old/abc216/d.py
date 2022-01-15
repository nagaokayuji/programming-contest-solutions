from collections import deque
N, M = map(int, input().split())

A = [deque() for _ in range(M)]
top = [[] for _ in range(N)]
q = deque()
for i in range(M):
    k = int(input())
    l = list(map(lambda x: x-1, map(int, input().split())))
    A[i] = deque(l)
    top[l[0]].append(i)  # queue index
    if len(top[l[0]]) == 2:
        q.append(l[0])
cnt = 0
while q:
    now = q.popleft()
    cnt += 1
    x, y = top[now]
    A[x].popleft()
    A[y].popleft()
    if len(A[x]):
        nx = A[x][0]
        top[nx].append(x)
        if len(top[nx]) == 2:
            q.append(nx)
    if len(A[y]):
        ny = A[y][0]
        top[ny].append(y)
        if len(top[ny]) == 2:
            q.append(ny)

if cnt == N:
    print("Yes")
else:
    print("No")
