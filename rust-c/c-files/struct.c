#include <stdio.h>

struct Point {
    int x;
    int y;
};

int get_sum(struct Point *point) {
    return point->x + point->y;
}
