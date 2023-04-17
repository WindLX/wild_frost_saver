# WildFrost(雪居之地)存档管理器

本项目是一个命令行的雪居之地存档管理器，它可以帮助你管理游戏的存档，可以将存档从一个路径复制到另一个路径，也可以将存档从另一个路径复制回来。

## 安装和运行

您可以通过以下步骤安装游戏存档管理器：

确保您的计算机已经安装了Rust编程语言的环境。

1. 在命令行中运行以下命令：

```bash
git clone https://github.com/your-username/game-save-manager.git
cd game-save-manager
cargo build --release
```

这将在target/release/目录下生成可执行文件。

如果您不想编译软件，您可以在仓库的发布页面下载预编译版本的软件。

## 使用方法

当程序启动时，它会提示你输入一个数字来选择要执行的操作，例如：

```
Please select a command:
1. Save files from old to new
2. Load files from new to old
q. Quit
```

你可以输入 `1` 来将旧路径中的存档文件复制到新路径中，输入 `2` 来将新路径中的存档文件复制回旧路径中，或者输入 `q` 退出程序。

## 注意事项

1. 在使用本程序之前，请确保已经备份好了存档文件，以免因为误操作导致存档文件被覆盖或删除。

2. 本程序只能管理本地计算机上的存档文件，无法管理云端或其他计算机上的存档文件。

3. 本程序仅支持 Windows 操作系统。