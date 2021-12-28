import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
ps = []
for a, b in [ti() for _ in range(N)]:
    ps.append(b)
    ps.append(a-b)

ps.sort(reverse=True)
print(sum(ps[:K]))
