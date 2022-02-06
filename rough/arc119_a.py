N = int(input())


# a* 2^b + c = N

INF = 10**18
ans = INF
for b in range(61):
    # a=1
    B = 2**b
    if B > N:
        break

    a = N//B
    c = N - a*B
    ans = min(ans, a+b+c)
print(ans)
