#include <iostream>
using namespace std;

class Solution
{
public:
    // k>=0 needed, returns 2^k
    int power_2(int k)
    {
        return 1 << k;
    }
    // takes x,k>=0--gives x*2^k
    int mult_power_2(int x, int k)
    {
        return x << k;
    }
    // must input nonnegative x
    int msb_index(int x)
    {
        int index = -1;
        while (x > 0)
        {
            ++index;
            x >>= 1;
        }
        return index;
    }
    // assumes x,y>=0

    const int int_min = -2147483648;
    const int int_max = 2147483647;

//assume d!=0
int divide(int x, int d) {
    bool same = true;

    if (x == int_min)
        {
            if (d == 1)
            {
                return int_min;
            }
            if (d == -1)
            {
                return int_max;
            }
            if (d== int_min){
                return 1;
            }
            if (d>0) {
                same = false;

            }
            else {
                d = -d;
            }
            const int n = msb_index(d);
            const int k = 30 - n;
            const int q = divide(int_min+ mult_power_2(d, k), d);
            const int result = power_2(k) - q;
            
            if (!same)
            {
                return -result;
            }

            return result;
        }

    if (d == int_min) return x == int_min ? 1 : 0;

    if (abs(x) < abs(d)) return 0;

    // normalize signs
    if (x < 0 && d < 0) {
        x = -x;  d = -d;
    } else if (x < 0) {
        same = false;  x = -x;
    } else if (d < 0) {
        same = false;  d = -d;
    }

    if (x == d) 
        return same ? 1 : -1;

    const int n = msb_index(d);
    const int m = msb_index(x);
    int k = m - n;
    //here we always underapproximate x -- it can be shown that at this point, if 2^kd>x, then 2^(k-1) d <= x
    if (mult_power_2(d, k) > x) --k;

    const int q = divide(x - mult_power_2(d, k), d);
    int result = power_2(k) + q;

    if (!same) {
        result = -result;
        if (result <= int_min) return int_min;
    } else {
        if (result >= int_max) return int_max;
    }
    return result;
}


};
