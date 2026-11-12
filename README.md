# Static File Server

```bash
staticfs HOST PORT
```

カレントディレクトリのファイルを HTTP で配る。`.` で始まるファイルと、ディレクトリの外を指すシンボリックリンクは配らない。仕様と注意は [docs/SPEC.md](docs/SPEC.md)。
