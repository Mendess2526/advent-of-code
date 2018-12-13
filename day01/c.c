#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>

int main() {
    char c;
    int freq = 0;
    char line[10];
    int i = 0;
    int f = open("input", O_RDONLY);
    while(read(f, &c, 1)){
        if(c == '\n') {
            freq += atoi(line);
            i = 0;
        } else {
            line[i++] = c;
            line[i] = '\0';
        }
    }
    printf("%d\n", freq);
    return 0;
}
