# Static File Server

English | [日本語](README.md)

```bash
staticfs HOST PORT
```

Serves the files of the current directory over HTTP. Files that start with `.` and symbolic links that point outside the directory are not served. For the specification and what to watch out for, see [docs/SPEC.md](docs/SPEC.md).
