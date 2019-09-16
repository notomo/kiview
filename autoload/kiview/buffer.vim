
function! s:new(bufnr, options) abort
    let buffer = {
        \ 'bufnr': a:bufnr,
        \ 'logger': kiview#logger#new().label('buffer'),
        \ 'options': a:options,
    \ }

    function! buffer.open() abort
        leftabove vsplit
        execute 'buffer' self.bufnr

        setlocal filetype=kiview
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

    return buffer
endfunction

function! kiview#buffer#new() abort
    let bufnr = nvim_create_buf(v:false, v:true)
    return s:new(bufnr, {})
endfunction

function! kiview#buffer#from_buffer() abort
    let bufnr = bufnr('%')
    let options = get(b:, 'kiview_options', {})
    let options['target'] = getline(line('.'))
    return s:new(bufnr, options)
endfunction
