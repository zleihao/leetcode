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

    //�õ���һ��������Ҫ�Ĳ���
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

    //ʾ��1
    boxes = "110";
    num = minOperations(boxes, &len);
    printf("ʾ��1: ");
    for (int i = 0; i < len; i++) {
        printf("%d ", num[i]);
    }
    printf("\n");

    //ʾ��2
    boxes = "001011";
    num = minOperations(boxes, &len);
    printf("ʾ��2: ");
    for (int i = 0; i < len; i++) {
        printf("%d ", num[i]);
    }
    printf("\n");

    return 0;
}