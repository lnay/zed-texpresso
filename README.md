# TeXpresso extension for Zed

Now working in Linux, MacOS, and Windows.

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

1. Build [TeXpresso](https://github.com/let-def/texpresso)
2. Install this [extension in Zed](zed://extension/texpresso)
3. Add to your settings.json file (global `~/.config/zed/settings.json` or workspace `WORKSPACE/.zed/settings.json`):
  ```jsonc
  {
    "lsp": {
      "texpresso-lsp": {
        "initialization_options": {
          "root_tex": "path/to/main/tex/file", // relative to workspace root, defaults to main.tex
          "texpresso_path": "path/to/texpresso/binary" // omissable if TeXpresso is in PATH
          // ^ this path must be to an executable called "texpresso" which must be in the same dir as the executable "texpresso-tonic"
        },
        "settings": {
          "preview_follow_cursor": true, // default true, makes preview constantly follow the position of the cursor
        }
      },
      "texlab": {
        "settings": {
          "texlab": {
            "build": {
              "onSave": false // we will be using texpresso to build
            }
          }
        }
      }
    },
    "languages": {
      "LaTeX": {
        "language_servers": ["texpresso-lsp", "texlab", "..."] // in workspaces where you do not want to use texpresso, replace "texpresso-lsp" with "!texpresso-lsp"
      }
    }
  }
  ```
> [!WARNING]
> This will work in <abbr title="Windows Subsystem for Linux">WSL</abbr> in Windows.
> But this means that TeXpresso must be built inside WSL and the workspace must be inside WSL too (not a Windows folder).
> You can open Zed from the WSL terminal with `zed .`. You may also need to go to the command palette and restart language servers if the preview window does not appear at first.
