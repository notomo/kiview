
function! kiview#action#new_handler(buffer, input_reader) abort
    let buffer = a:buffer
    let input_reader = a:input_reader
    let handler = {
        \ 'funcs': {
            \ 'open': { args, options -> s:open_targets(args) },
            \ 'tab_open': { args, options -> s:tab_open_targets(args) },
            \ 'vertical_open': { args, options -> s:vertical_open_targets(args) },
            \ 'create': { args, options -> s:create(buffer, args, options) },
            \ 'update': { args, options -> s:update(buffer, args, options) },
            \ 'quit': { args, options -> s:quit(buffer) },
            \ 'confirm_new': { args, options -> s:confirm_new(input_reader) },
            \ 'confirm_remove': { args, options -> s:confirm_remove(input_reader) },
            \ 'confirm_rename': { args, options -> s:confirm_rename(args, input_reader) },
            \ 'copy': { args, options -> s:copy(buffer, args) },
            \ 'cut': { args, options -> s:cut(buffer, args) },
            \ 'clear_register': { args, options -> s:clear_register(buffer) },
        \ },
    \ }

    function! handler.handle(action) abort
        if !has_key(self.funcs, a:action.name)
            return
        endif

        return self.funcs[a:action.name](a:action.args, a:action.options)
    endfunction

    return handler
endfunction

function! s:open_targets(args) abort
    wincmd w
    for arg in a:args
        execute 'edit' arg
    endfor
endfunction

function! s:tab_open_targets(args) abort
    for arg in a:args
        execute 'tabedit' arg
    endfor
endfunction

function! s:vertical_open_targets(args) abort
    wincmd w
    for arg in a:args
        execute 'vsplit' arg
    endfor
endfunction

function! s:create(buffer, args, options) abort
    call a:buffer.write(a:args)
    call a:buffer.set(a:options)
    call a:buffer.open()
endfunction

function! s:update(buffer, args, options) abort
    call a:buffer.write(a:args)
    call a:buffer.restore_cursor(a:options)
    call a:buffer.set(a:options)
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

function! s:confirm_rename(args, input_reader) abort
    let message = printf('rename from %s to: ', a:args[0])
    let name = a:input_reader.read(message)
    if empty(name)
        return
    endif
    return 'rename -no-confirm -path=' . name
endfunction

function! s:copy(buffer, args) abort
    call a:buffer.save_register(a:args)
endfunction

function! s:cut(buffer, args) abort
    call a:buffer.save_cut_register(a:args)
endfunction

function! s:clear_register(buffer) abort
    call a:buffer.clear_register()
endfunction
