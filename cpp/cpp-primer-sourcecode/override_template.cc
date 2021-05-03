#include <iostream>
#include <sstream>
using namespace std;

template <typename T> string debug_rep(const T &t) {
    cout << "const T &t" << endl;
    ostringstream ret;
    ret << t;
    return ret.str();
}

template <typename T> string debug_rep(T *p) {
    cout << "T *p" << endl;
    ostringstream ret;
    ret << "pointer: " << p;
    if (p) {
        ret << " " << debug_rep(*p);
    } else {
        ret << "null pointer";
    }
    return ret.str();
}

string debug_rep(const string &s) {
    cout << "const string &s" << endl;
    return '"' + s + '"';
}

int main() {
    string s("hi");
    cout << debug_rep(s) << endl; // 调用debug_rep(const T &t) 有非模板会调用非模板
    cout << debug_rep(&s) << endl; // 调用debug_rep(T *p)
    const string *sp = &s;
    cout << debug_rep(sp) << endl; // debug_rep(T *p)
    return 0;
}