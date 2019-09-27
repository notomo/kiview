
function! s:new(bufnr, current_path, line_number, current_target, targets) abort
    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'current_path': a:current_path,
        \ 'line_number': a:line_number,
        \ 'current_target': a:current_target,
        \ 'targets': a:targets,
        \ 'logger': kiview#logger#new('buffer'),
        \ 'register': kiview#register#new(a:bufnr),
        \ 'history': kiview#history#new(a:bufnr),
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
        let length = nvim_buf_line_count(self.bufnr)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, 0, length, v:false, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        call self.logger.buffer_log(self.bufnr)
    endfunction

    function! buffer.set(options) abort
        let options = get(b:, 'kiview_options', {})
        let options['current_path'] = a:options['current_path']

        call nvim_buf_set_var(self.bufnr, 'kiview_options', options)
        call self.logger.log('options: ' . string(options))
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
        let current_path = getcwd()
        let line_number = 1
        let current_target = v:null
        let targets = []
        return s:new(bufnr, current_path, line_number, current_target, targets)
    endif

    let bufnr = bufnr('%')
    let options = get(b:, 'kiview_options', {})
    let current_path = get(options, 'current_path', getcwd())
    let line_number = line('.')
    let current_target = getline(line_number)
    let targets = getbufline('%', a:range[0], a:range[1])
    return s:new(bufnr, current_path, line_number, current_target, targets)
endfunction
