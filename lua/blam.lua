local M = {}

local blame_ns = vim.api.nvim_create_namespace("blame")

function M.add_virtual_text(text)
    local bufnr = vim.api.nvim_get_current_buf()
    local line, col = unpack(vim.api.nvim_win_get_cursor(0))

    local opts = {
        end_line = line,
        id = 1,
        virt_text = { { text, "Comment" } },
        virt_text_pos = "eol",
        --virt_text_win_col = 0
    }
    M.mark = vim.api.nvim_buf_set_extmark(bufnr, blame_ns, line - 1, col, opts)
end

function M.remove_virtual_text()
    local bufnr = vim.api.nvim_get_current_buf()
    vim.api.nvim_buf_del_extmark(bufnr, blame_ns, M.mark)
end

function M.peek_blame()
    local line = vim.api.nvim_win_get_cursor(0)[1]
    local file = vim.fn.expand("%")
    local blame = vim.fn.system("git blame -L " .. line .. "," .. line .. " --porcelain -- " .. file)
    return blame
end

return M
