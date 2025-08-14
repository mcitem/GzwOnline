# 概要

![image](https://github.com/user-attachments/assets/d1d4fec4-42ea-4e7c-b6bc-f7cfb2e41d7f)

```
Root
├── API                      后端 Rust crate
│   └── migration            sea-orm 迁移工具
│       ├── category.xlsx    藏品分类数据
│       ├── collection.xlsx  藏品数据
│       └── photo_album.xlsx 其他图片数据
└── APP uni-app(Vue3) 项目目录
```

# 运行

```
cd API
echo "DATABASE_URL = sqlite://sqlite.db?mode=rwc" > .env
cargo run -p migration
cargo run
```

```
pnpm --filter app dev
```

# Refer

MiceFine Museum (MIT License)
- https://github.com/micefind/museum
- https://gitee.com/micefind/museum
