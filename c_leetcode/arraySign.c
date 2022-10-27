#include "stdio.h"
#include <assert.h>

int arraySign(int *nums, int numsSize)
{
    int cnt = 0;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] > 0) {
            continue;
        } else if (nums[i] == 0) {
            return 0;
        } else {
            cnt++;
        }
    }

    if (cnt % 2 == 0) {
        return 1;
    }
    return -1;
}


int main()
{
    int num1[] = {-1,-2,-3,-4,3,2,1};
    assert(arraySign(num1, 7) == 1);

    int num2[] = {1,5,0,2,-3};
    assert(arraySign(num2, 5) == 0);

    int num3[] = {-1,1,-1,1,-1};
    assert(arraySign(num3, 5) == -1);

    return 0;
}