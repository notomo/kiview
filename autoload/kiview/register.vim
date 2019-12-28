
let s:register = v:null

function! kiview#register#new() abort
    if !empty(s:register)
        return s:register
    endif

    let register = {
        \ 'targets': [],
        \ 'has_cut': v:false,
        \ 'logger': kiview#logger#new('register'),
    \ }

    function! register.cut(targets) abort
        let self.targets = a:targets
        let self.has_cut = v:true

        call self.logger.logf('cut: %s', self.targets)
    endfunction

    function! register.copy(targets) abort
        let self.targets = a:targets
        let self.has_cut = v:false

        call self.logger.logf('copy: %s', self.targets)
    endfunction

    function! register.clear() abort
        call self.copy([])

        call self.logger.log('clear')
    endfunction

    let s:register = register

    return register
endfunction
