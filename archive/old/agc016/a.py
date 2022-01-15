s = list(map(lambda x: ord(x) - ord('a'), list(input())))

ans = 101

for ch in range(26):
    t = s.copy()
    if not ch in s:
        continue
    if all(map(lambda x: x == ch, t)):
        ans = 0
        break
    for count in range(1, 300):
        pt = t.copy()
        for i in range(len(t)-1):
            if pt[i] == ch or pt[i+1] == ch:
                t[i] = ch
        t = t[:-1]
        if all(map(lambda x: x == ch, t)):
            ans = min(ans, count)
            break
print(ans)
