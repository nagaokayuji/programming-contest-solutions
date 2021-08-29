from heapq import *

N, K = map(int, input().split())
A = list(map(int, input().split()))

A.sort()
A.reverse()


INF = 10**19
ok = INF
ng = 0


while ok-ng > 1:
    mid = (ok+ng)//2
    cnt = 0
    for x in A:
        if mid > x:
            break
        cnt += x - mid + 1
    if cnt <= K:
        ok = mid
    else:
        ng = mid
# print(ok)

ans = 0
cnt = 0
for x in A:
    if x < ok:
        break
    add = (x-ok + 1) * (x + ok)//2
    # print(ok, x, add)
    ans += add
    cnt += x-ok+1


print(ans + (ok-1) * (K - cnt))
