# LiteSVM TypeScript Tutorial

这是一个使用 TypeScript 和 `litesvm` JS/TS 绑定进行 Solana 程序测试的示例项目。

## 目录结构

*   `package.json`: 项目依赖 (`litesvm`, `@solana/web3.js` 等)
*   `tsconfig.json`: TypeScript 编译配置
*   `tests/litesvm.test.ts`: 包含 Mocha 测试用例
    *   基础系统账户操作
    *   自定义账户数据读写
    *   SPL Token 账户模拟
    *   转账交易执行
    *   Sysvar (Clock) 时间控制

## 如何运行

确保你已经安装了 Node.js 和 npm。

1.  **安装依赖**

    ```bash
    npm install
    ```

2.  **运行测试**

    ```bash
    npm test
    ```

这将使用 `ts-node` 直接运行测试文件。
