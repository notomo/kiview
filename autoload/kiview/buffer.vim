
function! s:new(bufnr, current_path, line_number, current_target, targets) abort
    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'current_path': a:current_path,
        \ 'line_number': a:line_number,
        \ 'current_target': a:current_target,
        \ 'targets': a:targets,
        \ 'logger': kiview#logger#new().label('buffer'),
    \ }

    function! buffer.open() abort
        leftabove vsplit
        execute 'buffer' self.bufnr

        setlocal filetype=kiview
        setlocal nonumber
        call nvim_win_set_width(win_getid(), 38)

        syntax match KiviewNode ".*\/$"

        call self.logger.log('opend bufnr: ' . self.bufnr)
    endfunction

    function! buffer.write(lines) abort
        let length = nvim_buf_line_count(self.bufnr)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:true)
        call nvim_buf_set_lines(self.bufnr, 0, length, v:false, a:lines)
        call nvim_buf_set_option(self.bufnr, 'modifiable', v:false)

        let lines = nvim_buf_get_lines(self.bufnr, 0, nvim_buf_line_count(self.bufnr), v:false)
        call self.logger.logs(lines)
    endfunction

    function! buffer.set(options) abort
        let options = get(b:, 'kiview_options', {})
        let history = get(options, 'history', {})
        let history[a:options['last_path']] = a:options['last_line_number']
        let options['history'] = history
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

    function! buffer.restore_cursor(options) abort
        let options = get(b:, 'kiview_options', {})
        let history = get(options, 'history', {})

        let last_path_line_number = get(a:options, 'last_path_line_number', 0)
        if !empty(last_path_line_number)
            call setpos('.', [self.bufnr, last_path_line_number, 1, 0])
            return
        endif

        let current_path = get(a:options, 'current_path', '')
        if !has_key(history, current_path)
            call self.logger.log('could not restore line number: ' . current_path)
            return
        endif

        let line_number = history[current_path]
        call setpos('.', [self.bufnr, line_number, 1, 0])
        call self.logger.log('restored line number: ' . current_path . ': ' . line_number)
    endfunction

    return buffer
endfunction

function! kiview#buffer#find() abort
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
    let targets = [current_target]
    return s:new(bufnr, current_path, line_number, current_target, targets)
endfunction
