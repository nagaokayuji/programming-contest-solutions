import sys
from collections import defaultdict, Counter, deque
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
MOD = 46
ac = Counter(np.array(li(), dtype=np.int64) % MOD)
bc = Counter(np.array(li(), dtype=np.int64) % MOD)
cc = Counter(np.array(li(), dtype=np.int64) % MOD)
ans = 0
for a in range(MOD):
    for b in range(MOD):
        c = (MOD+MOD-a-b) % MOD
        ans += ac[a]*bc[b]*cc[c]
print(ans)
