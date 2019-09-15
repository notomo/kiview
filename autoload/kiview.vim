
let s:logger = kiview#logger#new().label('kiview')

function! kiview#main(arg) abort
    let buffer = kiview#buffer#new()
    let event_service = kiview#event#service()
    let node = kiview#node#new(a:arg, event_service)

    call event_service.on_node_updated(node.id, { id -> s:on_node_updated(id, node, buffer) })
    call node.collect()

    call buffer.open()

    return node
endfunction

function! s:on_node_updated(id, node, buffer) abort
    call a:buffer.write(a:node.lines())
    call s:logger.log('finished callback on node updated')
endfunction
