N = int(input())
S = list(map(lambda x: ord(x)-ord('a'), list(input())))

ans = 0
for i, x in enumerate(S):
    ans += S[i] * 2**i
print(ans)
