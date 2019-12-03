
let s:default_line_number = 2

function! kiview#history#new(bufnr) abort
    let history = {
        \ 'line_numbers': {},
        \ 'bufnr': a:bufnr,
        \ 'logger': kiview#logger#new('history'),
    \ }

    function! history.add(path, line_number) abort
        let self.line_numbers[a:path] = a:line_number
    endfunction

    function! history.restore(path) abort
        if empty(win_findbuf(self.bufnr))
            return
        endif

        if !has_key(self.line_numbers, a:path)
            call setpos('.', [self.bufnr, s:default_line_number, 1, 0])
            return
        endif

        let line_number = self.line_numbers[a:path]
        call setpos('.', [self.bufnr, line_number, 1, 0])

        call self.logger.logf('restore: path=%s, line=%s', a:path, line_number)
    endfunction

    return history
endfunction
