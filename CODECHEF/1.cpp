#include <iostream>
#include <algorithm>
using namespace std;

int main()
{
    int T, I, time;
    cin >> T;
    for (int z = 0; z < T; z++)
    {
        cin >> I;
        int b[] = {0, 0};
        int dish[I];
        time = 0;
        for (int i = 0; i < I; i++)
        {
            cin >> dish[i];
        }
        std::sort(dish, dish + I);
        b[0] = dish[I - 1];
        b[1] = dish[I - 2];
        for (int j = I - 3; j >= 0;)
        {
            //cout << time;
            if (b[0] > b[1])
            {
                b[0] = b[0] - b[1];
                time += b[1];
                b[1] = dish[j];
                j--;
            }
            else if (b[0] < b[1])
            {
                b[1] = b[1] - b[0];
                time += b[0];
                b[0] = dish[j];
                j--;
            }
            else if (b[0] == b[1])
            {
                time += b[0];
                //cout << j;
                if (j >= 1)
                {
                    j--;
                    b[0] = dish[j];
                    j--;
                    b[1] = dish[j];
                }
                else
                {
                    b[0] = 0;
                    b[1] = dish[j];
                    j--;
                }
            }
            if (j == -1)
            {
                time += dish[0];
                break;
            }
        }
        cout << time;
    }
    return 0;
}