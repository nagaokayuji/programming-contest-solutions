from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, B = mi()


def f(x):
    now = x
    ret = 1
    while now:
        ret *= (now % 10)
        now //= 10
    return ret


ans = 0
for i in range(50):
    two = 2**i
    if two + B > N:
        break
    for j in range(50):
        three = two * (3**j)
        if three + B > N:
            break
        for k in range(50):
            five = three * (5**k)
            if five + B > N:
                break
            for l in range(50):
                seven = five * (7**l)
                m = seven + B
                if m > N:
                    break
                if f(m) == seven:
                    ans += 1

if N >= B and not f(B):
    ans += 1
print(ans)
