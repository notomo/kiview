
let s:currents = {}

function! kiview#current#new(bufnr, range) abort
    if has_key(s:currents, a:bufnr)
        let current = s:currents[a:bufnr]
        let current.line_number = line('.')
        let pattern = '^' . repeat(' ', indent(current.line_number)) . '\S'
        let current.next_sibling_line_number = search(pattern, 'nW')
        let current.depth = indent(current.line_number)

        let current.target = s:get_target(current.line_number)
        let current.targets = map(range(a:range[0], a:range[1]), { _, line_number -> s:get_target(line_number) })
        return current
    endif

    let current = {
        \ 'path': getcwd(),
        \ 'line_number': 1,
        \ 'target': v:null,
        \ 'targets': [],
        \ 'next_sibling_line_number': 1,
        \ 'depth': 0,
    \ }

    function! current.set(path) abort
        let self.path = a:path
    endfunction

    let s:currents[a:bufnr] = current
    execute printf('autocmd BufWipeout <buffer=%s> call s:clean("%s")', a:bufnr, a:bufnr)

    return current
endfunction

function! s:clean(bufnr) abort
    if !has_key(s:currents, a:bufnr)
        return
    endif
    call remove(s:currents, a:bufnr)
endfunction

function! s:get_target(line_number) abort
    let line_number = a:line_number
    let indent = indent(line_number)
    let target = getline(line_number)[indent :]
    while line_number > 0
        if indent == 0
            break
        endif

        let line_number = line_number - 1
        let prev_indent = indent(line_number)
        if prev_indent < indent
            let target = getline(line_number)[prev_indent :] . target
            let indent = prev_indent
        endif
    endwhile

    return target
endfunction
