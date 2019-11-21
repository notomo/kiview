
let s:limitter = kiview#limitter#new()
let s:id = 0

function! kiview#command#new(buffer, action_handler, event_service, arg, parent_id) abort
    let s:id += 1

    let cmd = s:build_cmd(a:buffer, a:arg)
    let command = {
        \ 'id': s:id,
        \ 'parent_id': a:parent_id,
        \ 'job': kiview#job#new(cmd, a:event_service),
        \ 'buffer': a:buffer,
        \ 'event_service': a:event_service,
        \ 'action_handler': a:action_handler,
        \ 'children': [],
        \ 'logger': kiview#logger#new('command: ' . s:id).label('parent: ' . a:parent_id),
    \ }

    function! command.start() abort
        call s:limitter.start({ -> self._start() }, self.id, self.parent_id, { id, callback -> self.event_service.on_command_finished(id, callback) })
    endfunction

    function! command._start() abort
        call self.logger.log('start')
        call self.event_service.on_job_finished(self.job.id, { id, err -> self.on_job_finished(id, err) })
        call self.job.start()
    endfunction

    function! command.on_job_finished(id, err) abort
        if !empty(a:err)
            call kiview#messenger#new().error(a:err)
            call self.event_service.command_finished(self.id)
            return
        endif

        try
            let json = json_decode(join(self.job.stdout, ''))
            for action in json['actions']
                let arg = self.action_handler.handle(action)
                if empty(arg)
                    continue
                endif

                let child = kiview#command#new(self.buffer, self.action_handler, self.event_service, arg, self.id)
                call add(self.children, child)
                call child.start()
            endfor
        catch
            call self.logger.trace(v:throwpoint)
            echoerr v:exception
        finally
            call self.event_service.command_finished(self.id)
        endtry

        call self.logger.log('finished')
    endfunction

    function! command.wait(...) abort
        if empty(a:000)
            let timeout_msec = 100
        else
            let timeout_msec = a:000[0]
        endif

        call self.job.wait(timeout_msec)
        for child in self.children
            call child.wait(timeout_msec)
        endfor
    endfunction

    return command
endfunction

function! s:build_cmd(buffer, arg) abort
    let options = {
        \ 'current-path': a:buffer.current.path,
        \ 'line-number': a:buffer.current.line_number,
        \ 'current-target': a:buffer.current.target,
        \ 'next-sibling-line-number': a:buffer.current.next_sibling_line_number,
        \ 'depth': a:buffer.current.depth,
        \ 'arg': a:arg,
    \ }

    let cmd_options = []
    for [k, v] in items(options)
        if empty(v)
            continue
        endif
        call extend(cmd_options, ['--' . k, v])
    endfor
    for target in a:buffer.current.targets
        call extend(cmd_options, ['--targets', target])
    endfor

    let register = a:buffer.register
    for path in register.paths
        call extend(cmd_options, ['--registered', path])
    endfor
    if register.has_cut
        call add(cmd_options, '--has-cut')
    endif

    if a:buffer.current.created
        call add(cmd_options, '--created')
    endif

    return extend(['kiview', 'do'], cmd_options)
endfunction
