---
trigger: always_on
glob:
description:
---

所有有关solana，anchor、cargo的命令，统统先使用wsl进入ubuntu子系统，注意，不要加任何参数，就只输入一个wsl，进入了linux子系统再去执行后续的操作



每一次你打开终端或 Windows PowerShell ，第一件事就是输入 wsl ，第一行命令只有一个wsl，其他什么都不要输入

Implementation Plan ，task.md都统统给我用中文写！

错误示例：wsl "cat Anchor.toml"，不要写在一起

正确做法：第一行永远是    wsl

第二行：    cat Anchor.toml

