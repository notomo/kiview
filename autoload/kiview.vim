
let s:last_command = v:null

function! kiview#main(range, arg, bufnr) abort
    let buffer = kiview#buffer#get_or_create(a:bufnr)
    let event_service = kiview#event#service()

    let parent_id = v:null
    let command = kiview#command#new(buffer, a:range, event_service, a:arg, parent_id)
    call command.start()

    let s:last_command = command
    return command
endfunction

function! kiview#get() abort
    let buffer = kiview#buffer#find(bufnr('%'))
    if !empty(buffer)
        return copy(buffer.current.get_target(line('.')))
    endif
    return v:null
endfunction

function! kiview#last_command() abort
    return s:last_command
endfunction
