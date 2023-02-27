local M = {}

local blame_ns = vim.api.nvim_create_namespace("blame")
local blame_enabled = false
local blame_cursor_move

local function add_virtual_text(text, opt)
    local bufnr = vim.api.nvim_get_current_buf()
    local line, col = unpack(vim.api.nvim_win_get_cursor(0))

    local opts = {
        end_line = line,
        id = 1,
        virt_text = { { text, opt.hl or "Comment" } },
        virt_text_pos = "eol",
    }
    M.mark = vim.api.nvim_buf_set_extmark(bufnr, blame_ns, line - 1, col, opts)
end

local function remove_virtual_text()
    local bufnr = vim.api.nvim_get_current_buf()
    vim.api.nvim_buf_del_extmark(bufnr, blame_ns, M.mark)
end

local function show_line_blame()
    M.remove_virtual_text()
    local line = vim.api.nvim_win_get_cursor(0)[1]
    local file = vim.fn.expand("%")
    local blame = M.core.get_line_blame(file, line)
    M.add_virtual_text(blame)
end

M.config = {
    enabled = true,
    peek_timeout = 4000,
    hl = "Comment",
}

function M.peek()
    if blame_enabled then return end
    show_line_blame()
    vim.defer_fn(M.remove_virtual_text, M.config.peek_timeout)
end

function M.toggle()
    if blame_enabled then
        remove_virtual_text()
        vim.api.nvim_del_autocmd(blame_cursor_move)
        blame_cursor_move = nil
    else
        blame_cursor_move = vim.api.nvim_create_autocmd("CursorMove", {
            callback = function()
                show_line_blame()
            end
        })
    end
    blame_enabled = not blame_enabled
end

function M.setup(opt)
    M.config = vim.tbl_deep_extend("force", M.config, opt)

    if type(opt.hl) == 'string' then
        if opt.hl:sub(1, 1) == '#' then
            vim.highlight.create('BlamLine', { fg = opt.hl, bg = 'none' })
            M.config.hl = 'BlamLine'
        else
            M.config.hl = opt.hl
        end
    elseif type(opt.hl) == "table" then
        if opt.hl.fg and opt.hl.bg then
            vim.highlight.create('BlamLine', { fg = opt.hl.fg, bg = opt.hl.bg })
        end
        M.config.hl = 'BlamLine'
    end

    blame_enabled = M.config.enabled

    M.core = require('blam.core')
end

return M
