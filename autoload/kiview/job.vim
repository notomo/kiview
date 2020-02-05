
let s:id = 0

function! kiview#job#new(cmd, event_service) abort
    let s:id += 1

    let job = {
        \ 'id': s:id,
        \ 'cmd': a:cmd,
        \ 'logger': kiview#logger#new('job: ' . s:id),
        \ 'stdout': [],
        \ 'stderr': [],
        \ 'started': v:false,
        \ 'done': v:false,
        \ 'event_service': a:event_service,
    \ }

    function! job.start(input) abort
        let options = {
            \ 'on_exit': function('s:handle_exit'),
            \ 'on_stdout': function('s:handle_stdout'),
            \ 'on_stderr': function('s:handle_stderr'),
            \ 'job': self,
        \ }

        call self.logger.log('start: ' . join(self.cmd, ' '))
        let self.internal_job_id = jobstart(self.cmd, options)
        if self.internal_job_id <= 0
            call self.logger.label('error').log('internal_job_id=' . self.internal_job_id)
            throw 'failed to start job: ' . self.internal_job_id
        endif
        call self.logger.log(a:input)
        let written = chansend(self.internal_job_id, [a:input, ''])
        if written == 0
            throw 'failed to chansend()'
        endif
        let self.started = v:true
    endfunction

    function! job.wait(timeout_msec) abort
        if !self.started
            call self.logger.log('has not started')
            return v:false
        endif
        if self.done
            call self.logger.log('already done')
            return v:true
        endif

        call self.logger.log('wait: ' . self.internal_job_id)
        let result = jobwait([self.internal_job_id], a:timeout_msec)
        call self.logger.log('wait result: ' . result[0])
        if result[0] == -3
            " FIXME
            call self.logger.log('invalid job id?')
            return v:false
        elseif result[0] != -1
            call self.logger.log('done')
            return v:true
        endif

        call jobstop(self.internal_job_id)
        throw printf('has not done in %d ms.', a:timeout_msec)
    endfunction

    function! job.on_finished() abort
        let self.done = v:true
        call self.event_service.job_finished(self.id, join(self.stderr, "\n"))
    endfunction

    return job
endfunction

function! s:handle_stderr(job_id, data, event) abort dict
    let valid_data = filter(a:data, { _, v -> !empty(v) })
    let valid_data = map(valid_data, { _, v -> tr(v, "\n", ' ') })
    call extend(self.job.stderr, valid_data)
    call self.job.logger.label('stderr').logs(valid_data)
endfunction

function! s:handle_stdout(job_id, data, event) abort dict
    let valid_data = filter(a:data, { _, v -> !empty(v) })
    call extend(self.job.stdout, valid_data)
    call self.job.logger.label('stdout').logs(valid_data)
endfunction

function! s:handle_exit(job_id, exit_code, event) abort dict
    call self.job.on_finished()
endfunction
