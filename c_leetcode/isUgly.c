#include "stdio.h"
#include "stdbool.h"
#include <assert.h>

bool isUgly(int n)
{
    if (n <= 0) {
        return false;
    }
    int factors[3] = { 2, 3, 5 };

    for (int i = 0; i < 3; i++) {
        while (n % factors[i] == 0) {
            n /= factors[i];
        }
    }

    return n == 1;
}


int main()
{
    //示例1
    assert(isUgly(6) == 1);

    //示例2
    assert(isUgly(1) == 1);

    //示例3
    assert(isUgly(14) == 0);

    return 0;
}