from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
AB = sorted([ti() for _ in range(N)], key=lambda x: (-x[0], -x[1]))
ps = []
for a, b in AB:
    ps.append(b)
    ps.append(a-b)

ps.sort()
ps.reverse()
print(sum(ps[:K]))
