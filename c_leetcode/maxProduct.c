#include "stdio.h"
#include <assert.h>

int maxProduct(int* nums, int numsSize)
{
    int i = 0, j = numsSize - 1;
    int ans = 0;

    while (i != j) {
        int tmp = (nums[i] - 1) * (nums[j] - 1);
        if (ans < tmp) {
            ans = tmp;
        }
        if (nums[i] > nums[j]) {
            j--;
        } else {
            i++;
        }
    }
    return ans;
}

int main()
{
    int nums1[4] = { 3, 4, 5, 2 };
    assert(maxProduct(nums1, 4) == 12);

    int nums2[4] = { 1, 5, 4, 5 };
    assert(maxProduct(nums2, 4) == 16);

    int nums3[2] = { 3, 7 };
    assert(maxProduct(nums3, 2) == 12);

    return 0;
}