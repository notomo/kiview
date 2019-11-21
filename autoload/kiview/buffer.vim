
let s:buffers = {}

function! kiview#buffer#new(range) abort
    let bufnr = bufnr('%')
    if has_key(s:buffers, bufnr)
        let buffer = s:buffers[bufnr]
        call buffer.current.update(a:range)
        return buffer
    endif

    let bufnr = nvim_create_buf(v:false, v:true)
    let buffer = {
        \ 'bufnr': bufnr,
        \ 'register': kiview#register#new(),
        \ 'history': kiview#history#new(bufnr),
        \ 'current': kiview#current#new(bufnr),
        \ 'logger': kiview#logger#new('buffer'),
        \ 'props': {},
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

    function! buffer.write(lines, props, start, end) abort
        call self.current.delete_marks(a:start, a:end)

        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, a:start, a:end, v:true, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        call self.current.set_props(a:props, a:start)

        call self.logger.label('line').buffer_log(self.bufnr)
    endfunction

    function! buffer.write_all(lines, props) abort
        call self.write(a:lines, a:props, 0, -1)
    endfunction

    function! buffer.close_windows() abort
        let window_ids = win_findbuf(self.bufnr)
        for id in window_ids
            call nvim_win_close(id, v:false)
        endfor
    endfunction

    let s:buffers[bufnr] = buffer
    execute printf('autocmd BufWipeout <buffer=%s> call s:clean("%s")', bufnr, bufnr)

    return buffer
endfunction

function! s:clean(bufnr) abort
    if !has_key(s:buffers, a:bufnr)
        return
    endif
    call remove(s:buffers, a:bufnr)
endfunction
