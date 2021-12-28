import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, Q = mi()
A = li()
Qs = [ti() for _ in range(Q)]
shift = 0
ans = []
for t, x, y in Qs:
    if t == 1:
        x -= 1
        y -= 1
        A[(x+shift) % N], A[(y+shift) % N] = A[(y+shift) % N], A[(x+shift) % N]

    elif t == 2:
        shift = (shift + N-1) % N

    else:
        x -= 1
        ans.append(str(A[(x+shift) % N]))

print(*ans)
