import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
N = list(map(int, list(str(N))))


def op(n):
    val = 0
    b = 1
    while n:
        val += b * n.pop()
        b *= 8

    ret = []
    while val:
        x = val % 9
        ret.append(x if x != 8 else 5)
        val //= 9
    while ret and ret[-1] == 0:
        ret.pop()
    return ret[::-1]


ans = N
for _ in range(K):
    ans = op(ans)
if not ans:
    ans.append(0)
print("".join(map(str, ans)))
