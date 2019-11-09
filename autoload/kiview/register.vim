
let s:registers = {}

function! kiview#register#new(bufnr) abort
    if has_key(s:registers, a:bufnr)
        return s:registers[a:bufnr]
    endif

    let register = {
        \ 'paths': [],
        \ 'has_cut': v:false,
        \ 'logger': kiview#logger#new('register'),
    \ }

    function! register.cut(paths) abort
        let self.paths = a:paths
        let self.has_cut = v:true

        call self.logger.log('cut: ' . string(self.paths))
    endfunction

    function! register.copy(paths) abort
        let self.paths = a:paths
        let self.has_cut = v:false

        call self.logger.log('copy: ' . string(self.paths))
    endfunction

    function! register.clear() abort
        call self.copy([])

        call self.logger.log('clear')
    endfunction

    let s:registers[a:bufnr] = register
    execute printf('autocmd BufWipeout <buffer=%s> ++nested call s:clean("%s")', a:bufnr, a:bufnr)

    return register
endfunction

function! s:clean(bufnr) abort
    if !has_key(s:registers, a:bufnr)
        return
    endif
    call remove(s:registers, a:bufnr)
endfunction
