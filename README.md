# Refine

在阅读源码时经常需要复制一块注释到 Google 翻译或 ChatGPT 中进行翻译，
但带着注释中的 `///` `//` `/*` 等符号非常影响翻译的效果，和阅读体验。
所以写了一个小工具，用来处理复制后粘贴板中的内容，
自动去除注释符号，并使用空格拼接为一行，再写回粘贴板。

## Example

原始粘贴板内容
``` rust
	/// Whether to wait for the clipboard's contents to be replaced after setting it.
	///
	/// The Wayland and X11 clipboards work by having the clipboard content being, at any given
	/// time, "owned" by a single process, and that process is expected to reply to all the requests
	/// from any other system process that wishes to access the clipboard's contents. As a
	/// consequence, when that process exits the contents of the clipboard will effectively be
	/// cleared since there is no longer anyone around to serve requests for it.
```

执行命令
```shell
refine
```

处理后粘贴板中的内容
```
Whether to wait for the clipboard's contents to be replaced after setting it. The Wayland and X11 clipboards work by having the clipboard content being, at any given time, "owned" by a single process, and that process is expected to reply to all the requests from any other system process that wishes to access the clipboard's contents. As a consequence, when that process exits the contents of the clipboard will effectively be cleared since there is no longer anyone around to serve requests for it.
```

