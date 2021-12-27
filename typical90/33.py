from pprint import pprint
from math import ceil
INF = float('inf')
def mi(): return map(int, input().split())


H, W = mi()
if W > H:
    H, W = W, H

if H < 2 or W < 2:
    print(H*W)
    exit()

hh = ceil(H/2)
ww = ceil(W/2)

print(hh*ww)
