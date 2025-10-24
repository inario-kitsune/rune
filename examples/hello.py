#!/usr/bin/env python3
"""
示例 Python 脚本
演示如何使用 Rune 运行 Python 脚本
"""

import sys

def main():
    print("你好，来自 Rune 的 Python 脚本！")

    if len(sys.argv) > 1:
        print(f"收到的参数: {sys.argv[1:]}")
    else:
        print("没有收到参数")

if __name__ == "__main__":
    main()
