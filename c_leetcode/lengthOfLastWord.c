#include "stdio.h"
#include <assert.h>
#include "string.h"

int lengthOfLastWord(char* s)
{
    int ans = 0;
    int length = strlen(s) - 1;

    while (length >= 0) {
        if (s[length] == ' ' && ans != 0) {
            break;
        }
        if (s[length] != ' ') {
            ans++;
        }
        length--;
    }
    return ans;
}

int main()
{
    assert(lengthOfLastWord("Hello World") == 5);
    assert(lengthOfLastWord("   fly me   to   the moon  ") == 4);
    assert(lengthOfLastWord("luffy is still joyboy") == 6);

    return 0;
}