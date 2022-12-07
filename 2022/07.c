#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct file {
    char* name;
    int size;
} file;

typedef struct directory {
    char* name;
    struct directory* parent;
    struct directory** subdirectories;
    int subdirectory_count;
    int file_count;
    int size;
    file** files;
} dir;

dir* mkdir(const char* name, dir* parent) {
    dir* new = (dir*) malloc(sizeof(dir));
    new->files = (file**) malloc(128 * sizeof(file*));
    new->subdirectories = (dir**) malloc(128 * sizeof(dir*));
    new->parent = parent;
    new->file_count = 0;
    new->subdirectory_count = 0;
    new->name = strdup(name);
    new->size = 0;
    return new;
}

file* touch(const char* name, int size){
    file* new = (file*) malloc(sizeof(file));
    new->name = strdup(name);
    new->size = size;
    return new;
}

dir* up(dir* curr) {
    return curr->parent;
}

dir* cdroot(dir* curr){
    if(curr->parent == NULL) 
        return curr;
    return cdroot(curr->parent);
}

dir* cd(char*name, dir* curr) {
    for(int i = 0; i < curr->subdirectory_count; ++i) {
        if (strcmp(curr->subdirectories[i]->name, name) == 0) {
            return curr->subdirectories[i];
        }
    }
    return NULL;
}

dir* decipherCommand(dir* curr, char* command) {
    if( strcmp(command, "$ cd /") == 0){
        return cdroot(curr);
    }
    if( strcmp(command, "$ cd ..") == 0 ){
        return up(curr);
    }
    if( strcmp(command, "$ ls") == 0 ){
        return curr;
    }
    return cd(command + 5, curr);
}

void split(const char* str, char* first, char* second, char c) {
    int i;
    for(i = 0;str[i] != c; ++i);
    strncpy(first, str, i);
    strcpy(second, str+i+1);
}

void addSize(dir* curr, int size) {
    if (curr == NULL)
        return;
    curr->size += size;
    addSize(up(curr), size);
}

void addToCurr(dir* curr, char* buff) {
    char* first = (char*) malloc(64 * sizeof(char));
    char* second = (char*) malloc(64 * sizeof(char));
    split(buff, first, second, ' ');
    if (strcmp(first, "dir") == 0){
        dir* new = mkdir(second, curr);
        curr->subdirectories[curr->subdirectory_count++] = new;
        return;
    }
    file* new = touch(second, atoi(first));
    curr->files[curr->file_count++] = new;
    addSize(curr, new->size);
}

void sumSizes(int* total, dir* curr){
    if (curr->size <= 100000)
        *total += curr->size;
    for(int i = 0; i < curr->subdirectory_count; ++i) {
        sumSizes(total,curr->subdirectories[i]);
    }
}

void findSmallestPossible(int lowest, int* min, dir* curr) {
    if(curr->size < lowest)
        return;
    if (curr->size <= *min){
        *min = curr->size;
    }
    for(int i = 0; i < curr->subdirectory_count; ++i) {
        findSmallestPossible(lowest ,min, curr->subdirectories[i]);
    }

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
    dir* root = mkdir("/", NULL);
    dir* curr = root;
    //building the filesystem
    int status = readline(buff);
    while(status == 0) {
        if(buff[0] == '$'){
            curr = decipherCommand(curr, buff);
        } else {
            addToCurr(curr, buff);
        }
        status = readline(buff);
    }
    int* total = (int*) malloc(sizeof(int));
    sumSizes(total, root);
    printf("%d\n", *total);
    *total = root->size;
    findSmallestPossible(30000000 - (70000000 - root->size), total, root);
    printf("%d\n", *total);
    return 0;
}
