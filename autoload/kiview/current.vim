
function! kiview#current#new(bufnr) abort
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

    function! current.update(range) abort
        let self.line_number = line('.')
        let pattern = '^' . repeat(' ', indent(self.line_number)) . '\S'
        let self.next_sibling_line_number = search(pattern, 'nW')
        if self.next_sibling_line_number == 0
            let self.next_sibling_line_number = self.line_number + 1
        endif
        let self.depth = indent(self.line_number)

        let self.target = s:get_target(self.line_number)
        let self.targets = map(range(a:range[0], a:range[1]), { _, line_number -> s:get_target(line_number) })
    endfunction

    return current
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
