#include "stdio.h"
#include <assert.h>

int majorityElement(int* nums, int numsSize)
{
    int i, now, count;
    now = 0;
    count = 0;

    for (i = 0; i < numsSize; i++) {
        if (count == 0)
            now = nums[i];

        if (now == nums[i])
            count++;
        else
            count--;
    }

    return now;
}

int main()
{
    //示例1
    int nums[3] = { 3, 2, 3 };
    assert(majorityElement(nums, 3) == 3);
    //示例2
    int arr[7] = { 2,2,1,1,1,2,2 };
    assert(majorityElement(arr, 7) == 2);
}