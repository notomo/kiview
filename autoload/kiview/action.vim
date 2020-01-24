
function! kiview#action#new_handler(buffer, arg) abort
    let buffer = a:buffer
    let arg = a:arg
    let input_reader = kiview#input_reader#new()
    let handler = {
        \ 'funcs': {
            \ 'open': { action -> s:open_targets(action, buffer) },
            \ 'tab_open': { action -> s:tab_open_targets(action, buffer) },
            \ 'vertical_open': { action -> s:vertical_open_targets(action, buffer) },
            \ 'horizontal_open': { action -> s:horizontal_open_targets(action, buffer) },
            \ 'open_leaves': { action -> s:open_leaves(action, buffer) },
            \ 'open_view': { action -> s:open_view(action, buffer) },
            \ 'add_history': { action -> s:add_history(action, buffer) },
            \ 'back_history': { action -> s:back_history(action, buffer) },
            \ 'try_to_restore_cursor': { action -> s:try_to_restore_cursor(action, buffer) },
            \ 'set_cursor': { action -> s:set_cursor(action, buffer) },
            \ 'set_path': { action -> s:set_path(action, buffer) },
            \ 'write_all': { action -> s:write_all(action, buffer) },
            \ 'write': { action -> s:write(action, buffer) },
            \ 'open_tree': { action -> s:open_tree(action, buffer) },
            \ 'close_tree': { action -> s:close_tree(action, buffer) },
            \ 'quit': { action -> s:quit(buffer) },
            \ 'confirm_new': { action -> s:confirm_new(input_reader, arg) },
            \ 'confirm_remove': { action -> s:confirm_remove(action, input_reader) },
            \ 'confirm_rename': { action -> s:confirm_rename(action, input_reader) },
            \ 'open_renamer': { action -> s:open_renamer(action, buffer) },
            \ 'complete_renamer': { action -> s:complete_renamer(action, buffer) },
            \ 'copy': { action -> s:copy(action, buffer) },
            \ 'cut': { action -> s:cut(action, buffer) },
            \ 'clear_register': { action -> s:clear_register(buffer) },
            \ 'toggle_selection': { action -> s:toggle_selection(action, buffer) },
            \ 'toggle_all_selection': { action -> s:toggle_all_selection(action, buffer) },
            \ 'show_error': { action -> s:show_error(action) },
            \ 'fork_buffer': { action -> s:fork_buffer(action, buffer) },
            \ 'choose': { action -> s:choose(action, buffer, input_reader) },
        \ },
    \ }

    function! handler.handle(action) abort
        if !has_key(self.funcs, a:action.name)
            return
        endif

        return self.funcs[a:action.name](a:action)
    endfunction

    return handler
endfunction

let s:split_names = {
    \ 'tab': 'tabedit',
    \ 'vertical': 'vsplit',
    \ 'horizontal': 'split',
\ }

function! s:open_leaves(action, buffer) abort
    let split = s:split_names[a:action.split_name]
    for path in a:action.paths
        execute a:action.mod_name split path
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:open_targets(action, buffer) abort
    wincmd w
    for path in a:action.paths
        execute 'edit' path
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:tab_open_targets(action, buffer) abort
    for path in a:action.paths
        execute 'tabedit' path
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:vertical_open_targets(action, buffer) abort
    wincmd w
    for path in a:action.paths
        execute 'vsplit' path
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:horizontal_open_targets(action, buffer) abort
    wincmd w
    for path in a:action.paths
        execute 'split' path
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:write_all(action, buffer) abort
    call a:buffer.current.unset_props_all()
    call a:buffer.write_all(a:action.lines)
    call a:buffer.current.set_props(a:action.props, 1)
    call a:buffer.current.clear_selection()
endfunction

function! s:write(action, buffer) abort
    let start = a:buffer.current.to_line_number(a:action.parent_id, 1) + 1
    let end = a:buffer.current.to_line_number(a:action.last_sibling_id, -1) + 1
    call a:buffer.current.unset_props(start, end)
    call a:buffer.write(a:action.lines, start, end)
    call a:buffer.current.set_props(a:action.props, start)
    call a:buffer.current.clear_selection()
endfunction

function! s:open_tree(action, buffer) abort
    let start = a:buffer.current.toggle_tree(a:action.id, v:true) + 1
    if a:action.count == 0
        return
    endif
    call a:buffer.write(a:action.lines, start, start)
    call a:buffer.current.set_props(a:action.props, start)
endfunction

function! s:close_tree(action, buffer) abort
    let line_number = a:buffer.current.toggle_tree(a:action.id, v:false)
    let next_sibling_line_number = a:buffer.current.to_line_number(a:action.next_sibling_id, 0)
    let line_count = next_sibling_line_number - line_number - 1
    if line_count == 0
        return
    endif
    let start = line_number + 1
    let end = line_number + line_count
    call a:buffer.current.unset_props(start, end)
    call a:buffer.write([], start, end + 1)
endfunction

function! s:try_to_restore_cursor(action, buffer) abort
    call a:buffer.history.restore_cursor(a:action.path)
    call a:buffer.current.set(a:action.path)
endfunction

function! s:set_path(action, buffer) abort
    call a:buffer.current.set(a:action.path)
endfunction

function! s:set_cursor(action, buffer) abort
    call a:buffer.current.set_cursor(a:action.line_number)
endfunction

function! s:back_history(action, buffer) abort
    let path = a:buffer.history.back()
    if empty(path)
        return
    endif
    return 'go -back -path=' . path
endfunction

function! s:add_history(action, buffer) abort
    call a:buffer.history.add(a:action.path, a:action.line_number, a:action.back)
endfunction

function! s:open_view(action, buffer) abort
    call a:buffer.open(a:action.split_name, a:action.mod_name)
    call a:buffer.history.restore_cursor('')
    call a:buffer.current.set(a:action.path)
endfunction

function! s:quit(buffer) abort
    call a:buffer.close_windows()
endfunction

function! s:confirm_new(input_reader, arg) abort
    let name = a:input_reader.read('new: ', [])
    if empty(name)
        return
    endif
    let paths = map(split(name, '\v\s+'), {_, v -> '-paths=' . v})
    return a:arg . ' ' . join(paths, ' ')
endfunction

function! s:confirm_remove(action, input_reader) abort
    let answer = a:input_reader.read('remove? Y/n: ', a:action.paths)
    if empty(answer) || answer !=? 'Y'
        return
    endif
    return 'remove -no-confirm'
endfunction

function! s:confirm_rename(action, input_reader) abort
    let name = a:input_reader.read('rename to: ', [a:action.path], a:action.relative_path)
    if empty(name)
        return
    endif
    return 'rename -no-confirm -path=' . name
endfunction

function! s:choose(action, buffer, input_reader) abort
    let targets = []
    let rename_targets = []
    for item in a:action.items
        let answer = a:input_reader.read('already exists (f)orce (n)o (r)ename: ', [item.path])
        if answer ==? 'n'
            continue
        elseif answer ==? 'r'
            let item.force = v:false
            call add(rename_targets, item)
            continue
        elseif answer ==? 'f'
            let item.force = v:true
            call add(targets, item)
        endif
    endfor

    if empty(targets) && empty(rename_targets)
        return
    elseif empty(targets) && !empty(rename_targets)
        let is_copy = !a:action.has_cut ? v:true : v:false
        let items = map(rename_targets, { _, v -> {'path': v.from, 'relative_path': v.from, 'id': v.from, 'to': v.path, 'is_copy': is_copy}})
        call a:buffer.renamer.open(a:action.path, items)
        return
    endif

    for target in rename_targets
        let new_name = a:input_reader.read('rename to: ', [], target.relative_path)
        if empty(new_name)
            continue
        endif
        let target.new_name = new_name
        call add(targets, target)
    endfor

    if a:action.has_cut
        call a:buffer.register.cut(targets)
    else
        call a:buffer.register.copy(targets)
    endif

    return 'paste -no-confirm'
endfunction

function! s:copy(action, buffer) abort
    call a:buffer.register.copy(a:action.items)
    call a:buffer.current.clear_selection()
    call kiview#messenger#new().info('Copied: ', a:action.items)
endfunction

function! s:cut(action, buffer) abort
    call a:buffer.register.cut(a:action.items)
    call a:buffer.current.clear_selection()
    call kiview#messenger#new().info('Cut: ', a:action.items)
endfunction

function! s:clear_register(buffer) abort
    call a:buffer.register.clear()
endfunction

function! s:toggle_selection(action, buffer) abort
    call a:buffer.current.toggle_selection(a:action.ids)
endfunction

function! s:toggle_all_selection(action, buffer) abort
    call a:buffer.current.toggle_all_selection()
endfunction

function! s:show_error(action) abort
    let message = printf('on %s: %s', a:action.path, a:action.message)
    call kiview#messenger#new().error(message)
endfunction

function! s:fork_buffer(action, buffer) abort
    for item in a:action.items
        let new_buffer = kiview#buffer#new()
        call new_buffer.history.copy(a:buffer.history)

        call s:write_all(item, new_buffer)

        call new_buffer.open(a:action.split_name, a:action.mod_name)
        call new_buffer.current.set(item.path)
    endfor
    call a:buffer.current.clear_selection()
endfunction

function! s:open_renamer(action, buffer) abort
    call a:buffer.renamer.open(a:action.path, a:action.items)
endfunction

function! s:complete_renamer(action, buffer) abort
    call a:buffer.renamer.complete(a:action.items, a:action.has_error)
endfunction
