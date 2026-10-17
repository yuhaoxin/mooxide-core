#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * 释放 [`mooxide_version`] 返回的字符串。
 *
 * # Safety
 * `ptr` 必须是 [`mooxide_version`] 的返回值且未被释放过。
 */
void mooxide_version_free(char *ptr);

/**
 * 返回库版本号字符串，调用方用 [`mooxide_version_free`] 释放。
 */
char *mooxide_version(void);

/**
 * 校验 C 侧能正常往返一个字符串，作为 FFI 链路的冒烟占位。
 *
 * 返回 0 表示 `input` 非空，1 表示为空或非法 UTF-8。
 *
 * # Safety
 * `input` 必须指向以 NUL 结尾的合法 C 字符串。
 */
int32_t mooxide_smoke(const char *input);
