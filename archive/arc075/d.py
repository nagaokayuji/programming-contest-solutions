from heapq import heappush, heappop, heapify, heappushpop, heapreplace
N, A, B = map(int, input().split())
H = [int(input()) for _ in range(N)]


def isok(c):
    count = 0
    for x in H:
        if c*B < x:
            count += (x - B*c + A-B-1)//(A-B)

    return count <= c


ok = 10**9
ng = 0
while ok-ng > 1:
    mid = (ok+ng)//2
    if isok(mid):
        ok = mid
    else:
        ng = mid

print(ok)
