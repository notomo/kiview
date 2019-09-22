
function! kiview#command#new(buffer, action_handler, event_service, arg) abort
    let cmd = s:build_cmd(a:buffer, a:arg)
    let command = {
        \ 'job': kiview#job#new(cmd, a:event_service),
        \ 'event_service': a:event_service,
        \ 'action_handler': a:action_handler,
        \ 'logger': kiview#logger#new().label('command'),
    \ }

    function! command.start() abort
        call self.event_service.on_job_finished(self.job.id, { id -> self.on_job_finished(id) })
        call self.job.start()
    endfunction

    function! command.on_job_finished(id) abort
        let json = json_decode(join(self.job.stdout, ''))

        for action in json['actions']
            call self.action_handler.handle(action)
        endfor

        call self.logger.log('finished callback on job finished')
    endfunction

    function! command.wait(...) abort
        if empty(a:000)
            let timeout_msec = 100
        else
            let timeout_msec = a:000[0]
        endif

        return self.job.wait(timeout_msec)
    endfunction

    return command
endfunction

function! s:build_cmd(buffer, arg) abort
    let options = {
        \ 'current-path': a:buffer.current_path,
        \ 'line-number': a:buffer.line_number,
        \ 'current-target': a:buffer.current_target,
        \ 'arg': a:arg,
    \ }
    let cmd_options = []
    for [k, v] in items(options)
        if empty(v)
            continue
        endif
        call extend(cmd_options, ['--' . k, v])
    endfor
    for target in a:buffer.targets
        call extend(cmd_options, ['--targets', target])
    endfor

    return extend(['kiview', 'do'], cmd_options)
endfunction
