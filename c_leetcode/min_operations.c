#include "stdio.h"
#include <stdlib.h>
#include <assert.h>

int minOperations(int *nums, int numsSize)
{
    int mix_op = 0;

    for (int i = 1; i < numsSize; i++) {
        int tmp = 0;

        if (nums[i] <= nums[i - 1]) {
            tmp = abs(nums[i] - nums[i - 1]) + 1;
            mix_op += tmp;
            nums[i] += tmp;
        }
    }

    return mix_op;
}

int main()
{
    //示例1
    int ums1[] = {1, 1, 1};
    assert(minOperations(ums1, 3) == 3);

    //示例2
    int ums2[] = {1,5,2,4,1};
    assert(minOperations(ums2, 5) == 14);
    //示例3
    int ums3[] = {8};
    assert(minOperations(ums3, 1) == 0);
}