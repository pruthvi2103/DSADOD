T = int(input())
for _ in range(T):
    N = int(input())
    F = list(map(int, input().split(' ')))
    C = list(map(int, input().split(' ')))
    OPT = [fuel for i, fuel in sorted(zip(C, F))]
    C = sorted(C)
    coins = 0
    dist = N
    for x in range(N):
        if OPT[x] < dist:
            c = OPT[x] * C[x]
        else:
            c = dist * C[x]
            break
        coins += c
        dist -= OPT[x]
    print(coins)
