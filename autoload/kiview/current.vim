
let s:namespace = nvim_create_namespace('kiview')

function! kiview#current#new(bufnr) abort
    let current = {
        \ 'path': getcwd(),
        \ 'line_number': 1,
        \ 'target': {'path': v:null},
        \ 'targets': [],
        \ 'next_sibling_line_number': 1,
        \ 'depth': 0,
        \ 'bufnr': a:bufnr,
        \ 'created': v:false,
        \ 'opened': v:false,
        \ 'props': {},
        \ 'logger': kiview#logger#new('current'),
    \ }

    function! current.set(path) abort
        let self.path = a:path
    endfunction

    function! current.set_cursor(line_number) abort
        call setpos('.', [self.bufnr, a:line_number, 1, 0])
    endfunction

    function! current.update(range) abort
        let self.line_number = line('.')
        let self.depth = indent(self.line_number)
        let self.next_sibling_line_number = self._get_next_sibling_line_number(self.line_number, self.depth)
        let self.created = v:true

        let self.target = self._get_target(self.line_number)
        let self.targets = self._get_targets(a:range[0], a:range[1])
    endfunction

    function! current._get_next_sibling_line_number(line_number, depth) abort
        let last_line_number = line('$')
        for line_number in range(a:line_number + 1, last_line_number)
            let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [line_number - 1, 0], [line_number - 1, 0], {})
            if empty(mark_ids)
                continue
            endif

            let depth = self.props[mark_ids[0][0]].depth
            if depth == a:depth
                return line_number
            elseif depth < a:depth
                return line_number
            endif
        endfor
        return last_line_number + 1
    endfunction

    function! current.unset_props(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start - 1, 0], [a:end - 1, 0], {})
        for [id, _, _] in mark_ids
            call nvim_buf_del_extmark(self.bufnr, s:namespace, id)
            call remove(self.props, id)
        endfor
    endfunction

    function! current.unset_props_all() abort
        call self.unset_props(0, 1)
    endfunction

    function! current.toggle_tree(line_number, opened) abort
        let index = a:line_number - 1
        let mark_id = nvim_buf_get_extmarks(self.bufnr, s:namespace, [index, 0], [index, 0], {})[0][0]
        let prop = self.props[mark_id]
        let prop.opened = a:opened
    endfunction

    function! current.set_props(props, start) abort
        let line_number = a:start - 1
        for prop in a:props
            let id = nvim_buf_set_extmark(self.bufnr, s:namespace, 0, line_number, 0, {})
            let self.props[id] = prop
            let line_number += 1
        endfor
    endfunction

    function! current._get_target(line_number) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:line_number -1 , 0], [a:line_number - 1, 0], {})
        for [id, _, _] in mark_ids
            return copy(self.props[id])
        endfor
    endfunction

    function! current._get_targets(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start -1 , 0], [a:end - 1, 0], {})
        let targets = []
        for [id, _, _] in mark_ids
            let prop = self.props[id]
            call add(targets, prop.path)
        endfor
        return targets
    endfunction

    return current
endfunction
