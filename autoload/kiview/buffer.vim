
function! kiview#buffer#new() abort
    let bufnr = nvim_create_buf(v:false, v:true)
    let buffer = {
        \ 'bufnr': bufnr,
        \ 'logger': kiview#logger#new().label('buffer'),
    \ }

    function! buffer.open() abort
        leftabove vsplit
        execute 'buffer' self.bufnr
        call self.logger.log('opend bufnr: ' . self.bufnr)
    endfunction

    function! buffer.write(lines) abort
        let length = nvim_buf_line_count(self.bufnr)
        call nvim_buf_set_lines(self.bufnr, 0, length, v:false, a:lines)
        call self.logger.logs(a:lines)
    endfunction

    return buffer
endfunction
