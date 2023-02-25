local M = {}


function M.peek_blame()
    local line = vim.api.nvim_win_get_cursor(0)[1]
    local blame = vim.fn.system("git blame -L " .. line .. "," .. line .. " --porcelain")
end

return M
