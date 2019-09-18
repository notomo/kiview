
let s:handlers = {
    \ 'open': { target -> s:open(target) }
\ }

function! kiview#action#handle(action) abort
    if !has_key(s:handlers, a:action.name)
        return
    endif

    call s:handlers[a:action.name](a:action.target)
endfunction

function! s:open(target) abort
    wincmd w
    execute 'edit' a:target
endfunction
