
let s:namespace = nvim_create_namespace('kiview')
let s:id = get(s:, 'id', 0)
let s:width = 75

function! kiview#renamer#new(source_id, bufnr) abort
    let renamer = {
        \ 'bufnr': v:null,
        \ 'window': v:null,
        \ 'source_id': a:source_id,
        \ 'source_bufnr': a:bufnr,
        \ 'items': [],
        \ 'targets': [],
        \ 'logger': kiview#logger#new('renamer'),
    \ }

    function! renamer.open(path, items) abort
        let self.bufnr = nvim_create_buf(v:false, v:true)
        let self.items = a:items

        let s:id += 1
        let name = printf('kiview://%s/%s/kiview-rename', self.source_id, s:id)
        call nvim_buf_set_name(self.bufnr, name)

        let lines = [''] + map(copy(a:items), { _, v -> v.relative_path })
        call nvim_buf_set_lines(self.bufnr, 0, -1, v:true, lines)

        call nvim_buf_set_virtual_text(self.bufnr, s:namespace, 0, [[a:path, 'Comment']], {})
        let line = 1
        for item in a:items
            call nvim_buf_set_extmark(self.bufnr, s:namespace, 0, line, 0, {})
            call nvim_buf_set_virtual_text(self.bufnr, s:namespace, line, [[' <- ' . item.relative_path, 'Comment']], {})
            let line += 1
        endfor

        let height = len(a:items)
        let max_width = max(map(copy(lines), { _, v -> strlen(v) }))
        let width = max_width > s:width ? max_width : s:width
        let self.window = nvim_open_win(self.bufnr, v:true, {
            \ 'relative': 'editor',
            \ 'width': width,
            \ 'height': len(a:items) + 2,
            \ 'row': &lines / 2 - (height / 2),
            \ 'col': &columns / 2 - (width / 2),
            \ 'anchor': 'NW',
            \ 'focusable': v:true,
            \ 'external': v:false,
            \ 'style': 'minimal',
        \ })
        execute 'lcd' a:path

        setlocal filetype=kiview-rename
        call nvim_buf_set_option(self.bufnr, 'bufhidden', 'wipe')
        call nvim_buf_set_option(self.bufnr, 'buftype', 'acwrite')
        call nvim_buf_set_option(self.bufnr, 'modified', v:false)
        call nvim_win_set_cursor(self.window, [2, 0])

        execute printf('autocmd BufWriteCmd <buffer=%s> ++nested call s:write(%s)', self.bufnr, self.source_bufnr)
    endfunction

    function! renamer.write() abort
        let targets = []
        let lines = getbufline(self.bufnr, 2, '$')
        let i = 0
        for line in lines
            let item = self.items[i]
            let i += 1
            let marks = nvim_buf_get_extmarks(self.bufnr, s:namespace, [i , 0], [i, -1], {})
            if empty(marks)
                continue
            endif
            if item.relative_path ==? line
                continue
            endif
            call add(targets, {'from': item.path, 'to': line, 'id': marks[0][0]})
        endfor
        let self.targets = targets
        call kiview#main('multiple_rename', {'bufnr': self.source_bufnr, 'range': [1, 1]})
        let self.targets = []
    endfunction

    function! renamer.complete(items) abort
        for item in a:items
            let mark = nvim_buf_get_extmark_by_id(self.bufnr, s:namespace, item.id)
            if empty(mark)
                continue
            endif
            let [line, col] = mark
            let self.items[line - 1] = item
            call nvim_buf_set_virtual_text(self.bufnr, s:namespace, line, [[' <- ' . item.relative_path, 'Comment']], {})
        endfor
        call nvim_buf_set_option(self.bufnr, 'modified', v:false)
    endfunction

    function! renamer.opened() abort
        return !empty(self.window) && nvim_win_is_valid(self.window) ? v:true : v:false
    endfunction

    return renamer
endfunction

function! s:write(source_bufnr) abort
    let buffer = kiview#buffer#find(a:source_bufnr)
    if empty(buffer)
        return
    endif
    call buffer.renamer.write()
endfunction
