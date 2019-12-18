
function! kiview#messenger#clear() abort
    let f = {}
    function! f.default(message) abort
        echomsg a:message
    endfunction

    let s:func = { message -> f.default(message) }
endfunction

call kiview#messenger#clear()


function! kiview#messenger#set_func(func) abort
    let s:func = { message -> a:func(message) }
endfunction

function! kiview#messenger#new() abort
    let messenger = {
        \ 'func': s:func,
    \ }

    function! messenger.warn(message) abort
        echohl WarningMsg
        call self.func('[kiview] ' . a:message)
        echohl None
    endfunction

    function! messenger.error(message) abort
        echohl ErrorMsg
        call self.func('[kiview] ' . a:message)
        echohl None
    endfunction

    function! messenger.info(message, targets) abort
        if len(a:targets) == 1
            call self.func('[kiview] ' . a:message . a:targets[0])
            return
        endif
        if empty(a:targets)
            call self.func('[kiview] ' . a:message)
            return
        endif
        let message = printf('[kiview] %s%s targets', a:message, len(a:targets))
        call self.func(message)
    endfunction

    return messenger
endfunction
