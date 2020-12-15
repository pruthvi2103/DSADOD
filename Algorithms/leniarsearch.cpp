#include <iostream>
using namespace std;

int main()
{
    int N, M, la = -2;
    cin >> N;
    cin >> M;
    int arr[N];
    for (int i = 0; i < N; i++)
    {
        cin >> arr[i];
        if (arr[i] == M)
        {
            la = i;
        }
    }
    cout << la + 1;
    return -1;
}