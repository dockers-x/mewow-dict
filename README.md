# mewow-dict
> base on [mdict-rs](https://github.com/zhimoe/mdict-rs) 

a simple web dictionary write in rust, base on mdx format dictionary file.
it's at an early stage of development, now only support mdx version 2.0 with encrypted=2 or 0

## usage
### run from code

1. the application will load mdx files in `resources/mdx` and `resources/user_mdx` by default,if you places your mdx file 
in other directory, you can change this `BUILTIN_DICT_DIR` and `USER_DICT_DIR` environment variable to specify the directory.
2. if your mdx file has separate css file, put it in `resources/static/` folder
3. run the application `cargo run --bin mewow-dict`

```bash
cargo run --bin mewow-dict
# now open your chrome, and search
# http://localhost:8181
``` 
### run from docker
```bash
docker run -p 8181:8181 -v /path/to/your/mdx:/app/dicts/user -v /path/to/your/static:/app/static czyt/mewow-dict:latest
```


## reference

+ MDX analysis[mdict-analysis](https://bitbucket.org/xwang/mdict-analysis/src/master/)
and a blog post [mdx-mdd-file-format](http://einverne.github.io/post/2018/08/mdx-mdd-file-format.html)
+ [free mdx download](https://mdict.org)
