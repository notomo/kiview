
function! kiview#action#new_handler(buffer, input_reader) abort
    let buffer = a:buffer
    let input_reader = a:input_reader
    let handler = {
        \ 'funcs': {
            \ 'open': { action -> s:open_targets(action) },
            \ 'tab_open': { action -> s:tab_open_targets(action) },
            \ 'vertical_open': { action -> s:vertical_open_targets(action) },
            \ 'create': { action -> s:create(action, buffer) },
            \ 'add_history': { action -> s:add_history(action, buffer) },
            \ 'try_to_restore_cursor': { action -> s:try_to_restore_cursor(action, buffer) },
            \ 'set_cursor': { action -> s:set_cursor(action, buffer) },
            \ 'set_path': { action -> s:set_path(action, buffer) },
            \ 'write_all': { action -> s:write_all(action, buffer) },
            \ 'open_tree': { action -> s:open_tree(action, buffer) },
            \ 'close_tree': { action -> s:close_tree(action, buffer) },
            \ 'quit': { action -> s:quit(buffer) },
            \ 'confirm_new': { action -> s:confirm_new(input_reader) },
            \ 'confirm_remove': { action -> s:confirm_remove(input_reader) },
            \ 'confirm_rename': { action -> s:confirm_rename(action, input_reader) },
            \ 'copy': { action -> s:copy(action, buffer) },
            \ 'cut': { action -> s:cut(action, buffer) },
            \ 'clear_register': { action -> s:clear_register(buffer) },
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

function! s:open_targets(action) abort
    wincmd w
    for path in a:action.paths
        execute 'edit' path
    endfor
endfunction

function! s:tab_open_targets(action) abort
    for path in a:action.paths
        execute 'tabedit' path
    endfor
endfunction

function! s:vertical_open_targets(action) abort
    wincmd w
    for path in a:action.paths
        execute 'vsplit' path
    endfor
endfunction

function! s:write_all(action, buffer) abort
    call a:buffer.current.unset_props_all()
    call a:buffer.write_all(a:action.lines)
    call a:buffer.current.set_props(a:action.props, 1)
endfunction

function! s:open_tree(action, buffer) abort
    call a:buffer.current.toggle_tree(a:action.root, v:true)
    if a:action.count == 0
        return
    endif
    let start = a:action.root + 1
    call a:buffer.write(a:action.lines, start, start)
    call a:buffer.current.set_props(a:action.props, start)
endfunction

function! s:close_tree(action, buffer) abort
    call a:buffer.current.toggle_tree(a:action.root, v:false)
    if a:action.count == 0
        return
    endif
    let start = a:action.root + 1
    let end = a:action.root + a:action.count
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
    call a:buffer.open()
endfunction

function! s:quit(buffer) abort
    call a:buffer.close_windows()
endfunction

function! s:confirm_new(input_reader) abort
    let name = a:input_reader.read('new: ')
    if empty(name)
        return
    endif
    return 'new -path=' . name
endfunction

function! s:confirm_remove(input_reader) abort
    let answer = a:input_reader.read('remove? Y/n: ')
    if empty(answer) || answer !=? 'Y'
        return
    endif
    return 'remove -no-confirm'
endfunction

function! s:confirm_rename(action, input_reader) abort
    let message = printf('rename from %s to: ', a:action.path)
    let name = a:input_reader.read(message)
    if empty(name)
        return
    endif
    return 'rename -no-confirm -path=' . name
endfunction

function! s:copy(action, buffer) abort
    call a:buffer.register.copy(a:action.paths)
endfunction

function! s:cut(action, buffer) abort
    call a:buffer.register.cut(a:action.paths)
endfunction

function! s:clear_register(buffer) abort
    call a:buffer.register.clear()
endfunction
