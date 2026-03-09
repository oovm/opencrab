# OpenCrab 多语言文档 - Verification Checklist

## 基础设施验证
- [ ] documentation 目录已创建
- [ ] package.json 文件存在且包含 VitePress 依赖
- [ ] .vitepress 目录存在
- [ ] .vitepress/config.ts 配置文件存在

## 多语言配置验证
- [ ] config.ts 中配置了英文 (en)
- [ ] config.ts 中配置了简体中文 (zh-hans)
- [ ] config.ts 中配置了繁体中文 (zh-hant)
- [ ] config.ts 中配置了日语 (ja)
- [ ] config.ts 中配置了韩语 (ko)
- [ ] config.ts 中配置了德语 (de)
- [ ] config.ts 中配置了法语 (fr)
- [ ] config.ts 中配置了俄语 (ru)
- [ ] 每种语言都有导航菜单配置
- [ ] 每种语言都有侧边栏配置

## 文档内容验证
- [ ] en/ 目录存在
- [ ] en/index.md 存在
- [ ] en/overview/ 目录存在
- [ ] en/readme.md 存在
- [ ] zh-hans/ 目录存在
- [ ] zh-hans/index.md 存在
- [ ] zh-hans/overview/ 目录存在
- [ ] zh-hans/readme.md 存在
- [ ] zh-hant/ 目录存在
- [ ] zh-hant/index.md 存在
- [ ] ja/ 目录存在
- [ ] ja/index.md 存在
- [ ] ko/ 目录存在
- [ ] ko/index.md 存在
- [ ] de/ 目录存在
- [ ] de/index.md 存在
- [ ] fr/ 目录存在
- [ ] fr/index.md 存在
- [ ] ru/ 目录存在
- [ ] ru/index.md 存在

## 功能验证
- [ ] npm install 可以成功安装依赖
- [ ] npm run docs:dev 可以成功启动
- [ ] 文档站点可以在浏览器中正常访问
- [ ] 语言切换功能正常工作
- [ ] 导航链接可以正常跳转
