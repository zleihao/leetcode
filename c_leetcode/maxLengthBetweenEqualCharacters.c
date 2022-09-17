#include "stdio.h"
#include <assert.h>
#include "string.h"

#define MAX(x,y) (x > y ? x : y)
int maxLengthBetweenEqualCharacters(char* s)
{
    int ans = -1;
    int map[26];
    int length = strlen(s);

    memset(map, -1, sizeof(map));
    for (int i = 0; i < length; i++) {
        if (map[s[i] - 'a'] < 0) {
            map[s[i] - 'a'] = i;
        } else {
            ans = MAX(ans, i - map[s[i] - 'a'] - 1);
        }
    }
    return ans;
}

int main()
{
    assert(maxLengthBetweenEqualCharacters("aa") == 0);
    assert(maxLengthBetweenEqualCharacters("abca") == 2);
    assert(maxLengthBetweenEqualCharacters("cbzxy") == -1);
    assert(maxLengthBetweenEqualCharacters("cabbac") == 4);
    return 0;
}