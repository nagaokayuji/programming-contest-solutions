N = input()

trzero = 0
for c in N[::-1]:
    if c == '0':
        trzero += 1
    else:
        break

lzero = 0
for c in N:
    if c == '0':
        lzero += 1
    else:
        break

if trzero >= lzero:
    s = '0'*(trzero-lzero)+N
    isp = True
    for i in range(len(s)//2):
        if s[i] != s[len(s)-1-i]:
            isp = False
            break

    if isp:
        print("Yes")
        exit()
print("No")
