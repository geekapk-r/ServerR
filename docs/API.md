# GeekApk RESTFul APIs

如果请求使用数据的长度过长，返回 `413 Payload Too Large`

如果验证失败，返回 `401 Unauthorized`

如果找不到资源，返回 `404 Not Found`

如果请求类型不合适，返回 `415 Unsupported Media Type`

如果内容解析失败，返回 `400 Bad Request`

## 其他 API

> 额外访问 __密码__ 表的内容

### 主页

`GET /` 返回主页 `web.html`

### 版本查询

`GET /version` 返回以 `':'` 分割的 API 版本、程序版本

### Ping

`GET /ping` 返回你的 IP 地址

### Validate

以 metapass 为 body `POST /validate-metapass/<uid>` 获得验证是否通过（200/非 200）

以 passhash 为 body `POST /validate/<uid>` 获得验证是否通过（200/非 200）

### 修改密码

以分发密码为 body，`PUT /pass/<uid>` 可以创建/更新用户密码

如果成功，返回 `200 OK`

如果验证失败，返回 `401 Unauthorized`

如果找不到用户，返回 `404 Not Found`

### WebHooks API

`GET /webhooks` 查询 WebHooks 配置

```json
[
    {
        "type": "replyToMessage",
        "data": 2333,
        "url": "https://bar.org/bot"
    }
]
```

### Admin API

> 以下 API 都需要 __权限汪__ 访问权限

如果验证失败，返回 `401 Unauthorized`

#### 创建用户

`POST /doge/useradd`

body 包含

```ruby
simple_name: string,
avatar_url: string / null,
user_name: string / null,
alias: string / null,
github: string,
bio: string(optional),
dev_bio: string / null
```

如果返回码为 `201 Created`，body 就应该是以 `':'` 切分的被创建的用户 _UID_ 文本表示和 `metapass`

#### 删除用户

`DELETE /doge/user/<uid>`

如果成功删除，返回 `410 Gone`

如果找不到用户，返回 `404 Not Found`

#### 改变用户停用状态

`PUT /doge/user/<uid>`

如果成功，则返回 `200 OK` 和 Json

例如，

```json
{
    "uid": 213,
    "enabled": true
}
```

如果找不到用户，返回 `404 Not Found`

#### 修改分发密码

以新 metapass 为 body `PUT /doge/user/<uid>/metapass`

如果找不到用户，返回 `404 Not Found`

如果成功，则返回 `200 OK` 和 Json

例如，

```json
{
    "uid": 233,
    "old-metapass": "old",
    "metapass": "new"
}
```

#### 分类管理

##### CREATE

以分类名为 body `POST /doge/category/<parent>`

如果成功，返回 `200 OK` 和已创建 _CID_ 文本表示

##### UPDATE

以新分类名为 body `PUT /doge/category/<tid>`

以新父分类为 body `PUT /doge/category/<tid>/parent`

如果成功，返回 `200 OK` 和 Json

例如，

```json
{
    "tid": 32,
    "name": "实用工具",
    "parent": 3
}
```

##### DELETE

`DELETE /doge/category/<tid>`

如果成功，返回 `410 Gone`

#### 删除评论

`DELETE /doge/comment/<cid>`

如果有子评论，删除评论也会在应用 __0__ 下以被删除所有者身份新建顶级评论 `Replies Deleted from {aid} (reply to {reply_comment})` 并将所有子评论的 `reply_comment` 字段修改为 `该评论 ID` 并且把 `app` 字段修改为 __0__，直接被删除的内容不会保留

`DELETE /doge/comment/<cid>`

如果添加 url 参数 `recursive` 则递归删除所有子评论

如果成功，返回 `410 Gone`

如果没找到评论，返回 `404 Not Found`

#### 删除应用

`DELETE /doge/comment/<cid>`

如果有评论，在应用 __0__ 下以应用发布者身份创建顶级评论 `Comments Deleted from {aid}` 并将所有评论的 `app` 字段修改为 __0__，`reply_comment` 修改为评论的 _cid_

如果添加 url 参数 `recursive` 则递归删除所有评论

如果成功，返回 `410 Gone`

如果没找到评论，返回 `404 Not Found`

## __用户__ API

> 额外访问 __跟随__ 表的内容

`GET /user/<uid>` 可以获取目标用户的 _User JSON_，如果使用 `short` 参数就不传输一些字段（简短 JSON）

`GET /user/all` 可以获得全站用户 _UID_ JSON 列表，参数

+ `max=num` 可以设置最大返回条数
+ `s=ctime` 可以设置以创建时间排行
+ `s=otime` 可以设置以上线时间排行
+ `s=followers` 可以设置以跟随人数徘行
+ `m=asc/dsc` 以设置是 `asc` 正序还是 `dsc` 倒序，默认正序

以关键字为 body `POST /user/search` 以搜索用户，返回用户 _UID_ JSON 列表，参数

+ `mode=sname/name/alias/github/bio/dev_bio` 以 __简单名、用户名、别名、GitHub、自述、开发者自述__ 搜索
+ `max=num` 可以设置最大返回条数
+ `s=ctime` 可以设置以创建时间排行
+ `s=otime` 可以设置以上线时间排行
+ `s=followers` 可以设置以跟随人数徘行
+ `m=asc/dsc` 以设置是 `asc` 正序还是 `dsc` 倒序，默认正序

`GET /user/<uid>/followers` 可以获取目标用户的跟随者 _用户_ UID JSON 列表，默认排行

`GET /user/<uid>/following` 可以获取目标用户的跟随 _用户_ UID JSON 列表，默认排行

`GET /user/<uid>/follow` 返回 follow 状态 `true/false`

`PUT /user/<uid>/follow` 改变 follow 状态，返回新状态 `true/false`

`GET /user/<uid>/online` 可以获取用户上线时间

`GET /user/<uid>/apps` 可以获取用户的应用 _AID_ JSON 列表，以更新时间排行

`GET /user/<uid>/comments` 可以获取用户的评论 _CID_ JSON 列表，以发布时间排行

`GET /user/<uid>/stars` 可以获得用户的星标应用 _AID_ JSON 列表

`GET /user/<uid>/cstars` 可以获得用户的星标评论 _CID_ JSON 列表

`GET /user/<uid>/timeline` 可以获得用户的时间线 JSON 对象列表，参数

+ `max=num` 设置最大返回条数

`GET /user/<uid>/notifications` 可以获得用户的通知 JSON 对象列表，需要验证

+ `all` 参数以返回已阅消息

以 User JSON 为 body `PUT /user/<uid>` 可以更新目标用户除 `id, created_at, followers_num, enabled` 外的信息

`PUT /user/<uid>/online` 可以更新上线时间为当前时间

`DELETE /user/<uid>` 可以删除自己，言论和应用不会删除

## __分类__ API

`GET /category/all` 获取所有分类 JSON 列表

`GET /category/<tid>` 获取这个分类的 JSON 对象

`GET /category/<tid>/parent` 获取这个分类的父分类

`GET /category/<tid>/apps` 获取这个分类的应用 _AID_ JSON 列表，参数

+ `max=num` 条数限制
+ `m=asc/dsc` 以设置是 `asc` 正序还是 `dsc` 倒序，默认正序
+ `s=ctime/updated/star/replies/size` 以 __创建时间、更新时间、星标数、评论数、体积__ 排序

以关键字为 body `POST /category/<tid>/search` 在分类中搜索应用，返回 _AID_ JSON 列表，参数

+ `max=num` 条数限制
+ `s=name/pkg/desc` 检索类型，__应用名、包名、描述__

## __应用__ API

> 额外访问 __星标__ 表的内容

以包含 `author_user, category, package_name(optional), app_name, alias(optional), icon_url(optional), app_description, visualizer(optional), button_text(optional), special(optional), previews(optional), app_permissions(optional), size` 的 App JSON 为 body `POST /app` 创建应用，返回应用 _AID_

`GET /app/all` 获取所有应用 _AID_ JSON 列表，按更新时间排序

`GET /app/<aid>` 获取 App JSON，可选 `short` 参数获取简版对象

`GET /app/<aid>/stars` 获取 star 的用户 _UID_ JSON 列表

`GET /app/<aid>/star` 查询星标状态，返回 `true/false`

`PUT /app/<aid>/stars` 以星标/取消星标应用，返回新状态 `true/false`

`GET /app/<aid>/comments` 获取评论列表，参数

+ `max=num` 条数限制
+ `s=ctime/utime/stars/replies` __创建时间、更新时间、评论星标数、回复数__ 排行

`GET /app/<aid>/updates` 获取更新 JSON 对象列表，以 `reversion` 排行

+ `max=num` 条数限制

以包含 `version_name, reversion, install_url, updates, api_min(optional), api_target(optional)` 的 Update JSON 为 body `POST /app/<aid>/update` 创建更新

以包含 `reply_comment, content` 的 comment JSON `POST /app/<aid>/comment` 创建评论，返回评论 _CID_

以 App JSON 为 body `PUT /app/<aid>` 更新应用的非 `id, author_user, created_at, updated_at, stars_num, comments_num` 字段

以关键字为 body `POST /app/search` 搜索应用，返回 _AID_ JSON 列表，参数

+ `max=num` 条数限制
+ `s=name/pkg/desc` 检索类型，__应用名、包名、描述__

`DELETE /app/<aid>` 以删除应用，评论将被迁移到特殊应用 __0__ 的评论下

## __评论__ API

> 额外访问 __评论星标__ 表的内容

`GET /comment/all` 获取创建时间排行的全站 _CID_ JSON 列表

+ `max=num` 条数限制

`GET /comment/<cid>` 获取评论 JSON 对象

以新内容为 body `PUT /comment/<cid>` 更新评论内容

`GET /comment/<cid>/star` 查询星标状态，返回 `true/false`

`PUT /comment/<cid>/star` 以星标/取消星标评论，返回新状态 `true/false`

`GET /comment/<cid>/replies` 获取回复评论的 _CID_ JSON 列表

`GET /comment/<cid>/stars` 获取星标用户 _UID_ JSON 列表

以关键字为 body `POST /comment/search` 搜索评论，返回 _CID_ JSON 列表，参数

+ `max=num` 条数限制

`DELETE /comment/<cid>` 删除评论，子评论会被迁移

## __更新__ API

以 App JSON 为 body `PUT /updates/<aid>/<reversion>` 更新 `version_name, install_url, updates, api_min, api_target`

`DELETE /updates/<aid>/<reversion>` 以删除指定 reversion

`GET /updates/<aid>` 以获得 reversion 数目

## __时间线__ API

`GET /timeline` 获取全站时间线 Timeline JSON 对象列表

+ `max=num` 限制条数

`GET /timeline/<uids>` 获取以 `','` 切分 uid 列表的时间线 JSON 对象列表

+ `max=num` 限制条数

## __通知__ API

`POST /notification/<uid>/<cid>` 新建一个 `@`

`PUT /notification/<uid>/<created>` 更新 `enabled` 状态，返回新状态 `true/false`

`DELETE /notification/<uid>/<created>` 删除目标通知
