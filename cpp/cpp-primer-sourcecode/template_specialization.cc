#include <iostream>
using namespace std;

template<typename T>
int compare(const T &v1, const T &v2) {
    if (v1 > v2) return 1;
    if (v1 < v2) return -1;
    return 0;
}

// 特例化上面的模板
template <>
int compare(const char* const &p1, const char* const &p2) {
    cout << "specialization";
    return strcmp(p1, p2);
}

template<>
int compare(const int &v1, const int &v2) {
    cout << "int specialization";
    return v1 < v2;
}

int main() {
    const char *p1 = "hi", *p2 = "mom";
    cout << compare(p1, p2) << endl;
    compare(3, 4);
    return 0;
}