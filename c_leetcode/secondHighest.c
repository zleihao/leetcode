#include "stdio.h"
#include "stdbool.h"
#include <assert.h>

int secondHighest(char *s)
{
    int first_max = -1, second_max = -1;

    while (*s != '\0') {
        int tmp = *s - '0';
        if (tmp >= 0 && tmp <= 9) {
            if (tmp > first_max) {
                second_max = first_max;
                first_max = tmp;
            } else if (tmp < first_max && tmp > second_max) {
                second_max = tmp;
            }
        }
        s++;
    }

    return second_max;
}

int main()
{
    char *s = NULL;
    //示例1
    s = "dfa12321afd";
    assert(secondHighest(s) == 2);

    //示例2
    s = "abc1111";
    assert(secondHighest(s) == -1);
}