# ServerR ![[status](https://status.geekapk.org)](https://img.shields.io/uptimerobot/status/m780333733-2b10476b498c316d8fb9e684.svg?style=flat-square) ![[weekly-status](https://status.geekapk.org)](https://img.shields.io/uptimerobot/ratio/7/m780333733-2b10476b498c316d8fb9e684.svg?style=flat-square)

<p align="center">
  <a href="https://api.geekapk.org/">
    <img src="https://geekapk.org/bitmap/symbolic/geekapk-320.png" alt="Visit GeekApk API SaaS Service" width=100 height=100>
  </a>

  <h3 align="center">GeekApk API ServerR</h3>

  <p align="center">
    <a href="https://geekapk.org/">ServerR</a> is a small GeekApk HTTP&WebSocket API implementation written in <a href="https://rust-lang.org">Rust</a> with <strong>Rocket, Diesel</strong>
    <br>
    <a href="https://github.com/geekapk-r/ServerR/wiki"><strong>Explore GeekApk API docs »</strong></a>
    <br>
    <br>
      <a href="https://travis-ci.org/geekapk-r/ServerR/">
        <img src="https://img.shields.io/travis/geekapk-r/ServerR.svg?style=flat-square" alt="travis ci">
      </a>
    ·
      <a href="https://gitter.im/geekapk-r/ServerR">
        <img src="https://img.shields.io/gitter/room/geekapk-r/ServerR.svg?style=flat-square" alt="chat on gitter">
      </a>
    ·
      <a href="https://geekapk-r.github.io/ServerR">
        <img src="https://img.shields.io/badge/rustdoc-here-brown.svg?style=flat-square" alt="rustdoc">
      </a>
    ·
      <a href="https://codecov.io/gh/geekapk-r/ServerR">
        <img src="https://img.shields.io/codecov/c/github/geekapk-r/ServerR.svg?style=flat-square" alt="coverage">
      </a>
    ·
      <a href="https://www.gnu.org/licenses/agpl-3.0.html">
        <img src="https://img.shields.io/github/license/geekapk-r/ServerR.svg?style=flat-square" alt="license">
      </a>
    ·
      <a href="https://github.com/geekapk-r/ServerR/pulse">
        <img src="https://img.shields.io/github/languages/code-size/geekapk-r/ServerR.svg?style=flat-square" alt="codesize">
      </a>
  </p>
</p>


> :memo: This repository contains code of __GeekApk RDBMS RESTFul + WebSocket API Server Application__

## Introduction 介绍

ServerR 是 __GeekApk API__ 的实现，GeekApk 是一个 __自由、开放、好玩__ 的 __Android 社区__ 项目，从 [酷安](https://coolapk.com) 起始，在自由度上超过酷安

GeekApk 里主要使用 __4__ 个模型： __用户、应用、分类、评论__

GeekApk 后端支持 __用户跟随__ 和 __星标__

GeekApk 的评论模型支持无限级嵌套

GeekApk，为创建更好玩的应用社区而奋斗

## 为什么离开酷安而新建 _GeekApk_

+ 酷安在国内，会 __不可避免__ 的遭到一些 _众所周知_ 的 [事情](https://blog.nfz.moe/archives/the-silence-city.html)
+ 酷安几乎不开源他们的程序，尤其是特意[阻止](https://github.com/bjzhou/Coolapk-kotlin#%E9%85%B7%E5%B8%82%E5%9C%BA-v7-%E7%89%88-api-%E7%9B%B8%E5%85%B3) __第三方客户端访问 API__
+ 酷安的用户质量自 _某日_ 开始下降，到现在已经 [不怎么样了](https://zhuanlan.zhihu.com/p/35743455)
+ 酷安的开发者质量也开始下降，部分 __普通用户__ 开始学习简单模式化的程序设计，但他们不愿意 [自己解决容易的问题](https://stfw.info)、想方设法甚至不顾用户体验的 __加固__ 甚至连 __中文文档__ 也不愿意看，对于 __远程代码执行__ 级别的漏洞也没有任何动作
+ 酷安的维护者也发生了变化，开始主要考虑 __盈利__ 而不是社区质量，虽然这对酷安是 __必然__ 发生的，即使部分 _Geek_ 无法接受
+ 参考 [neoFelhz](https://blog.nfz.moe/archives/coolapk-in-my-mind.html)、 [Rachel](https://blog.stfw.info/2017/05/11/why-leave-coolapk)、[iKirby](https://ikirby.me/125.html) 的博文：为什么 __离开酷安__

## Docs 文档

所有文档包括 __部署__ 帮助和 __API__ 文档储存在 [wiki](https://github.com/geekapk-r/ServerR/wiki) 里，请移步查看

## Friends 友情链接

+ [neoHosts](https://github.com/neoFelhz/neohosts) - 本 README 使用了 __neoHosts__ 的模式
+ [Web](https://github.com/geekapk-r/web.geekapk.org) - Web 纯静态 _JavaScript HTML_ 客户端
+ [Dropage Project](https://legacy.gitbook.com/book/duangsuse/the-dropage-project/details) - 致敬最初的设计

## Maintainer

__ServerR © GeekApk Contributors__, Released under the __AGPL-3.0 License__.
Authored and maintained by __GeekApk Team__ and the help from other contributors

## License 许可证

GeekApk 使用常规 __自由__ 许可证，许可证可以保护 GeekApk 源代码 __不被私有化__

```plain
Copyright (C) 2018 GeekApk Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
```
