
from collections import Counter

h, w = map(int, input().split())
cnt = [0]*101
a = [list(map(int, input().split())) for _ in range(h)]
fl = []
for r in a:
    for x in r:
        fl.append(x)

mn = min(fl)

print(sum(fl) - mn*h*w)
