# JSON Objects

## __用户__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
id | number | 用户 _UID_
simple_name | string | 用户简名
avatar_url | string / null | 用户头像链接
name | string | 用户名
alias | string / null | 用户别名
github | string / null | github 名
bio | string | 介绍（可省略）
dev_bio | string / null | 开发者介绍（可省略）
created_at | number(datetime) | 创建时间
online_at | number(datetime) | 上线时间
followers_num | number | 跟随人数
enabled | boolean | 启用状态

### User Examples

```json
{
    "id": 3,
    "simple_name": "rachel030219",
    "avatar_url": "https://avatars1.githubusercontent.com/u/13704467?s=460&v=4",
    "name": "Rachel",
    "alias": null,
    "github": "Rachel030219",
    "bio": "Rachel 坏耶",
    "dev_bio": "Code? Is that edible?",
    "created_at": 1527689792253,
    "online_at": 1527689792253,
    "followers_num": 15,
    "enabled": true
}
```

```json
{
    "id": 233,
    "simple_name": "yuuta",
    "avatar_url": "https://avatars0.githubusercontent.com/u/17158086?s=460&v=4",
    "name": "Yuuta",
    "alias": "Trumeet",
    "github": "Trumeet",
    "bio": "Trumeet 好耶",
    "dev_bio": null,
    "created_at": 1527689792353,
    "online_at": 1527689795253,
    "followers_num": 200,
    "enabled": true
}
```

```json
{
    "id": 234,
    "simple_name": "pandecheng",
    "avatar_url": "https://avatars0.githubusercontent.com/u/26184272?s=460&v=4",
    "name": "pandecheng",
    "alias": null,
    "github": "pandecheng36",
    "bio": "",
    "dev_bio": "女装大佬，图标包高产赛 **，日常修图改图",
    "created_at": 1527689892353,
    "online_at": 1527689995253,
    "followers_num": 200,
    "enabled": false
}
```

~~小潘日常被管理欺负~~

## __分类__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
id | number | 分类 _TID_
name | string | 分类名
parent | number | 父分类 _TID_

### Category Examples

```json
{
    "id": 1,
    "name": "应用",
    "parent": null
}
```

```json
{
    "id": 2,
    "name": "工具",
    "parent": 1
}
```

```json
{
    "id": 3,
    "name": "编辑器",
    "parent": 2
}
```

## __应用__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
id | number | 应用 _AID_
author | number | 所有者 _UID_
category | number | 分类 _TID_
package | string / null | 包名
name | string | 应用名
alias | string / null | 别名
icon_url | string / null | 图标链接
desc | string | 描述（可省略）
visual | string / null | 展示器名（可省略）
button | string / null | 按钮文本覆盖（可省略）
special | string / null | 特殊文本
previews | string / null | 预览 url 文本（可省略）
permissons | string / null | 权限（可省略）
size | number | 安装包大小
created_at | number(datetime) | 创建时间
updated_at | number(datetime) | 更新时间
stars_num | number | 星标数目
comments_num | number | 评论数目

### App Examples

```json
{
    "id": 1,
    "author": 2,
    "category": 3,
    "package": "com.drakeet.purewriter",
    "name": "纯纯写作",
    "alias": "Pure Writer",
    "icon_url": ":coolapk",
    "desc": ":apkpure",
    "visual": null,
    "button": null,
    "special": "永不打折降价",
    "previews": ":coolapk",
    "permissions": ":coolapk",
    "size": 23333,
    "created_at": 0,
    "updated_at": 0,
    "stars_num": 500,
    "comments_num": 100
}
```

```json
{
    "id": 3,
    "author": 233,
    "category": 7,
    "package": "kh.android.dir",
    "name": "Dir",
    "alias": null,
    "icon_url": ":coolapk",
    "desc": ":apkpure",
    "visual": null,
    "button": "获取~/删除 QAQ/Getting...",
    "special": null,
    "previews": ":coolapk",
    "permissions": ":coolapk",
    "size": 23333,
    "created_at": 0,
    "updated_at": 0,
    "stars_num": 500,
    "comments_num": 100
}
```

```json
{
    "id": 5,
    "author": 0,
    "category": 12,
    "package": "org.geekapk.virtual.news.android-pill",
    "name": "Android Pill",
    "alias": "药丸的 Android",
    "icon_url": "https://example.org/favicon.ico",
    "desc": "日常放送一点 :pill: 的 Android 资讯",
    "visual": "news",
    "button": null,
    "special": null,
    "previews": "",
    "permissions": "",
    "size": 0,
    "created_at": 0,
    "updated_at": 0,
    "stars_num": 700,
    "comments_num": 100
}
```

## __评论__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
id | number | 评论 _CID_
author | number | 所有者 _UID_
app | number | 所在 _AID_
reply | number / null | 回复给 _CID_ 或不是回复
content | string | 内容
stars_num | number | 星标数目
replies_num | number | 回复数目
created_at | number(datetime) | 创建时间
updated_at | number(datetime) / null | 更新时间或没有更新过

### Comment Examples

```json
{
    "id": 306,
    "author": 233,
    "app": 1,
    "reply": null,
    "content": "好耶（跑",
    "stars_num": 0,
    "replies_num": 0,
    "created_at": 1527693014709,
    "updated_at": null
}
```

```json
{
    "id": 500,
    "author": 234,
    "app": 5,
    "reply": 451,
    "content": "这个厉害了&em@doge/smile!&",
    "stars_num": 4,
    "replies_num": 0,
    "created_at": 1527693014809,
    "updated_at": null
}
```

```json
{
    "id": 62,
    "author": 3,
    "app": 3,
    "reply": null,
    "content": "&em@emoji/fish!&",
    "stars_num": 0,
    "replies_num": 0,
    "created_at": 1527693113700,
    "updated_at": 1527693113809
}
```

## __更新__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
app | number | 更新目标 _AID_
ver | string | 版本名
rev | number | 修订号
install_url | string | 安装链接
updates | string | 更新内容
api_min | number / null | 最低兼容 __Android API__ 版本
api_target | number / null | 目标 __Android API__ 版本

### Update Examples

```json
{
    "app": 1,
    "ver": "3.0.0",
    "rev": 14,
    "install_url": "coolapk:com.drakeet.purewriter",
    "updates": ":coolapk",
    "api_min": 17,
    "api_target": 24
}
```

## __时间线__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
u | number | 用户 _UID_
t | number | 时间线类型
d | number | 数据
c | number(datetime) | 创建日期

### Timeline Examples

```json
{
    "u": 233,
    "t": 8,
    "d": 1,
    "c": 1527693113809
}
```

## __通知__ JSON

| 字段名 | 类型 | 描述
| - | :-: | -:
user | number | 用户 _UID_
created_at | number(datetime) | 创建时间
type | number | 通知类型
data | number | 数据
enabled | boolean | 是否已阅

### Notification Examples

```json
{
    "user": 233,
    "created_at": 1527693113809,
    "type": 1,
    "data": 542,
    "enabled": false
}
```
