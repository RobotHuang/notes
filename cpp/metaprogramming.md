# Pritical C++ Metaprogramming

## Introduction
### What's metaprogramming
通过代码生成代码，比如：C 语言的宏，Bash 脚本生成 C 文件的 boilerplate files  
### Begin Metaprogramming with `static_assert`
```cpp
#include <iostream>

int main() {
    static_assert(sizeof(void *) == 8, "expected 64-bit platform");

//#define IS_MY_PLATFORM_64
//#ifdef IS_MY_PLATFORM_64
//    static const std::uint64_t default_buffer_size = 100 * 1024 *1024;
//#else
//    static const std::uint64_t default_buffer_size = 1024 * 1024 * 1024;
//#endif
    // 与上面的代码等价
    static const std::uint64_t default_buffer_size =
            std::conditional<sizeof(void*) == 8,
                std::integral_constant<std::uint64_t, 100 * 1024 *1024>,
                std::integral_constant<std::uint64_t, 1024 * 1024 * 1024>
                >::type::value;

    return 0;
}
```

## C++ Metaprogramming in Practice
