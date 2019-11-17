
function! kiview#register#new() abort
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

    return register
endfunction
