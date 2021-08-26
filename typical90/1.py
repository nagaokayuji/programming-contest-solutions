import numba
from numba import njit
import numpy as np
N, L = map(int, input().split())
K = int(input())
A = np.array(list(map(int, input().split())))
A = np.append(A, L)
B = A.copy()
for i in range(1, N+1):
    B[i] = A[i] - A[i-1]
'''
K 個の切れ目を選ぶ
→ K+1 の範囲に分かれる
その最小値を最大にする

3 34
1
8 13 26


↓

13

7 45
2
7 11 16 20 28 34 38

↓

12


3 100
1
28 54 81

↓

46
'''

INF = 10**18
NG = INF
OK = 0


@njit(cache=True)
def check(x, B, K):
    # すべて x 以上のk+1区間に分割できるか？
    cnt = 0
    sum = 0
    for d in B:
        sum += d
        if sum >= x:
            cnt += 1
            sum = 0
    return cnt >= K+1


while NG - OK > 1:
    mid = (OK + NG) // 2
    if check(mid, B, K):
        OK = mid
    else:
        NG = mid
print(OK)
