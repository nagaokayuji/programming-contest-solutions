s = input()
n = len(s)

acgt = list("ACGT")
ans = 0
for l in range(len(s)):
    r = l
    while r < n and s[r] in acgt:
        r += 1
    ans = max(ans, r-l)

print(ans)
