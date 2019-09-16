
let s:logger = kiview#logger#new().label('command')

function! kiview#command#create(arg) abort
    let buffer = kiview#buffer#new()
    let event_service = kiview#event#service()
    let node = kiview#node#new(a:arg, event_service, {'cwd': getcwd()})

    call event_service.on_node_updated(node.id, { id -> s:on_node_updated(id, node, buffer) })
    call node.collect()

    call buffer.open()

    return node
endfunction

function! kiview#command#do(arg) abort
    let buffer = kiview#buffer#from_buffer()
    let event_service = kiview#event#service()
    let node = kiview#node#new(a:arg, event_service, buffer.options)

    call event_service.on_node_updated(node.id, { id -> s:on_node_updated(id, node, buffer) })
    call node.collect()

    return node
endfunction

function! s:on_node_updated(id, node, buffer) abort
    call a:buffer.write(a:node.lines())
    call a:buffer.set(a:node.options)
    call s:logger.log('finished callback on node updated')
endfunction
