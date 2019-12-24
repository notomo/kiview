
let s:namespace = nvim_create_namespace('kiview')
let s:hl_namespace = nvim_create_namespace('kiview_highlight')
let s:group_hl_namespace = nvim_create_namespace('kiview_group_highlight')

function! kiview#current#new(bufnr) abort
    let current = {
        \ 'path': getcwd(),
        \ 'line_number': 2,
        \ 'target': v:null,
        \ 'targets': [],
        \ 'selected_targets': [],
        \ 'next_sibling_line_number': 1,
        \ 'parent_line_number': 1,
        \ 'last_sibling_line_number': 1,
        \ 'bufnr': a:bufnr,
        \ 'created': v:false,
        \ 'props': {},
        \ 'selected': {},
        \ 'logger': kiview#logger#new('current'),
    \ }

    function! current.set(path) abort
        let self.path = a:path

        let current = win_getid()
        for id in win_findbuf(self.bufnr)
            call nvim_set_current_win(id)
            execute 'lcd' a:path
        endfor
        call nvim_set_current_win(current)

        call self.logger.log('set cwd: ' . self.path)
    endfunction

    function! current.set_cursor(line_number) abort
        call setpos('.', [self.bufnr, a:line_number, 1, 0])
    endfunction

    function! current.update(range) abort
        let self.line_number = line('.')
        let depth = self._get_depth(self.line_number)
        let self.next_sibling_line_number = self._get_next_sibling_line_number(self.line_number, depth)
        let self.parent_line_number = self._get_parent_line_number(self.line_number, depth)
        let self.last_sibling_line_number = self._get_last_sibling_line_number(self.line_number, depth)
        let self.created = v:true

        let self.target = self._get_target(self.line_number)
        let self.targets = self._get_targets(a:range[0], a:range[1])
        let self.selected_targets = self._get_selected_targets()

        call self.logger.label('range').log(a:range)
    endfunction

    function! current._get_depth(line_number) abort
        let marks = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:line_number - 1, 0], [a:line_number - 1, 0], {})
        return self.props[marks[0][0]].depth
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

    function! current._get_parent_line_number(line_number, depth) abort
        let first_line_number = 1
        for line_number in range(a:line_number, first_line_number, -1)
            let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [line_number - 1, 0], [line_number - 1, 0], {})
            if empty(mark_ids)
                continue
            endif

            let depth = self.props[mark_ids[0][0]].depth
            if depth < a:depth
                return line_number
            endif
        endfor
        return first_line_number
    endfunction

    function! current._get_last_sibling_line_number(line_number, depth) abort
        let last_line_number = line('$')
        for line_number in range(a:line_number, last_line_number)
            let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [line_number - 1, 0], [line_number - 1, 0], {})
            if empty(mark_ids)
                continue
            endif

            let depth = self.props[mark_ids[0][0]].depth
            if depth < a:depth
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
            if has_key(self.selected, id)
                call remove(self.selected, id)
            endif
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

        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:line_number -1 , 0], [a:line_number - 1, 0], {})
        if !empty(mark_ids) && has_key(self.selected, mark_ids[0][0])
            return
        endif

        if prop.opened
            call nvim_buf_add_highlight(self.bufnr, s:group_hl_namespace, 'KiviewNodeOpen', a:line_number - 1, 0, -1)
        else
            call nvim_buf_clear_namespace(self.bufnr, s:group_hl_namespace, a:line_number - 1, a:line_number)
        endif
    endfunction

    function! current.toggle_selection(ids) abort
        for mark_id in a:ids
            if !has_key(self.props, mark_id)
                throw 'invalid mark id: ' . mark_id
            endif

            let [row, _] = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, mark_id)
            if has_key(self.selected, mark_id)
                call nvim_buf_clear_namespace(self.bufnr, s:hl_namespace, row, row + 1)
                call remove(self.selected, mark_id)
                continue
            endif
            call nvim_buf_add_highlight(self.bufnr, s:hl_namespace, 'KiviewSelected', row, 0, -1)
            let self.selected[mark_id] = v:true
        endfor
    endfunction

    function! current.set_props(props, start) abort
        let line_number = a:start - 1
        for prop in a:props
            let id = nvim_buf_set_extmark(self.bufnr, s:namespace, 0, line_number, 0, {})
            let self.props[id] = prop
            let line_number += 1
        endfor
    endfunction

    function! current.clear_selection() abort
        call nvim_buf_clear_namespace(self.bufnr, s:hl_namespace, 0, -1)
        let self.selected = {}
    endfunction

    function! current._get_target(line_number) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:line_number - 1, 0], [a:line_number - 1, 0], {})
        for [id, _, _] in mark_ids
            let prop = copy(self.props[id])
            return self._to_target(id, prop)
        endfor
    endfunction

    function! current._get_targets(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start - 1, 0], [a:end - 1, 0], {})
        let targets = []
        for [id, _, _] in mark_ids
            let prop = self.props[id]
            call add(targets, self._to_target(id, prop))
        endfor
        return targets
    endfunction

    function! current._get_selected_targets() abort
        let mark_ids = keys(self.selected)
        let targets = []
        for id in mark_ids
            let prop = self.props[id]
            call add(targets, self._to_target(id, prop))
        endfor
        return targets
    endfunction

    function! current._to_target(id, prop) abort
        return {
            \ 'id': str2nr(a:id),
            \ 'path': a:prop.path,
            \ 'is_parent_node': a:prop.is_parent_node,
            \ 'depth': a:prop.depth,
            \ 'opened': has_key(a:prop, 'opened') ? a:prop.opened : v:false,
        \ }
    endfunction

    return current
endfunction
