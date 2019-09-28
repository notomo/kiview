
let s:currents = {}

function! kiview#current#new(bufnr, range) abort
    if has_key(s:currents, a:bufnr)
        let current = s:currents[a:bufnr]
        let current.line_number = line('.')
        let current.target = getline(current.line_number)
        let current.targets = getbufline(a:bufnr, a:range[0], a:range[1])
        return current
    endif

    let current = {
        \ 'path': getcwd(),
        \ 'line_number': 1,
        \ 'target': v:null,
        \ 'targets': [],
    \ }

    function! current.set(path) abort
        let self.path = a:path
    endfunction

    let s:currents[a:bufnr] = current

    return current
endfunction
