# Welcome to Bookyo Blog

This is a simple blog, backend powered by actic-web, frontend powerd by astro.

use actic-web + mongodb + jwt.

just rename .env.example to .env and edit
```
MONGO_URL=mongodb://rust:rust@127.0.0.1:27017/rust
MONGO_DB_NAME=rust
JWT_SECRET=testmyrustblog
HOST=http://127.0.0.1:8080
INIT_EMAIL=admin@admin.com
INIT_PASSWORD=adminadmin
INIT_USERNAME=admin
```
MONGO_URL is mongo connect uri.
MONGO_DB_NAME is database name.
JWT_SECRET is jwt authencation secret.
HOST is your domain bind this api server.
INIT_EMAIL is init user email while first start app.
INIT_PASSWORD is init user password.
INIT_USRENAME is init user username.

if you worry about malicious registration, you should comment main.rs line 49. Ban register api.

features:
- login and register with jwt token authencation.
- blog api with create, update, getOne, getMore and upload image.

backend api git: https://github.com/bookyo/rust_blog

front web git: https://github.com/bookyo/rust_blog_web

api:
- GET /blog
- GET /blogs
- POST /blog
- PATCH /blog
- POST /upload
- POST /login
- POST /register

deploy at: https://deno.com/deploy

backend api Use oracle Free Tier: https://www.oracle.com/cloud/free/

preview url: https://blog.zizhaidi.com