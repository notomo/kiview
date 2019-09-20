
let s:JOB_FINISHED = 'KiviewJobFinished'

function! kiview#event#service() abort
    let s:job_callbacks = {}

    let service = {
        \ 'logger': kiview#logger#new().label('event'),
    \ }

    function! service.on_job_finished(job_id, callback) abort
        let s:job_callbacks[a:job_id] = a:callback
        execute printf('autocmd User %s:%s ++nested ++once call s:job_callback(expand("<amatch>"))', s:JOB_FINISHED, a:job_id)
    endfunction

    function! service.job_finished(job_id) abort
        let event = printf('%s:%s', s:JOB_FINISHED, a:job_id)
        call self.logger.log(event)
        execute printf('doautocmd User %s', event)
    endfunction

    return service
endfunction

function! s:job_callback(amatch) abort
    let [_, id] = split(a:amatch, s:JOB_FINISHED . ':', 'keep')
    call s:job_callbacks[id](id)

    call remove(s:job_callbacks, id)
endfunction
