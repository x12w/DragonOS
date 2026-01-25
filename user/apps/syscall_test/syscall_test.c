#include <stdio.h>
#include <unistd.h>
#include <sys/syscall.h> // 包含 syscall 函数的头文件 [cite: 35]

int main() {
    // 使用 libc 的 syscall 函数直接调用 2333 号系统调用 [cite: 35]
    long ret = syscall(2333);

    // 打印返回值，预期应为 6666 [cite: 32, 34]
    printf("Syscall 2333 return value: %ld\n", ret);

    return 0;
}
