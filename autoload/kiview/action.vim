
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
            \ 'restore_cursor': { action -> s:restore_cursor(action, buffer) },
            \ 'write_all': { action -> s:write_all(action, buffer) },
            \ 'write': { action -> s:write(action, buffer) },
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
    call a:buffer.write_all(a:action.paths)
endfunction

function! s:write(action, buffer) abort
    call a:buffer.write(a:action.paths, a:action.start, a:action.end)
endfunction

function! s:restore_cursor(action, buffer) abort
    let path = a:action.path
    call a:buffer.history.restore(path, a:action.line_number)
    call a:buffer.current.set(path)
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
