import bisect
import array
l, q = map(int, input().split())

A = array.array("i", [0, l])

ans = []
for i in range(q):
    c, x = map(int, input().split())
    if c == 2:
        idx = bisect.bisect(A, x)
        ans.append(A[idx]-A[idx-1])
    else:
        bisect.insort(A, x)

print(*ans)
