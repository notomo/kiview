
let s:namespace = nvim_create_namespace('kiview')

function! kiview#current#new(bufnr) abort
    let current = {
        \ 'path': getcwd(),
        \ 'line_number': 1,
        \ 'target': v:null,
        \ 'targets': [],
        \ 'next_sibling_line_number': 1,
        \ 'depth': 0,
        \ 'bufnr': a:bufnr,
        \ 'created': v:false,
        \ 'props': {},
        \ 'logger': kiview#logger#new('current'),
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
        let self.created = v:true

        let self.target = self._get_targets(self.line_number - 1, self.line_number - 1)[0]
        let self.targets = self._get_targets(a:range[0] - 1, a:range[1] - 1)
    endfunction

    function! current.delete_marks(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start, 0], [a:end, 0], {})
        for [id, _, _] in mark_ids
            call nvim_buf_del_extmark(self.bufnr, s:namespace, id)
            call remove(self.props, id)
        endfor
    endfunction

    function! current.set_props(props, start) abort
        let line_number = a:start
        for prop in a:props
            let id = nvim_buf_set_extmark(self.bufnr, s:namespace, 0, line_number, 0, {})
            let self.props[id] = prop
            let line_number += 1
        endfor
    endfunction

    function! current._get_targets(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start, 0], [a:end, 0], {})
        let targets = []
        for [id, _, _] in mark_ids
            let prop = self.props[id]
            call add(targets, prop.path)
        endfor
        return targets
    endfunction

    return current
endfunction
