# 数据结构和限定

> `SmallSerial`、`Serial` 本质上不是一个类型，这里为了方便将它作为一个类型，希望大家能理解原意

## 用户

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
id | `SmallSerial` | | `PRIMARY KEY`
simple_name | `VarChar` | 非空、长度小于 __20__ 个字符、只能包含 `[A-z]`、`[a-z]`、`[0-9]`、`_` | 用户的机器可读名称
avatar_url | `VarChar` | 长度小于 __500__ 字符 | 用户的头像 _URL_
user_name | `VarChar` | 非空、长度小于 __100__ 字符 | 用户名
alias | `VarChar` | 长度小于 __50__ 字符 | 用户别名
github | `VarChar` | 长度小于 __50__ 字符 | _GitHub_ 用户名
bio | `VarChar` | 非空、长度小于 __500__ 字符 | `DEFAULT ''` 用户自我介绍
dev_bio | `VarChar` | 长度小于 __500__ 字符 | 作为开发者的自我介绍
created_at | `TimeStamp` | 非空 | `DEFAULT now()` 用户创建时间
online_at | `TimeStamp` | 非空 | `DEFAULT now()` 用户最近上线时间，由用户手动更新
followers_num | `SmallInt` | 非空 | `DEFAULT 0` 用户的跟随者数目
enabled | `Boolean` | 非空 | `DEFAULT true` 用户是否启用

## 分类

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
id | `SmallSerial` | | `PRIMARY KEY`
category_name | `VarChar` | 非空 | 分类名，因为只有内部人员操作不加限制
parent_category | `SmallInt` | | 父 _分类_

## 应用

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
id | `SmallSerial` | | `PRIMARY KEY`
author_user | `SmallInt` | 非空 | 创建者 _UID_
category | `SmallInt` | 非空 | 归属 _TID_
package_name | `VarChar` | 长度小于 __60__ 字符 | 包名
app_name | `VarChar` | 非空、长度小于 __60__ 字符 | 应用名
alias | `VarChar` | 长度小于 __60__ 字符 | 别名
icon_url | `VarChar` | 长度小于 __500__ 字符 | 图标 _URL_
app_description | `VarChar` | 非空、长度小于 __10000__ 字符 | `DEFAULT ''` 应用描述
visualizer | `VarChar` | 长度小于 __20__ 字符 | 应用模型视图
button_text | `VarChar` | 长度小于 __60__ 字符 | _安装卸载按钮_ 文本覆盖
special | `VarChar` | 长度小于 __12__ 字符 | 特殊标识
previews | `VarChar` | 长度小于 __4000__ 字符 | 以 `';'` 切分的预览图 _URL_
app_permissions | `VarChar` | 长度小于 __10000__ 字符 | 以 `'\n'` 切分的权限列表
size | `Integer` | 非空 | `DEFAULT 0` 应用安装包以 __百字节__ 计体积
created_at | `TimeStamp` | 非空 | `DEFAULT now()` 创建时间
updated_at | `TimeStamp` | 非空 | `DEFAULT now()` 更新时间
stars_num | `SmallInt` | 非空 | `DEFAULT 0` 星标数
comments_num | `Integer` | 非空 | `DEFAULT 0` 评论数

## 评论

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
id | `Serial` | | `PRIMARY KEY`
author_user | `SmallInt` | 非空 | 创建人 _UID_
app | `SmallInt` | 非空 | 评论 _AID_
reply_comment | `Integer` | | 回复 _CID_
content | `VarChar` | 非空、长度小于 __7000__ 字符 | 评论内容
stars_num | `SmallInt` | 非空 | `DEFAULT 0` 星标数
replies_num | `Integer` | 非空 | `DEFAULT 0` 回复数
created_at | `TimeStamp` | 非空 | `DEFAULT now()` 创建时间
updated_at | `TimeStamp` | | 修改时间

## 跟随

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | 非空 | 跟随者 _UID_
followed_user | `SmallInt` | 非空 | 被跟随者 _UID_

## 密码

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | | `PRIMARY KEY` 目标用户
metapass | `VarChar` | 非空 | 分发密码
passhash | `VarChar` | | __SHA-256__ 密码取样

## 更新

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
app | `SmallInt` | 非空 | 目标 _AID_
version_name | `VarChar` | 非空，长度小于 __40__ 字符 | 版本名
reversion | `SmallInt` | 非空 | 修订号
install_url | `VarChar` | 非空，长度小于 __500__ 字符 | 安装 _GFC_
updates | `VarChar` | 非空，长度小于 __6000__ 字符 | 更新内容
api_min | `SmallInt` | | 最低 SDK 版本
api_target | `SmallInt` | | 目标 SDK 版本

## 星标

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | 非空 | 操作 _用户_
app | `SmallInt` | 非空 | 目标 _应用_

## 评论星标

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | 非空 | 操作 _用户_
comment | `Integer` | 非空 | 目标 _评论_

## 时间线

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | 非空 | _用户_
created_at | `TimeStamp` | 非空 | `DEFAULT now()` 创建时间
line_type | `SmallInt` | 非空 | 时间线类型
line_data | `Integer` | 非空 | 时间线数据

## 通知

字段名 | 类型 | 限制 | 注
| - | :-: | :-: | -:
uid | `SmallInt` | 非空 | _用户_
created_at | `TimeStamp` | 非空 | `DEFAULT now()` 创建时间
notification_type | `SmallInt` | 非空 | 通知类型
notification_data | `Integer` | 非空 | 通知数据
enabled | `Boolean` | 非空 | `DEFAULT false` 已经阅读
