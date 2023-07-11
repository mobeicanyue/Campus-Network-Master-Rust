# Login to CQNU campus network in `Rust`
[![Release](https://github.com/mobeicanyue/Campus-Network-Master-Rust/actions/workflows/release.yml/badge.svg)](https://github.com/mobeicanyue/Campus-Network-Master-Rust/actions/workflows/release.yml)

* 用Rust 实现的Campus Network Master for CQNU 校园网登陆
* 程序更小，更快，更安全
* 支持arm64平台(测试中，不保证可用性)  

>**用这个登录会不会泄密？**  
>不会，你的账号密码都是在本地，不会上传到任何地方，你可以查看源码，或者使用Wireshark抓包查看网络请求。

## **使用方法**
1. 下载适合你操作系统的可执行文件
2. 解压可执行文件
3. 重命名你的可执行文件为 `学号;密码` 格式  
    如 `2020051XXXXXX;123456.exe`（Windows） 或者 `2020051XXXXXX;123456`（Linux 或 macos）
4. 运行可执行文件  
    Windows 双击运行;  
    Linux 或 macos 打开终端，输入 `./2020051XXXXXX;123456` 回车
### 运行截图
![image](https://github.com/mobeicanyue/Campus-Network-Master-Rust/assets/81098819/d2b85c04-bb41-4b9b-90f4-1287c0cc461f)
## **设置开机自启（可选）**
### **Windows**
1. 输入 `Win + R` 打开运行窗口
2. 输入 `shell:startup` 打开启动文件夹
3. 将可执行文件复制到启动文件夹
4. 这样就可以开机自启了

### **Linux**
- 配置rc.local文件或者图形化界面添加即可
- 你已经是一个Linux大佬了，自己想办法吧 :)

### **MacOS**
1. 打开系统设置
2. 点击用户与群组
3. 点击登录项
4. 点击左下角的加号
5. 选择可执行文件
6. 这样就可以开机自启了

More detials in https://github.com/mobeicanyue/Campus-Network-Master