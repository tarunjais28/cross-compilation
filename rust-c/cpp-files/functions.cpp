#include <iostream>

class Points {
public:
    int x;
    int y;

    Points(int x, int y) : x(x), y(y) {}

    void display() const {
        std::cout << "Point(" << x << ", " << y << ")" << std::endl;
    }
};

extern "C" {
    Points* create_point(int x, int y) {
        return new Points(x, y);
    }

    void display_point(Points* p) {
        p->display();
    }

    void destroy_point(Points* p) {
        delete p;
    }
}
