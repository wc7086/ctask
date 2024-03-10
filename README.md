# 一个简单的签到任务列表

使用以下命令安装

```
crago install ctask
```

或

```
cargo install --git https://github.com/wc7086/ctask
```

安装后第一次启动会在用户目录释放配置文件 `ctaskconfig.json`


配置示例

```json5
{
    "job": {
        // 要签到的账号总数，从 001 开始。
        // 输入 10 则会生成十个签到任务
        "total_account": 1,
        "tasks": {
            // 前面是签到任务名称，后面是时间间隔，以天为单位
            "任务0": 1,
            "任务1": 5
        }
    },
    "task_list": {
        "001": { // 账号名称
            "任务0": { // 任务名称
              "start_time": "2024-03-10" // 开始时间
            },
            "任务1": {
              "start_time": "2024-03-13"
            }
          }
    }
}
```
