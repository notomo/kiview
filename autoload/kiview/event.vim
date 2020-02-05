
let s:JOB_FINISHED = 'KiviewJobFinished'
let s:COMMAND_FINISHED = 'KiviewCommandFinished'

let s:callbacks = {}

function! kiview#event#service() abort
    let service = {
        \ 'logger': kiview#logger#new('event'),
    \ }

    function! service.on_job_finished(id, callback) abort
        call self._on_event(s:JOB_FINISHED, a:id, a:callback)
    endfunction

    function! service.on_command_finished(id, callback) abort
        call self._on_event(s:COMMAND_FINISHED, a:id, a:callback)
    endfunction

    function! service._on_event(event_name, id, callback) abort
        if !has_key(s:callbacks, a:event_name)
            let s:callbacks[a:event_name] = {}
        endif
        let s:callbacks[a:event_name][a:id] = a:callback
        execute printf('autocmd User %s:%s:*,%s:%s:*/* ++nested ++once call s:callback(expand("<amatch>"), "%s")', a:event_name, a:id, a:event_name, a:id, a:event_name)
    endfunction

    function! service.job_finished(id, err) abort
        call self._emit(s:JOB_FINISHED, a:id . ':' . a:err)
    endfunction

    function! service.command_finished(id) abort
        call self._emit(s:COMMAND_FINISHED, a:id . ':')
    endfunction

    function! service._emit(event_name, id) abort
        let event = printf('%s:%s', a:event_name, a:id)
        call self.logger.log(event)
        execute printf('doautocmd User %s', event)
    endfunction

    return service
endfunction

function! s:callback(amatch, event_name) abort
    let id_err = a:amatch[stridx(a:amatch, a:event_name . ':'):]
    let index = stridx(id_err, ':', len(a:event_name . ':'))
    let id = id_err[len(a:event_name . ':') : index - 1]
    let err = id_err[index + 1 :]
    if !has_key(s:callbacks[a:event_name], id)
        return
    endif
    call s:callbacks[a:event_name][id](id, err)

    call remove(s:callbacks[a:event_name], id)
endfunction
