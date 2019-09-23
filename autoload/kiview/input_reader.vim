
function! kiview#input_reader#clear() abort
    let s:input_reader = v:null
endfunction
call kiview#input_reader#clear()

function! kiview#input_reader#new() abort
    if !empty(s:input_reader)
        return s:input_reader
    endif

    let input_reader = {}

    function! input_reader.read(message) abort
        call inputsave()
        let input = input(a:message)
        call inputrestore()
        return input
    endfunction

    return input_reader
endfunction

function! kiview#input_reader#set(input_reader) abort
    let s:input_reader = a:input_reader
endfunction
