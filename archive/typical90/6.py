import bisect
N, K = map(int, input().split())
S = list(map(lambda x: ord(x) - ord('a'), input()))

chs = [[] for _ in range(26)]
for i, x in enumerate(S):
    chs[x].append(i)

ans = ""
index = 0

for count in range(K):
    left = K - count
    for chi, ch in enumerate(chs):
        if not ch:
            continue
        i = bisect.bisect_left(ch, index)
        if i == len(ch) or N-ch[i] < left:
            continue
        ans += chr(ord("a") + chi)
        index = ch[i] + 1
        break

print(ans)
