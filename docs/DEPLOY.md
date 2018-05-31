# 部署指南

本篇 wiki 会教你如何在 __类 Unix__ 操作系统上部署 _ServerR_

wiki 提供的命令等发行分离的资源只针对 __FreeBSD 和 Debian Linux__

## 获得二进制

你可以使用 `cargo build` 自行编译或直接在 [这里](https://github.com/geekapk-r/ServerR/releases) 获取预编译程序

## 数据库配置

你需要获得数据库的管理员权限，然后创建一个 __GeekApk 数据库用户__

```bash
su postgresql
psql
```

```sql
CREATE USER geekapk WITH PASSWORD 'dolphins';
CREATE DATABASE geekapk;
GRANT ALL PRIVILEGES ON DATABASE geekapk TO geekapk;
```

这样，你的 `DATABASE_URL` 就是 `postgres://geekapk:dolphins@localhost/geekapk`

## 环境变量

GeekApk 提供了两种环境变量，Diesel 和 Rocket 又各占两种

+ `DOGETOK` 管理权限密码
+ `WEBHOOKS` [WebHooks 配置](https://github.com/geekapk-r/ServerR/wiki/API-v1-%E5%9F%BA%E6%9C%AC%E5%AE%9A%E4%B9%89#webhooks)

+ `ROCKET_ENV` 一般为 `test` 或 `prod`
+ `DATABASE_URL` 你的数据库连接配置
+ `VERBOSE` 如果有这个变量，开启调试模式

## Rocket 配置

[看这里](https://rocket.rs/guide/configuration/#environment) 创建 `Rocket.toml`

## Systemd daemonize 部署

```ini
[Unit]
Description=GeekApk API Service
After=network.target

[Service]
ExecStart=${HOME}/.cargo/bin/geekapkd
Restart=always
RestartSec=0
Environment=ROCKET_ENV=production
```

`journalctl` 会记录程序的日志输出

也可以选择每天重启：

```bash
sudo echo "24 * * * * root systemctl restart fort.service" >> /etc/cron.d/0daily
sudo echo "24 * * * * root run-parts /etc/cron.daily" >> /etc/cron.d/0daily
```

## Supervisor daemonzie 部署

```ini
[program:cat]
command=/root/.cargo/bin/geekapkd
process_name=GeekApk Server
numprocs=1
directory=/tmp
umask=022
priority=999
autostart=true
autorestart=unexpected
startsecs=10
startretries=3
exitcodes=0
stopsignal=TERM
stopwaitsecs=10
stopasgroup=false
killasgroup=false
user=root
redirect_stderr=false
stdout_logfile=/var/log/geekapk.log
stdout_logfile_maxbytes=9MB
stdout_logfile_backups=10
stdout_capture_maxbytes=4MB
stdout_events_enabled=false
stderr_logfile=/var/log/geekapk-err.log
stderr_logfile_maxbytes=5MB
stderr_logfile_backups=10
stderr_capture_maxbytes=1MB
stderr_events_enabled=false
environment=ROCKET_ENV=prod
serverurl=AUTO
```

## HTTPS 代理

建议使用 [CloudFlare](https://cloudflare.com)，不然也可以用 [Apache/Nginx proxy](https://github.com/geekapk-r/GeekApkR/blob/master/proxy_apache.conf)

## Benchmark

可以使用 __siege__ 或 __ab__ 或 __wrk__ 测试性能
