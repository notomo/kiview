
function! kiview#main(range, arg) abort
    let buffer = kiview#buffer#find(a:range)
    let event_service = kiview#event#service()

    let parent_id = v:null
    let command = kiview#command#new(buffer, event_service, a:arg, parent_id)
    call command.start()

    return command
endfunction
