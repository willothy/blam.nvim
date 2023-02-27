# Blam

Inspired by VSCode and GitLens, this plugin simply adds blame info as virtual text to the end of your current line. Written in Rust using `git2` and `nvim-utils`.



https://user-images.githubusercontent.com/38540736/221464520-057099bb-e6d5-47d9-98bc-b177021b5e42.mp4



## Installation

Install using your favorite package manager

### Lazy.nvim
```lua
{
    'willothy/blam.nvim',
    build = 'make',
    config = true
}
```

### Packer.nvim
```lua
use('willothy/blam.nvim', {
    run = 'make',
    config = function()
        require("blam").setup()
    end
})
```

## Usage

Blam includes no mappings, just a few functions:
```lua
-- Toggles line blame (will update as you move the cursor)
require("blam").toggle()

-- Peeks the blame for the current line (will disappear when cursor is moved or after a timeout)
require("blam").peek()
```

Here's my setup:
```lua
{
    'willothy/blam.nvim',
    build = 'make',
    config = true,
    init = function()
        vim.keymap.set("n", "<leader>b", require("blam").peek) 
    end
}
```

## Configuration

Blam comes with the following defaults:
```lua
{
    -- Can be a hex color
    -- or a table with fg and bg colors
    -- or a highlight group
    hl = "Comment",
    -- Timeout before peek blame disappears 
    -- Set to 0 to disable and only hide peek blame on cursor move
    peek_timeout = 0,
    -- Whether line blame will be enabled on startup
    -- Disabled by defauly as it can be a bit intrusive (I prefer using peek)
    enabled = false,
}
```


