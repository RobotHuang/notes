# C++ Primer

## 第1章 开始
### 编译、运行
* 程序源文件命名约定：.cc, .cxx, .cpp, .cp, .c
* 编译：`$ cc prog1.cc`
* 运行：编译完后会生成 .out 结尾的文件（在UNIX下）或者 .exe 结尾的文件（在 Windows下）  

### 输入输出
C++ 包含了一个标准库来提供IO机制，即 iostream 库。 iostream 包含 ostream （输出流）和 istream （输入流）两个基础类型。流是字符序列。  
  
4个标准IO对象：
1. cin
2. cout
3. cerr
4. clog

使用这个必须包含头文件：`#include<iostream>`，并且他们是在 std 命名空间底下的。  
`std::endl` 的效果是结束当前行，并将缓冲区中的内容刷到设备中

### 注释
`\\` 和 `/*  */`  
编译器预处理的时候会将注释去掉
```cpp
std::cout << "/*";
std::cout << "/*";
// 下面这行有错误
// std::cout << /* "*/" */;
```
### 循环语句
`for` 和 `while`
#### 读取数量不定的输入数据
```cpp
#include <iostream>
int main() {
    int sum = 0, value = 0;
    while (std::cin>>value) {
        sum += value;
    }
    std::cout<<sum<<std::endl;
    return 0;
}
```
### if
### class
类定义了一个类型和其关联的一组操作

## 第二章 变量和基本类型
### 基本内置类型
基本内置类型包括**算术类型**和**空类型**  
算术类型：整形（包括字符和布尔类型）和浮点型  
C++标准给了算术类型尺寸的最小值
|类型|含义|最小尺寸|
|----|----|----|
|bool|布尔类型|未定义|
|char|字符|8位|
|wchar_t|宽字符|16位|
|char16_t|Unicode字符|16位|
|char32_t|Unicode字符|32位|
|short|短整型|16位|
|int|整型|16位|
|long|长整型|32位|
|long long|长整型|64位|
|float|单精度浮点数|6位有效数字|
|double|双精度浮点数|10位有效数字|
|long double|扩展精度浮点数|10位有效数字|

通过添加 unsigned ，将无符号数变成有符号数  
字符型被分为了三种：char, signed char, unsigned char。但是字符型只有两种表现形式：带符号和无符号的，这取决于编译器。  

> 如何选择类型  
>* 已知数据为无符号，选择无符号类型
>* 整数运算使用 int，数据较大的时候选择 long long
>* 算术运算不要使用 char 和 bool
>* 执行浮点数运算优先选 double

#### 类型转换
自动类型转换是十分危险的，要注意隐式的类型转换，尤其是含有无符号表达式。
```cpp
unsigned u = 10;
int i = -42;
std::cout << u + i << std::endl; // 结果不正确，整数转为无符号整数
// 循环不会结束
for (unsigned u = 10; u >= 0; --u) {
    std::cout << u << std::endl;
}
```
#### 字面值常量
字面值常量：424， "abcdef"
##### 整型和浮点型字面值
```cpp
20 /* 十进制 */
024 /* 八进制 */
0x14 /* 十六进制 */
3.1415
2.1415E0
0.0
0e0
.001
```
##### 字符和字符串
```cpp
'a'
"Hello world"
```
##### 转义序列
\n, \v, \t, \a...
##### 指定字面值的类型
```cpp
L'a' // 宽字符
u8"hi!" // utf8
42ULL // unsigned long long
1E-3F // float
3.1415L // long double
```
### 变量
#### 变量定义
类型说明符 变量名
##### 初始值
当变量创建的时候获得一个特定的值，则这个变量被初始化了。需要注意的是，**初始化不是赋值**。
##### 列表初始化
这是 C++ 11 新增加的特性。
```cpp
int units_sold{0};
``` 
使用列表初始化且初始值存在丢失信息的风险，则编译器将报错。
##### 默认初始化
函数体外的变量根据变量类型初始化，函数体内的变量不初始化，所以需要手动初始化。
##### 变量声明和定义的关系
C++ 使用的是分离式编译，使得每个文件可以单独的进行编译。  
* 声明：规定了变量的名字和类型
* 定义：申请了存储空间，也可能为变量赋值
```cpp
extern int i // 声明i而非定义i
int j; // 声明并且定义j
extern int j = 1; //声明且定义，任何包含初始化的声明是定义
```
#### 标识符
由下划线，字母，数字组成，不以数字开头。
#### 作用域
作用域以花括号分隔
### 复合类型
复合类型是指基于其他类型定义的类型，比如：引用和指针。复合类型声明包括基本数据类型加上类型修饰符。  
建议将 * 和 & 与变量名写在一起。
#### 引用
引用为对象起了另外一个名字
```cpp
int ival = 1024;
int &refVal = ival; // refVal指向ival，引用必须初始化
```
引用和值绑定在一起，不是复制。不能直接引用字面量。引用绑定对象后不能绑定到另外的对象。[例子代码](cpp-primer-sourcecode/reference.cc)
#### 指针
指针指向另外一种类型。指针与引用不同，引用是类型别名而指针本身是一个对象，指针无须在定义时候赋值。指针存放的是对象的地址
```cpp
int ival = 42;
int *p = &ival; // 取地址
int pval = *p; // 解引用
int *voidptr = nullptr // 不要使用 NULL，NULL的值是0
```
#### 指向指针的指针、指向指针的引用
指向指针的指针
```cpp
int ival = 1024;
int *pi = &ival;
int **ppi = &pi;
```
指向指针的引用
```cpp
int i = 42;
int *p;
int *&r = p; // r是对一个指针的引用

r = &i // 相当于给p赋值
```
### const 限定符
默认情况下 const 只在当前文件有效。如果要在多个文件之间共享，就在声明和定义都加上 extern 关键字
```cpp
// file_1.cc 定义
extern const int bufSize = fcn();
// file_1.h 共享
extern const int bufSize;
```
#### const 引用
```cpp
const int ci = 1024;
const int &r1 = ci; // r1 不能修改，不能用非常量引用去引用常量对象
```
## 模板和泛型编程
### 定义模板
以关键字 `template` 开始，然后接着是模板参数列表，由一个或多个模板参数构成，用`< ... >`包起来。  
编译器可以根据参数推断出模板参数的类型来进行实例化。模板的参数其实可以和函数参数进行类比，也可以分为形参和实参。  
模板参数有两类：类型模板参数和非类型模板参数。
#### 模板编译
编译器遇到模板定义的时候并不生成代码，只有实例化出模板的一个特定版本时才会生成代码。因此，编译器需要掌握模板函数或者类模板的定义，所以在头文件中要包含定义。   
模板编译错误会发生在三个阶段：
1. 编译模板本身，通常只会检测语法错误。
2. 编译器遇到模板使用时，检查参数类型是否匹配和参数数目是否正确。
3. 模板实例化的时候，检查类型相关的错误。
#### 函数模板
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
#### 类模板
与函数模板不同，编译器不能为类模板推断模板参数类型。
##### 定义类模板
```cpp

```