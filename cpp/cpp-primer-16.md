# 模板和泛型编程
## 定义模板
以关键字 `template` 开始，然后接着是模板参数列表，由一个或多个模板参数构成，用`< ... >`包起来。  
编译器可以根据参数推断出模板参数的类型来进行实例化。模板的参数其实可以和函数参数进行类比，也可以分为形参和实参。  
模板参数有两类：类型模板参数和非类型模板参数。
### 模板编译
编译器遇到模板定义的时候并不生成代码，只有实例化出模板的一个特定版本时才会生成代码。因此，编译器需要掌握模板函数或者类模板的定义，所以在头文件中要包含定义。   
模板编译错误会发生在三个阶段：
1. 编译模板本身，通常只会检测语法错误。
2. 编译器遇到模板使用时，检查参数类型是否匹配和参数数目是否正确。
3. 模板实例化的时候，检查类型相关的错误。
### 函数模板
```cpp
// typename 可以替换为 class
template <typename T>
int compare(const T &v1, const T &v2) {
    if (v1 < v2) return -1;
    if (v2 < v1) return 1;
    return 0;
}
// 使用
compare(1, 2);

// 非类型模板参数
template<unsigned N, unsigned M>
int compare(const char (&p1)[N], const char (&p2)[M]) {
    return strcmp(p1, p2);
}
compare("hi", "mom"); // 实例化compare(const char (&p1)[3], const char (&p2)[4])

// 函数模板可以声明为inline和constexpr
template <typename T> inline T min(const T&, const T&);
```
**绑定到非类型整型参数的实参必须是一个常量表达式，绑定到指针或引用非类型参数的实参必须具有静态生存周期（不能使用非静态局部变量或者动态对象）**
### 类模板
与函数模板不同，编译器不能为类模板推断模板参数类型。
#### 定义类模板
```cpp
template <typename T>
class Blob {
public:
    typedef T value_type;
    typedef typename std::vector<T>::size_type size_type; // typename告诉编译器size_type是嵌套类型
    Blob();
    Blob(std::initializer_list<T> il);
private:
    std::shared_ptr<std::vector<T>> data;
    void check(size_type i, const std::string &msg) const;
};

// 构造函数
template <typename T>
Blob<T>::Blob(std::initializer_list<T> il): data(std::make_shared<T>(il)) {}

// 类成员
template <typename T>
T& Blob<T>::back() {
    check(0, "back on empty Blob");
    return data->back();
}

// 在类模板外使用类模板名
template <typename T>
BlobPtr<T> BlobPtr<T>::operator++(int) {
    // 在类作用域里面无须指定模板实参，等价于BlobPtr<T> ret = *this;
    BlobPtr ret = *this;
    ++*this;
    return ret;
}
```
#### 实例化类模板
```cpp
Blob<int> ia;
Blob<int> ia2 = {0, 1, 2, 3, 4};
```
#### 类模板和友元
类与友元各自是模板是相互无关了。如果一个类模板包含一个非模板友元，则友元可以访问所有模板实例。如果友元自身是模板，类可以授权给所有友元模板实例，也可以只授权给特定的实例。  
```cpp
template <typename> class BlobPtr;
template <typename> class Blob;
template <typename T> bool operator==(const Blob<T>&, const Blob<T>&);
template <typename T>
class Blob {
    friend class BlobPtr<T>;
    friend bool operator==<T>(const Blob<T>&, const Blob<T>&);
}

// 前置声明，在将模板的一个特定实例声明为友元的时候要用到
template <typename T> class Pal;
class C {
    friend class Pal<C>; // Pal的特定实例才是C的友元
    template <typename T> friend class Pal2; // Pal2的所有实例都是C的友元
};
template <typename T> class C2 {
    friend class Pal<T>; // C2的每个实例将相同实例化的Pal声明为友元
    friend class Pal3; // Pal3是非模板类，它是C2所有实例的友元
}
```
在新标准中，我们可以将模板类型参数声明为友元
```cpp
template <typename Type> class Bar {
    friend Type;
};
```
类型为 Foo，Foo 将成为 Bar<Foo> 的友元。
#### 模板类型别名
```cpp
typedef Blob<string> StrBlob;
template<typename T> using twin = pair<T, T>; // 新标准
```
#### 类模板的 static 成员
每个类模板的实例都有一个独有的 static 对象
```cpp
template <typename T> class Foo {
public:
    static std::size_t count() {return ctr;}
private:
    static std::size_t ctr;
};

Foo<int> fi, fi2, fi3; // 三个对象共享相同的Foo<int>::ctr
Foo<string> fs; // fs和fi的ctr不一样，一个是Foo<string>::ctr，另外一个是Foo<int>::ctr
```
### 模板参数
#### 模板参数与作用域
一个模板参数名的可用范围是在其声明之后，至模板声明或定义结束之前。模板参数会隐藏外层作用域中声明的相同名字。’
```cpp
typedef double A;
template <typename A, typename B> void f(A a, B b) {
    A tmp = a;
    // double B; // error
}
```
#### 模板声明
模板声明必须带模板参数，声明中模板参数的名字不必与定义中相同。
#### 使用类的类型成员
我们想使用模板类型参数的类型成员就要加上 typename 关键字。
```cpp
template <typename T>
typename T::value_type top(const T& c) {
    if (!c.empty()) {
        return c.back();
    } else {
        return typename T::value_type();
    }
}

// T 有可能是下面这样
class Foo {
    typedef int value_type;
}
```
#### 默认模板参数
```cpp
template <typename T, typename F = less<T>>
int compare(const T &v1, const T &v2, F f = F()) {
    if (f(v1, v2)) return -1;
    if (f(v2, v1)) return 1;
    return 0;
}
```
#### 模板默认实参与类模板
```cpp
template <typename T = int>
class Numbers {
public:
    Numbers(T v = 0): val(v) {}
private:
    T val;
}

Numbers<long double> lots_of_precision;
Numbers<> average_precision;
```
### 成员模板
#### 普通类的成员模板
```cpp
class DebugDelete {
public:
    DebugDelete(std::ostream &s = std::cerr): os(s) {}
    template <typename T> void operator()(T *p) {
        os << "deleting unique_ptr" << std::endl;
        delete p;
    }
private:
    std::ostream &os;
}

unique_ptr<int, DebugDelete> p(new int, DebugDelete());
```
#### 类模板的成员模板
```cpp
template <typename T> class Blob {
    template <typename It> Blob(It b, It e);
    //...
}

template<typename T>
template<typename It>
Blob<T>::Blob(It a, It b): data(std::make_shared<std::vector<T>>) {}
```
### 控制实例化
显示实例化可以避免在多个文件中实例化相同模板的额外开销。
```cpp
extern template class Blob<string> //声明，注意声明一定要在使用之前
Blob<string> sa1, sa2; // 不会实例化Blob<string>
```
在其他文件中要有定义
```cpp
template class Blob<string>
```

实例化定义会实例化类的所有成员

### 效率与灵活性
shared_ptr 和 unique_ptr 删除器绑定的例子，一个是编译时，一个是运行时

## 模板实参推断
### 类型转换与模板类型参数
编译器通常不对模板实参进行类型转换，而是生成一个新的模板参数。除了以下的几种情况除外：
1. const 转换：可以将一个非 const 对象的引用（或指针）传递给一个 const 的引用（或指针）
2. 数组或函数指针转换：如果函数形参不是引用类型，则可以对数组或函数类型的实参应用正常的指针转换

```cpp
template <typename T> T fobj(T, T);
template <typename T> T fref(const T&, const T&);
string s1("a value");
const string s2("another value");
fobj(s1, s2); // fobj(string, string)
fref(s1, s2); // fref(const string&, const string&)

int a[10], b[42];
fobj(a, b); // fobj(int*, int*)
fref(a, b); // error, 类型不匹配
```

使用相同模板参数类型的函数形参类型必须相同
```cpp
template <typename T> compare(const T&, const T&);
compare(1024, 1024); // 通过
long lng;
compare(lng, 1024); // 不通过，不能实例化compare(long, int)

template <typename A, typename B> compare(A, B);
compare(lng, 1024); // 通过
```

函数模板可以存在普通类型
```cpp
template <typename T> ostream &print(ostream &os, const T &obj) {
    return os << obj;
}
print(cout, 42);
ofstream f("output");
print(f, 10); // f强制转换为ostream
```
### 函数模板显示实参
如果编译器无法自动推导出模板实参的类型，我们需要显示指出实参的类型。
```cpp
template <typename T1, typename T2, typename T3>
T1 sum(T2, T3); // T1显然无法自动推导出来

auto val3 = sum<long long>(i, lng);
```
显示模板实参由左至右的顺序与对应的模板实参匹配
```cpp
// 糟糕设计
template <typename T1, typename T2, typename T3>
T3 alternative_sum(T2, T1);
auto val3 = alternative_sum<long long, int, long>(i, lng); // 必须三个参数都指定
```
正常的类型转换可以应用于显示指定的实参，不会重新实例化
```cpp
long lng;
compare<long>(lng, 1024);
compare<int>(lng, 1024);
```
### 尾置返回类型与类型转换
当我们不知道返回结果的准确类型，但知道所需类型是处理的序列的元素类型，比如：
```cpp
template <typename It>
auto fcn(It beg, It end) -> decltype(*beg) {
    return *beg;
}

vector<int> vi = {1, 2, 3, 4, 5};
auto &i = fcn(vi.begin(), vi.end()); // fcn返回int&
```
`*beg` 返回的是一个左值，`decltype(*beg)` 推断出的类型是元素的引用，比如 int 序列，推断出的就是 int&。
#### 进行类型转换的标准库模板类
以上面的例子来说，如果我们不想返回`&int`，而是`int`，我们可以使用标准库的类型转换模板，这些模板定义在头文件 `type_traits` 里面。
```cpp
template <typename It>
auto fcn2(It beg, It end) -> typename remove_reference<decltype(*beg)>::type {
    return *beg;
}
```

标准类型转换表
|Mod\<T>的Mod为|若T为|Mod\<T>::type为|
|--|--|--|
|remove_reference|X&或X&&<br>否则|X<br>T|
|add_const|X&，const X 或函数<br>否则|T<br>const T|
|add_lvalue_reference|X&<br>X&&<br>否则|T<br>X&<br>T&|
|add_rvalue_reference|X&或X&&<br>否则|T<br>T&&|
|remove_pointer|X*<br>否则|X<br>T|
|add_pointer|X&或X&&<br>否则|X*<br>T*|
|make_signed|unsigned X<br>否则|X<br>T|
|make_unsigned|带符号类型<br>否则|unsigned X<br>T|
|remove_extent|X[n]<br>否则|X<br>T|
|remove_all_extents|X[n1][n2]...<br>否则|X<br>T|
每个模板都有一个名为type的public成员，表示一个类型。
### 函数指针和实参推断
```cpp
template <typename T> int compare(const T&, const T&);
int (*pf1)(const int&, const int&) = compare;

void func(int(*) (const string&, const string&));
void func(int(*) (const int&, const int&));
func(compare); // 错误，不知道使用哪个comapre实例
func(compare<int>);
```
### 模板实参推断和引用
```cpp
template <typename T> void f(T &p);
```
**从左值引用函数参数推断类型**：
1. 当一个函数参数是模板类型参数的一个普通左值引用时候后，只能传递给它一个左值
```cpp
template <typename T> void f1(T&);
f1(i);
f1(5); //错误，5是右值
```
2. 如果一个函数的参数类型是常量引用（const T&），可以传递给它任何类型的实参，const是函数形参的一部分而不是模板参数类型的一部分。
```cpp
template <typename T> void f2(const T&);
f2(i);
f2(5); // 正确，可以传递右值
```
**从右值引用函数参数推断类型**：类型推断过程类似普通左值引用函数参数的推断过程
```cpp
template <typename T> void f3(T&&);
f3(42);
```
**引用折叠和右值引用参数**  
在模板推断中，C++ 定义两个规则允许将右值引用绑定到一个左值上，比如，i是int, 调用`f3(i)`（f3是上面那块代码中的f3）  
1. 当我们将一个左值传递给函数的右值引用参数，且此右值引用指向模板类型参数时，编译器推断模板类型参数为实参的左值引用。因此，当我们调用`f3(i)`，编译器推断 T 的类型为`int&`。
2. 如果我们间接创建一个引用的引用，则这些引用进行了折叠，引用会折叠成一个普通的引用。X& &, X& &&, X&& &都折叠成了 X&。

**编写接受右值引用参数的模板函数** 
编译器可以自动推导出引用类型，且如果函数的参数是一个右值引用，就会导致一些难以预料的错误
```cpp
template <typename T> void f3(T&& val) {
    T t = val;
    t = fcn(t);
    if (val == t) {}
}

int i;
f3(i); // T为int&
f3(42); // T为int
```
可以通过重载去避免
```cpp
template <typename T> void f(T&&);
template <typename T> void f(const T&);
```

### 理解 std::move
**std::move是如何定义的**  
```cpp
template <typename T>
typename remove_reference<T>::type&& mover(T&& t) {
    return static_cast<typename remove_reference<T>::type&&>(t);
}
```
**std::move是如何工作的**  
```cpp
string s1("hi!");
std::move(s1);
```
传递给 move 的实参是一个左值
* 推断出 T 的类型为 string&
* 因此 remove_reference 用 string& 进行实例化
* remove_reference(string&)的type成员是string
* move 的返回类型是 string&&
* move 的函数参数 t 实例化为 string& &&，会折叠为 string&  

因此，这个调用实例化`move<string&>`，即 `string&& move(string &t)`  
**从一个左值 static_cast 到一个右值引是允许的**
不能隐式的将一个左值转换为右值引用，但是可以用 static_cast 显示地将一个左值转换为一个右值引用
### 转发
[source code](cpp-primer-sourcecode/forward.cc)  
**定义能保持类型信息的函数参数**  
通过引用折叠来实现
```cpp
template <typename F, typename T1, typename T2>
void flip2(F f, T1 &&t1, T2 &&t2) {
    f(t2, t1);
}
```
当传递给 `flip2()` 的函数为以下形式将会出错：
```cpp
void g(int &&i, int &j) {
    cout << i << " " << j << endl;
}
```
`t2` 会以左值的形式传递给 `g` 的参数 `i`。除非是模板函数，否则不允许将右值引用绑定到左值，所以会报错。  
**使用 std::forward 保持类型信息**  
```cpp
template <typename F, typename T1, typename T2>
void flip(F f, T1 &&t1, T2 &&t2) {
    f(std::forward<T2>(t2), std::forward<T1>(t1));
}
```
## 重载与模板
匹配优先级规则：
* 如果同样好的函数中只有一个是非模板函数，则选择此函数
* 如果不存在非模板，选择更特例化的模板
* 否则，此调用有歧义

```cpp
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
```
## 可变参数模板
可变数目的参数被称为参数包，分为模板参数包和函数参数包
```cpp
template <typename T, typename... Args>
void foo(const T &t, const Args&... rest);

int i = 0;
double d = 3.14;
string s = "how now brown cow";
foo(i, s, 42, d);
foo(s, 42, "hi");
foo(d, s);
foo("hi");
```
**sizeof...运算符**
用于计算参数包中参数的个数
```cpp
template<typename ... Args> void g(Args ... args) {
    cout << sizeof...(Args) << endl;
    cout << sizeof...(args) << endl;
}
```
### 编写可变参数模板
```cpp
template <typename T>
ostream &print(ostream &os, const T &t) {
    return os << t;
}
template <typename T, typename... Args>
ostream &print(ostream &os, const T &t, const Args&... rest) {
    os << t << ", ";
    return print(os, rest...); // 递归调用，打印其他实参
}
```
### 包扩展
扩展包的时候要提供用于每个扩展元素的模式（比如一个函数），在模式右边放一个省略号来触发扩展操作。
```cpp
template <typename T, typename... Args>
ostream &print(ostream &os, const T &t, const Args&... rest) {
    os << t << ", ";
    return print(os, rest...); // 递归调用，打印其他实参
}
```
`print(os, rest...);`就是包扩展  
使用一个函数作为一个模式
```cpp
template <typename... Args>
ostream &errorMsg(ostream &os, const Args&... rest) {
    return print(os, debug_rep(rest)...);
}
```
相当于
```cpp
print(os, debug_rep(a1), debug_rep(a2));
```
### 转发参数包
`std::forward(Args)(args)...`
## 模板特例化
**定义模板特例化**
```cpp
template<typename T>
int compare(const T&, const T&);
// 特例化上面的模板
template <>
int compare(const char* const &p1, const char* const &p2) {
    return strcmp(p1, p2);
}
```
**特例化本质上是模板的一个实例**

**类模板特例化**
```cpp
namespace std {
template<>
struct hash<Sales_data> {

}
}
```
**类模板部分特例化**  
类模板特例化不必为所有模板参数提供实参，类模板的部分特例化本身是一个模板
```cpp
template <class T> struct remove_reference<T&> {
    typedef T type;
}
```
**特例化成员**
```cpp
template<typename T> struct Foo {
    Foo(const T &t = T()):mem(t){}
    void Bar(){}
    T mem;
}
template<>
void Foo<int>::Bar() {

}

Foo<int> fi;
fi.Bar(); // 调用特例化的Bar()
```