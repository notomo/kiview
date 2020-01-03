
function! kiview#input_reader#clear() abort
    let s:func = function('input')
endfunction
call kiview#input_reader#clear()

function! kiview#input_reader#set_func(func) abort
    let s:func = a:func
endfunction

function! kiview#input_reader#new() abort
    let input_reader = {
        \ 'func': s:func,
    \ }

    function! input_reader.read(propmt, targets, ...) abort
        let default = ''
        if len(a:000) > 0
            let default = a:000[0]
        endif

        call inputsave()

        let message = a:propmt
        if !empty(a:targets)
            let message = join(a:targets, "\n") . "\n" . message
        endif

        let input = self.func(message, default)
        call inputrestore()
        return input
    endfunction

    return input_reader
endfunction
