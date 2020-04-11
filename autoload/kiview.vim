doautocmd User KiviewSourceLoad

let s:last_command = v:null

function! kiview#main(arg, ...) abort
    let options = get(a:000, 0, {})
    let bufnr = get(options, 'bufnr', bufnr('%'))
    let range = get(options, 'range', [line('.'), line('.')])

    let buffer = kiview#buffer#get_or_create(bufnr)
    let event_service = kiview#event#service()

    let parent_id = v:null
    let command = kiview#command#new(buffer, range, event_service, a:arg, parent_id)
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

if !exists('s:job_id')
  let s:job_id = 0
endif

function! kiview#main_dev(arg, ...) abort
    call s:start()
    call s:notify(a:arg)

    let command = {}
    function! command.wait() abort
        sleep 1000m
    endfunction
    return command
endfunction

let s:executable = expand('<sfile>:h:h') . '/src/target/debug/kiview'

function! s:start() abort
    if s:job_id != 0
        return
    endif

    let id = jobstart([s:executable], {
        \ 'rpc': v:true,
        \ 'on_stderr': function('s:on_stderr')
    \ })
    if id <= 0
        throw 'failed to start job: ' . id
    endif
    let s:job_id = id
    return id
endfunction

function! s:notify(arg)
  call rpcnotify(s:job_id, 'do', a:arg)
endfunction

function! s:on_stderr(id, data, event) dict
  echomsg 'stderr: ' . join(a:data, "\n")
endfunction
