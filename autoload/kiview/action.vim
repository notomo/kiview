
function! kiview#action#new_handler(buffer) abort
    let buffer = a:buffer
    let input_reader = kiview#input_reader#new()
    let handler = {
        \ 'funcs': {
            \ 'open': { action -> s:open_targets(action, buffer) },
            \ 'tab_open': { action -> s:tab_open_targets(action, buffer) },
            \ 'vertical_open': { action -> s:vertical_open_targets(action, buffer) },
            \ 'horizontal_open': { action -> s:horizontal_open_targets(action, buffer) },
            \ 'create': { action -> s:create(action, buffer) },
            \ 'add_history': { action -> s:add_history(action, buffer) },
            \ 'try_to_restore_cursor': { action -> s:try_to_restore_cursor(action, buffer) },
            \ 'set_cursor': { action -> s:set_cursor(action, buffer) },
            \ 'set_path': { action -> s:set_path(action, buffer) },
            \ 'write_all': { action -> s:write_all(action, buffer) },
            \ 'write': { action -> s:write(action, buffer) },
            \ 'open_tree': { action -> s:open_tree(action, buffer) },
            \ 'close_tree': { action -> s:close_tree(action, buffer) },
            \ 'quit': { action -> s:quit(buffer) },
            \ 'confirm_new': { action -> s:confirm_new(input_reader) },
            \ 'confirm_remove': { action -> s:confirm_remove(action, input_reader) },
            \ 'confirm_rename': { action -> s:confirm_rename(action, input_reader) },
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
    call a:buffer.history.restore(a:action.path)
    call a:buffer.current.set(a:action.path)
endfunction

function! s:set_path(action, buffer) abort
    call a:buffer.current.set(a:action.path)
endfunction

function! s:set_cursor(action, buffer) abort
    call a:buffer.current.set_cursor(a:action.line_number)
endfunction

function! s:add_history(action, buffer) abort
    call a:buffer.history.add(a:action.path, a:action.line_number)
endfunction

function! s:create(action, buffer) abort
    call a:buffer.open(a:action.split_name, a:action.mod_name)
    call a:buffer.history.restore('')
    call a:buffer.current.set(a:action.path)
endfunction

function! s:quit(buffer) abort
    call a:buffer.close_windows()
endfunction

function! s:confirm_new(input_reader) abort
    let name = a:input_reader.read('new: ', [])
    if empty(name)
        return
    endif
    let paths = map(split(name, '\v\s+'), {_, v -> '-paths=' . v})
    return 'new ' . join(paths, ' ')
endfunction

function! s:confirm_remove(action, input_reader) abort
    let answer = a:input_reader.read('remove? Y/n: ', a:action.paths)
    if empty(answer) || answer !=? 'Y'
        return
    endif
    return 'remove -no-confirm'
endfunction

function! s:confirm_rename(action, input_reader) abort
    let name = a:input_reader.read('rename to: ', [a:action.path])
    if empty(name)
        return
    endif
    return 'rename -no-confirm -path=' . name
endfunction

function! s:choose(action, buffer, input_reader) abort
    let paths = []
    for path in a:action.paths
        let answer = a:input_reader.read('already exists (f)orce (n)o: ', [path])
        if answer !=? 'f'
            continue
        endif
        call add(paths, path)
    endfor

    if empty(paths)
        return
    endif

    if a:action.has_cut
        call a:buffer.register.cut(paths)
    else
        call a:buffer.register.copy(paths)
    endif

    return 'paste -no-confirm'
endfunction

function! s:copy(action, buffer) abort
    call a:buffer.register.copy(a:action.paths)
    call a:buffer.current.clear_selection()
    call kiview#messenger#new().info('Copied: ', a:action.paths)
endfunction

function! s:cut(action, buffer) abort
    call a:buffer.register.cut(a:action.paths)
    call a:buffer.current.clear_selection()
    call kiview#messenger#new().info('Cut: ', a:action.paths)
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
        let new_buffer.history = deepcopy(a:buffer.history)

        call s:write_all(item, new_buffer)

        call new_buffer.open(a:action.split_name, a:action.mod_name)
        call new_buffer.current.set(item.path)
    endfor
endfunction
