M, K = map(int, input().split())

if K >= pow(2, M):
    print(-1)
    exit()

if M == 0:
    if K == 0:
        print(0, 0)
    else:
        print(-1)
elif M == 1:
    if K == 0:
        print(0, 0, 1, 1)
    else:
        print(-1)
else:
    l = [K]
    for i in range(1 << M):
        if i == K:
            continue
        l.append(i)
    print(*l, K, *l[:0:-1])
