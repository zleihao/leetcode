#include "stdio.h"
#include <stdlib.h>
#include <assert.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* shuffle(int* nums, int numsSize, int n, int* returnSize)
{
    *returnSize = 0;

    int* ans = (int*)malloc(sizeof(int) * numsSize);
    int i = 0;

    while (*returnSize < numsSize) {
        ans[(*returnSize)++] = nums[i];
        ans[(*returnSize)++] = nums[i + n];
        i++;
    }
    return ans;
}

int main()
{
    int nums[6] = { 2,5,1,3,4,7 };
    int n;
    int* ans = shuffle(nums, 6, 3, &n);

    for (int i = 0;i < n;i++) {
        printf("%d ", ans[i]);
    }
    printf("\n");

    return 0;
}