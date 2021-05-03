#include <iostream>

int main() {
    using std::cout;
    using std::endl;
    unsigned u = 10, u2 = 42;
    cout << u2 - u << endl;
    cout << u - u2 << endl; //64位 4294967264
    return 0;
}