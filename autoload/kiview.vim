
function! kiview#main(range, arg) abort
    let buffer = kiview#buffer#get_or_create()
    let event_service = kiview#event#service()

    let parent_id = v:null
    let command = kiview#command#new(buffer, a:range, event_service, a:arg, parent_id)
    call command.start()

    return command
endfunction

function! kiview#get() abort
    let buffer = kiview#buffer#find()
    if !empty(buffer)
        return copy(buffer.current.get_target(line('.')))
    endif
    return v:null
endfunction
