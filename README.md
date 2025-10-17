# mewow-dict
> base on [mdict-rs](https://github.com/zhimoe/mdict-rs) 

a simple web dictionary write in rust, base on mdx format dictionary file.
it's at an early stage of development, now only support mdx version 2.0 with encrypted=2 or 0

## Features
- ✅ MDX file parsing and indexing
- ✅ MDD resource file parsing (images, audio, CSS, JS, etc.)
- ✅ Web-based dictionary interface
- ✅ Support for multiple dictionary files

## usage
### run from code

1. the application will load mdx files in `resources/mdx` and `resources/user_mdx` by default,if you places your mdx file 
in other directory, you can change this `BUILTIN_DICT_DIR` and `USER_DICT_DIR` environment variable to specify the directory.
2. if your mdx file has a separate mdd resource file (images, audio, etc.), place it in the same directory with the same name 
(e.g., `dict.mdx` and `dict.mdd`). Resources will be automatically loaded and accessible at `/mdd/{resource_path}`
3. if your mdx file has separate css file, put it in `resources/static/` folder
4. run the application `cargo run --bin mewow-dict`

```bash
cargo run --bin mewow-dict
# now open your chrome, and search
# http://localhost:8181
``` 
### run from docker
```bash
docker run -p 8181:8181 -v /path/to/your/mdx:/app/dicts/user -v /path/to/your/static:/app/static czyt/mewow-dict:latest
```

## MDD Resource Support

MDD (Mdict Data) files contain resources like images, audio, CSS, and JavaScript files referenced in dictionary entries. 

### How it works:
1. Place `.mdd` files in the same directory as your `.mdx` files
2. The application automatically loads all MDD files on startup
3. Resources are accessible via the URL pattern: `/mdd/{resource_path}`
4. Supported resource types:
   - Images: jpg, png, gif, svg, webp, ico
   - Audio: mp3, wav, ogg
   - Video: mp4, webm
   - Fonts: woff, woff2, ttf, otf
   - Web assets: css, js, html, json, xml

### Example:
If your dictionary HTML references an image like `<img src="example.png">`, you can update it to `<img src="/mdd/example.png">` and the image will be served from the MDD file.


## reference

+ MDX analysis[mdict-analysis](https://bitbucket.org/xwang/mdict-analysis/src/master/)
and a blog post [mdx-mdd-file-format](http://einverne.github.io/post/2018/08/mdx-mdd-file-format.html)
+ [free mdx download](https://mdict.org)
+ [wikit](https://github.com/ikey4u/wikit)
