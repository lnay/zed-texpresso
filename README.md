# TeXpresso extension for Zed

https://github.com/user-attachments/assets/477093ea-d44d-4bad-b3da-814a33c859c2

Use this alongside the LaTeX extension for Zed, but disable the build-on-save in settings:
```jsonc
  {
    "lsp": {
      "texlab": {
        "settings": {
          "texlab": {
            "build": {
              "onSave": false
            }
          }
        }
      }
    }
  }
```

## Setup

1. Build TeXpresso from this branch: https://github.com/lnay/texpresso/tree/utf-8
2. Install this repo as a [dev extension in Zed](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally), or from the extensions tab when available (maybe soon)
3. Add to your settings.json file (global `~/.config/zed/settings.json` or workspace `WORKSPACE/.zed/settings.json`):
  ```jsonc
  {
    "lsp": {
      "texpresso-lsp": {
        "initialization_options": {
          "root_tex": "path/to/main/tex/file", // relative to workspace root, defaults to main.tex
          "texpresso_path": "path/to/texpresso/binary" // omissable if TeXpresso is in PATH
        }
      }
    }
  }
  ```
