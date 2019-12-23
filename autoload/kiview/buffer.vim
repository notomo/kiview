
let s:buffers = {}
let s:id = 0

let s:split_names = {
    \ 'no': '',
    \ 'tab': 'tabedit',
    \ 'vertical': 'vsplit',
    \ 'horizontal': 'split',
\ }

function! kiview#buffer#get_or_create(range) abort
    let buffer = kiview#buffer#find(a:range)
    if !empty(buffer)
        return buffer
    endif

    return kiview#buffer#new()
endfunction

function! kiview#buffer#find(range) abort
    let bufnr = bufnr('%')
    if has_key(s:buffers, bufnr)
        let buffer = s:buffers[bufnr]
        call buffer.current.update(a:range)
        return buffer
    endif
    return v:null
endfunction

function! kiview#buffer#new() abort
    let bufnr = nvim_create_buf(v:false, v:true)

    let s:id += 1
    let name = printf('kiview://%s/kiview', s:id)
    call nvim_buf_set_name(bufnr, name)

    let buffer = {
        \ 'bufnr': bufnr,
        \ 'register': kiview#register#new(),
        \ 'history': kiview#history#new(bufnr),
        \ 'current': kiview#current#new(bufnr),
        \ 'logger': kiview#logger#new('buffer'),
    \ }

    function! buffer.open(split_name, mod_name) abort
        execute a:mod_name s:split_names[a:split_name]
        execute 'buffer' self.bufnr

        setlocal filetype=kiview
        setlocal nonumber
        call nvim_win_set_width(win_getid(), 38)
        call nvim_buf_set_option(self.bufnr, 'bufhidden', 'wipe')

        call self.logger.log('opend bufnr: ' . self.bufnr)
    endfunction

    function! buffer.write(lines, start, end) abort
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, a:start - 1, a:end - 1, v:true, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        call self.logger.label('line').buffer_log(self.bufnr)
    endfunction

    function! buffer.write_all(lines) abort
        call self.write(a:lines, 1, 0)
    endfunction

    function! buffer.close_windows() abort
        let window_ids = win_findbuf(self.bufnr)
        for id in window_ids
            call nvim_win_close(id, v:false)
        endfor
    endfunction

    let s:buffers[bufnr] = buffer
    execute printf('autocmd BufWipeout <buffer=%s> call s:clean("%s")', bufnr, bufnr)
    execute printf('autocmd BufReadCmd <buffer=%s> ++nested call s:reload()', bufnr)

    return buffer
endfunction

function! s:clean(bufnr) abort
    if !has_key(s:buffers, a:bufnr)
        return
    endif
    call remove(s:buffers, a:bufnr)
endfunction

function! s:reload() abort
    Kiview
    setlocal filetype=kiview
endfunction
