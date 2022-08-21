#include "stdio.h"
#include <assert.h>

int reverse(int x)
{
    long ans = 0;
    int num = x;
    while (num != 0) {
        ans = ans * 10 + num % 10;
        num /= 10;
    }
    return (int)ans != ans ? 0 : ans;
}

int main()
{
    //示例1
    assert(reverse(123) == 321);
    //示例2
    assert(reverse(-123) == -321);
    //示例3
    assert(reverse(120) == 21);
    //示例4
    assert(reverse(0) == 0);
}