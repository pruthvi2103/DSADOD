#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
    int T;
    cin >> T;
    for (int z = 0; z < T; z++)
    {
        int N, dist = 0;
        cin >> N;
        int cars[N];
        for (int i = 0; i < N; i++)
        {
            cin >> cars[i];
        }
        if (cars[0] == 0)
        {
            cout << 0;
            continue;
        }
        else
        {
            int fuel = cars[0];
            for (int i = 1; i < N; i++)
            {
                fuel--;
                dist++;
                if (fuel == 0 && cars[i] == 0)
                    break;
                fuel += cars[i];
            }
            dist += fuel;
            cout << dist;
        }
    }
}