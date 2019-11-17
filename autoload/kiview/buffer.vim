
let s:buffers = {}

function! s:new(bufnr, range) abort
    if has_key(s:buffers, a:bufnr)
        let buffer = s:buffers[a:bufnr]
        call buffer.current.update(a:range)
        return buffer
    endif

    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'register': kiview#register#new(),
        \ 'history': kiview#history#new(a:bufnr),
        \ 'current': kiview#current#new(a:bufnr),
        \ 'logger': kiview#logger#new('buffer'),
    \ }

    function! buffer.open() abort
        leftabove vsplit
        execute 'buffer' self.bufnr

        setlocal filetype=kiview
        setlocal nonumber
        call nvim_win_set_width(win_getid(), 38)
        call nvim_buf_set_option(self.bufnr, 'bufhidden', 'wipe')

        call self.logger.log('opend bufnr: ' . self.bufnr)
    endfunction

    function! buffer.write(lines, start, end) abort
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, a:start, a:end, v:true, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        call self.logger.label('line').buffer_log(self.bufnr)
    endfunction

    function! buffer.write_all(lines) abort
        call self.write(a:lines, 0, -1)
    endfunction

    function! buffer.close_windows() abort
        let window_ids = win_findbuf(self.bufnr)
        for id in window_ids
            call nvim_win_close(id, v:false)
        endfor
    endfunction

    let s:buffers[a:bufnr] = buffer
    execute printf('autocmd BufWipeout <buffer=%s> call s:clean("%s")', a:bufnr, a:bufnr)

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

function! s:clean(bufnr) abort
    if !has_key(s:buffers, a:bufnr)
        return
    endif
    call remove(s:buffers, a:bufnr)
endfunction
