
function! s:new(bufnr, range) abort
    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'register': kiview#register#new(a:bufnr),
        \ 'history': kiview#history#new(a:bufnr),
        \ 'current': kiview#current#new(a:bufnr, a:range),
        \ 'logger': kiview#logger#new('buffer'),
    \ }

    function! buffer.open() abort
        leftabove vsplit
        execute 'buffer' self.bufnr

        setlocal filetype=kiview
        setlocal nonumber
        call nvim_win_set_width(win_getid(), 38)

        call self.logger.log('opend bufnr: ' . self.bufnr)
    endfunction

    function! buffer.write(lines) abort
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, 0, -1, v:true, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        call self.logger.label('line').buffer_log(self.bufnr)
    endfunction

    function! buffer.close_windows() abort
        let window_ids = win_findbuf(self.bufnr)
        for id in window_ids
            call nvim_win_close(id, v:false)
        endfor
    endfunction

    return buffer
endfunction

function! kiview#buffer#find(range) abort
    if &filetype !=? 'kiview'
        let bufnr = nvim_create_buf(v:false, v:true)
        return s:new(bufnr, a:range)
    endif

    let bufnr = bufnr('%')
    return s:new(bufnr, a:range)
endfunction
