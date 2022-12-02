#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include <assert.h>

int *minOperations(char *boxes, int *returnSize)
{
    int left = (boxes[0] == '1' ? 1 : 0);
    int right = 0, step = 0;
    *returnSize = strlen(boxes);

    int *ans = (int *)malloc(sizeof(int) * (*returnSize));
    memset(ans, 0, sizeof(int) * (*returnSize));

    //得到第一个盒子需要的步数
    for (int i = 1; i < *returnSize; i++) {
        if (boxes[i] == '1') {
            ans[0] += i;
            right++;
        }
    }

    for (int i = 1; i < *returnSize; i++) {
        ans[i] = ans[i - 1] + left - right;
        if (boxes[i] == '1') {
            left++;
            right--;
        }
    }
    return ans;
}

int main()
{
    char *boxes = NULL;
    int len = 0;
    int *num = NULL;

    //示例1
    boxes = "110";
    num = minOperations(boxes, &len);
    printf("示例1: ");
    for (int i = 0; i < len; i++) {
        printf("%d ", num[i]);
    }
    printf("\n");

    //示例2
    boxes = "001011";
    num = minOperations(boxes, &len);
    printf("示例2: ");
    for (int i = 0; i < len; i++) {
        printf("%d ", num[i]);
    }
    printf("\n");

    return 0;
}