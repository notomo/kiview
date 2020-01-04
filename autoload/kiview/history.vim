
let s:default_line_number = 2

function! kiview#history#new(bufnr) abort
    let history = {
        \ 'line_numbers': {},
        \ 'paths': [],
        \ 'bufnr': a:bufnr,
        \ 'logger': kiview#logger#new('history'),
    \ }

    function! history.copy(history) abort
        let self.line_numbers = copy(a:history.line_numbers)
        let self.paths = copy(a:history.paths)
    endfunction

    function! history.add(path, line_number, is_back) abort
        let self.line_numbers[a:path] = a:line_number
        if !a:is_back
            call insert(self.paths, a:path, 0)
            call self.logger.logf('add: path=%s', a:path)
        endif
    endfunction

    function! history.restore_cursor(path) abort
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

    function! history.back() abort
        if empty(self.paths)
            return v:null
        endif

        let path = remove(self.paths, 0)
        call self.logger.logf('back: path=%s', path)
        return path
    endfunction

    return history
endfunction
