
function! kiview#main(range, arg) abort
    let buffer = kiview#buffer#new(a:range)
    let input_reader = kiview#input_reader#new()
    let action_handler = kiview#action#new_handler(buffer, input_reader)

    let event_service = kiview#event#service()

    let parent_id = v:null
    let command = kiview#command#new(buffer, action_handler, event_service, a:arg, parent_id)
    call command.start()

    return command
endfunction
