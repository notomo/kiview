
function! kiview#action#new_handler(buffer, input_reader) abort
    let buffer = a:buffer
    let input_reader = a:input_reader
    let handler = {
        \ 'funcs': {
            \ 'open': { action -> s:open_targets(action) },
            \ 'tab_open': { action -> s:tab_open_targets(action) },
            \ 'vertical_open': { action -> s:vertical_open_targets(action) },
            \ 'create': { action -> s:create(action, buffer) },
            \ 'update': { action -> s:update(action, buffer) },
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
    for arg in a:action.args
        execute 'edit' arg
    endfor
endfunction

function! s:tab_open_targets(action) abort
    for arg in a:action.args
        execute 'tabedit' arg
    endfor
endfunction

function! s:vertical_open_targets(action) abort
    wincmd w
    for arg in a:action.args
        execute 'vsplit' arg
    endfor
endfunction

function! s:create(action, buffer) abort
    call a:buffer.write(a:action.args)
    call a:buffer.set(a:action.options)
    call a:buffer.open()
endfunction

function! s:update(action, buffer) abort
    call a:buffer.write(a:action.args)
    call a:buffer.restore_cursor(a:action.options)
    call a:buffer.set(a:action.options)
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
    let message = printf('rename from %s to: ', a:action.arg)
    let name = a:input_reader.read(message)
    if empty(name)
        return
    endif
    return 'rename -no-confirm -path=' . name
endfunction

function! s:copy(action, buffer) abort
    call a:buffer.register.copy(a:action.args)
endfunction

function! s:cut(action, buffer) abort
    call a:buffer.register.cut(a:action.args)
endfunction

function! s:clear_register(buffer) abort
    call a:buffer.register.clear()
endfunction
