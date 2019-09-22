
function! kiview#action#new_handler(buffer) abort
    let buffer = a:buffer
    let handler = {
        \ 'funcs': {
            \ 'open': { args, options -> s:open_targets(args) },
            \ 'tab_open': { args, options -> s:tab_open_targets(args) },
            \ 'create': { args, options -> s:create(buffer, args, options) },
            \ 'update': { args, options -> s:update(buffer, args, options) },
            \ 'quit': { args, options -> s:quit(buffer) },
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
