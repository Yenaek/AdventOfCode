#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int max(int a, int b) {
    if ( a > b ) 
        return a;
    return b;
}

void replaceLowest(int curr, int* maximum, int length){
    int i;
    int min = 0;
    for(i = 1; i < length; ++i) {
        if (maximum[i] < maximum[min])
            min = i;
    }
    maximum[min] = max(maximum[min], curr);
}

int readline(char* output) {
    char c;
    while(1) {
        c = getchar();
        if(c == EOF) {
            return 1;
        } 
        if(c == '\n'){
            *output = '\0';
            return 0;
        }
        *output = c;
        output++;
    }
}


int main(int argc, char* argv[]) {
    char buff[256];
    int curr = 0;
    int len = 3;
    int maximum[len];
    for(int i = 0; i < len; ++i){
        maximum[i] = 0;
    } 
    readline(buff);
    do  {
        if (strcmp(buff, "") == 0) {
            replaceLowest(curr, maximum, len);
            curr = 0;
        }
        curr += atoi(buff);
    } while (readline(buff) == 0);
    int sum = 0;
    for(int i = 0; i < len; ++i){
        printf("%d ", maximum[i]);
        sum += maximum[i];
    }
    printf("\n%d\n", sum);
    
    return 0;
}
