T = int(input())

for _ in range(T):
    L = int(input())
    s = input()
    if s == "2020":
        print("YES")
    elif s[0:4] == "2020":
        print("YES")
    elif s[0:2] == "20" and s[L-2:L] == "20":
        print("YES")
    elif s[L-4:L] == "2020":
        print("YES")
    elif s[0] == "2" and s[L-3:L] == "020":
        print("YES")
    elif s[0:3] == "202" and s[L-1] == "0":
        print("YES")
    else:
        print("NO")
