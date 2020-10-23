# xlog

需要[sqlite](https://www.sqlite.org)


need [sqlite](https://www.sqlite.org)


## vs code 配置调试
[参考](http://llever.com/2019/08/30/%E5%A6%82%E4%BD%95%E7%94%A8vscode%E8%B0%83%E8%AF%95rust%E8%AF%91/)

### 安装扩展
安装[Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
安装[codelldb](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

### 配置
点击`运行` -> `打开/添加配置`会打开`.vscode/launch.json`这个文件
在`configurations`中添加
```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug executable 'xlog'",
    "cargo": {
        "args": [
            "build",
            "--bin=xlog",
            "--package=xlog"
        ],
        "filter": {
            "name": "xlog",
            "kind": "bin"
        }
    },
    "args": [
        "input_str",
        "-t type"
    ],
    "cwd": "${workspaceFolder}"
}
```
在`args`里面配置需要的参数(`cargo.args`这个是用来构建项目的参数,下面这个`args`是运行程序时添加的参数)
