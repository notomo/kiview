
function! s:new(bufnr, current_path, targets) abort
    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'current_path': a:current_path,
        \ 'targets': a:targets,
        \ 'logger': kiview#logger#new().label('buffer'),
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
        call nvim_buf_set_lines(self.bufnr, 0, length, v:false, a:lines)

        let lines = nvim_buf_get_lines(self.bufnr, 0, nvim_buf_line_count(self.bufnr), v:false)
        call self.logger.logs(lines)
    endfunction

    function! buffer.set(options) abort
        call nvim_buf_set_var(self.bufnr, 'kiview_options', a:options)
        call self.logger.log('options: ' . string(a:options))
    endfunction

    function! buffer.close_windows() abort
        let window_ids = win_findbuf(self.bufnr)
        for id in window_ids
            call nvim_win_close(id, v:false)
        endfor
    endfunction

    return buffer
endfunction

function! kiview#buffer#find() abort
    if &filetype !=? 'kiview'
        let bufnr = nvim_create_buf(v:false, v:true)
        let current_path = getcwd()
        let targets = []
        return s:new(bufnr, current_path, targets)
    endif

    let bufnr = bufnr('%')
    let options = get(b:, 'kiview_options', {})
    let current_path = get(options, 'current_path', getcwd())
    let targets = [getline(line('.'))]
    return s:new(bufnr, current_path, targets)
endfunction
