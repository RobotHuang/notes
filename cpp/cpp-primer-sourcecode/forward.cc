#include <iostream>
using namespace std;

template <typename F, typename T1, typename T2>
void flip1(F f, T1 t1, T2 t2) {
    f(t2, t1);
}

template <typename F, typename T1, typename T2>
void flip2(F f, T1 &&t1, T2 &&t2) {
    f(t2, t1);
}

template <typename F, typename T1, typename T2>
void flip(F f, T1 &&t1, T2 &&t2) {
    f(std::forward<T2>(t2), std::forward<T1>(t1));
}

void g(int &&i, int &j) {
    cout << i << " " << j << endl;
}

void f(int v1, int &v2) {
    cout << v1 << " " << ++v2 << endl;
}

int main() {
    int i = 0;
    f(42, i);
    cout << "i=" << i << endl;
    int j = 0;
    flip1(f, j, 42); // j值不会改变，因为传进flip1的是j的拷贝
    cout << "j=" << j << endl;
    flip2(f, j, 42); // 根据规则推断出t1的类型为int&
    cout << "j=" << j << endl;
    // flip2(g, i, 42); // error: rvalue reference to type 'int' cannot bind to lvalue of type 'int'
}