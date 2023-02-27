local M = {}

local blame_ns = vim.api.nvim_create_namespace("blame")
local mark = nil
local blame_cursor_move = nil
local blame_enabled = false

local function add_virtual_text(text)
    local bufnr = vim.api.nvim_get_current_buf()
    local line, col = unpack(vim.api.nvim_win_get_cursor(0))

    local opts = {
        end_line = line,
        id = 1,
        virt_text = { { text, M.config.hl or "Comment" } },
        virt_text_pos = "eol",
    }
    mark = vim.api.nvim_buf_set_extmark(bufnr, blame_ns, line - 1, col, opts)
end

local function remove_virtual_text()
    if mark == nil then return end
    local bufnr = vim.api.nvim_get_current_buf()
    vim.api.nvim_buf_del_extmark(bufnr, blame_ns, mark)
    mark = nil
end

local function show_line_blame()
    remove_virtual_text()
    local line = vim.api.nvim_win_get_cursor(0)[1]
    local file = vim.fn.expand("%")
    local blame = M.core.get_line_blame(file, line)
    add_virtual_text(blame)
end

M.config = {
    enabled = true,
    peek_timeout = 0,
    hl = "Comment",
}

function M.peek()
    if blame_enabled then return end
    show_line_blame()
    blame_cursor_move = vim.api.nvim_create_autocmd("CursorMove", {
        callback = function()
            remove_virtual_text()
            if blame_cursor_move == nil then return end
            vim.api.nvim_del_autocmd(blame_cursor_move)
            blame_cursor_move = nil
        end
    })
    if M.config.peek_timeout > 0 then
        vim.defer_fn(remove_virtual_text, M.config.peek_timeout)
    end
end

function M.toggle()
    if blame_enabled == true then
        remove_virtual_text()
        vim.api.nvim_del_autocmd(blame_cursor_move)
        blame_cursor_move = nil
    else
        if blame_cursor_move ~= nil then
            vim.api.nvim_del_autocmd(blame_cursor_move)
        end
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

    if M.config.enabled then
        M.toggle()
    end

    M.core = require('blam.core')
end

return M
