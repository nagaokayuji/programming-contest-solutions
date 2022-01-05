from collections import defaultdict, Counter, deque
N = int(input())
S = input()

q = deque()

for i in range(N):
    x = S[i]
    if not q:
        q.append(x)
        continue
    if q[0] == x:
        q.popleft()
        continue
    if q[-1] == x:
        q.pop()
        continue
    if i == N-1:
        q.append(x)
        continue

    if len(q) == 1:
        q.append(x)
        continue
    if len(q) == 2:
        nx = S[i+1]
        if x == nx:
            q.append(x)
            continue
        if q[0] == nx:
            q.append(x)
            continue
        if q[-1] == nx:
            q.appendleft(x)
            continue

print(len(q))


'''
RGBRGB

'''
