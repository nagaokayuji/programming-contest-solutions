from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def ii(): return int(input())


N = ii()
ans = 0
r = 0
for i in range(2, int(N**0.5)+1):
    while N % i == 0:
        r += 1
        N //= i
if N > 1:
    r += 1

# print(r)
k = 1
while k < r:
    k <<= 1
    ans += 1
print(ans)
