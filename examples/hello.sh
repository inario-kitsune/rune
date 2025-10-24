#!/bin/bash
# 示例 Shell 脚本
# 演示如何使用 Rune 运行 Shell 脚本

echo "你好，来自 Rune 的 Shell 脚本！"

if [ $# -gt 0 ]; then
    echo "收到的参数: $@"
else
    echo "没有收到参数"
fi
