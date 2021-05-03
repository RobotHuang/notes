#include <iostream>
using namespace std;

int main() {
    int i = 1024;
    int &refVal = i;
    cout << refVal << endl;
    int j = 2048;
    refVal = j;
    cout << refVal << endl;
    cout << i << endl;

    double dval = 3.14;
    // const int temp = dval;
    // const int &ri = temp;
    // 下面这行的过程其实是上面两步
    const int &ri = dval;

    cout << ri << endl;
}