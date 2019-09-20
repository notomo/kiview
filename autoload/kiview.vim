
function! kiview#main(arg) abort
    let buffer = kiview#buffer#find()
    let action_handler = kiview#action#new_handler(buffer)

    let event_service = kiview#event#service()

    let command = kiview#command#new(buffer, action_handler, event_service, a:arg)
    call command.start()

    return command
endfunction
