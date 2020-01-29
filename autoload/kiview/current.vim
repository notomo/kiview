
let s:namespace = nvim_create_namespace('kiview')
let s:hl_namespace = nvim_create_namespace('kiview_highlight')
let s:group_hl_namespace = nvim_create_namespace('kiview_group_highlight')

function! kiview#current#new(bufnr) abort
    let current = {
        \ 'bufnr': a:bufnr,
        \ 'props': {},
        \ 'selected': {},
        \ 'logger': kiview#logger#new('current'),
    \ }

    function! current.set(path) abort
        let current = win_getid()
        for id in win_findbuf(self.bufnr)
            call nvim_set_current_win(id)
            execute 'lcd' a:path
        endfor
        call nvim_set_current_win(current)

        call self.logger.log('set cwd: ' . a:path)
    endfunction

    function! current.set_cursor(line_number) abort
        call setpos('.', [self.bufnr, a:line_number, 1, 0])
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

    function! current.toggle_tree(id, opened) abort
        call self.logger.logf('id: %s, opened: %s', a:id, a:opened)
        let [index, _] = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, a:id)
        let line_number = index + 1
        let prop = self.props[a:id]
        let prop.opened = a:opened

        if has_key(self.selected, a:id)
            return line_number
        endif

        if prop.opened
            call nvim_buf_add_highlight(self.bufnr, s:group_hl_namespace, 'KiviewNodeOpen', index, 0, -1)
        else
            call nvim_buf_clear_namespace(self.bufnr, s:group_hl_namespace, index, index + 1)
        endif
        return line_number
    endfunction

    function! current._select(mark_id) abort
        if !has_key(self.props, a:mark_id)
            throw 'invalid mark id: ' . a:mark_id
        endif

        if has_key(self.selected, a:mark_id)
            return
        endif

        let [row, _] = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, a:mark_id)
        call nvim_buf_add_highlight(self.bufnr, s:hl_namespace, 'KiviewSelected', row, 0, -1)
        let self.selected[a:mark_id] = v:true
    endfunction

    function! current.select(ids) abort
        for mark_id in a:ids
            call self._select(mark_id)
        endfor
    endfunction

    function! current._unselect(mark_id) abort
        if !has_key(self.props, a:mark_id)
            throw 'invalid mark id: ' . a:mark_id
        endif

        if !has_key(self.selected, a:mark_id)
            return
        endif

        let [row, _] = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, a:mark_id)
        call nvim_buf_clear_namespace(self.bufnr, s:hl_namespace, row, row + 1)
        call remove(self.selected, a:mark_id)
        let prop = self.props[a:mark_id]
        if has_key(prop, 'opened') && prop.opened
            call nvim_buf_add_highlight(self.bufnr, s:group_hl_namespace, 'KiviewNodeOpen', row, 0, -1)
        endif
    endfunction

    function! current.unselect(ids) abort
        for mark_id in a:ids
            call self._unselect(mark_id)
        endfor
    endfunction

    function! current.toggle_selection(ids) abort
        for mark_id in a:ids
            if !has_key(self.props, mark_id)
                throw 'invalid mark id: ' . mark_id
            endif

            if has_key(self.selected, mark_id)
                call self._unselect(mark_id)
                continue
            endif
            call self._select(mark_id)
        endfor
    endfunction

    function! current.toggle_all_selection() abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [1, 0], [-1, 0], {})
        call self.toggle_selection(map(mark_ids, { _, v -> v[0] }))
    endfunction

    function! current.select_all() abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [1, 0], [-1, 0], {})
        call self.select(map(mark_ids, { _, v -> v[0] }))
    endfunction

    function! current.to_line_number(id, default) abort
        if empty(a:id)
            return a:default
        endif
        let [index, _] = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, a:id)
        return index + 1
    endfunction

    function! current.set_props(props, start) abort
        if empty(a:props)
            call self.logger.label('line').buffer_log(self.bufnr, s:namespace, self.props)
            return
        endif

        let line_number = a:start - 1
        let pairs = []
        for prop in a:props
            let id = nvim_buf_set_extmark(self.bufnr, s:namespace, 0, line_number, 0, {})
            call add(pairs, [id, prop])
            let self.props[id] = prop
            let line_number += 1
        endfor
        let last_sibling_id = id

        let [_, prev_prop] = pairs[0]
        for [id, prop] in pairs[1:]
            let prev_prop.next_sibling_id = id
            let prev_prop = prop
        endfor

        for [id, prop] in pairs
            if prop.is_parent_node
                call self.logger.label('line').buffer_log(self.bufnr, s:namespace, self.props)
                return
            endif
            let prop.last_sibling_id = last_sibling_id
        endfor

        call self.logger.label('line').buffer_log(self.bufnr, s:namespace, self.props)
    endfunction

    function! current.clear_selection() abort
        if !nvim_buf_is_valid(self.bufnr)
            return
        endif
        call nvim_buf_clear_namespace(self.bufnr, s:hl_namespace, 0, -1)
        let self.selected = {}
    endfunction

    function! current.get_target(line_number) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:line_number - 1, 0], [a:line_number - 1, 0], {})
        for [id, _, _] in mark_ids
            let prop = copy(self.props[id])
            return self._to_target(id, prop)
        endfor
    endfunction

    function! current.get_targets(start, end) abort
        let mark_ids = nvim_buf_get_extmarks(self.bufnr, s:namespace, [a:start - 1, 0], [a:end - 1, 0], {})
        let targets = []
        for [id, _, _] in mark_ids
            let prop = self.props[id]
            call add(targets, self._to_target(id, prop))
        endfor
        return targets
    endfunction

    function! current.get_selected_targets() abort
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
            \ 'parent_id': has_key(a:prop, 'parent_id') ? a:prop.parent_id : v:null,
            \ 'last_sibling_id': has_key(a:prop, 'last_sibling_id') ? a:prop.last_sibling_id : v:null,
            \ 'next_sibling_id': has_key(a:prop, 'next_sibling_id') ? a:prop.next_sibling_id : v:null,
        \ }
    endfunction

    return current
endfunction
