
let s:running = v:false
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
        \ 'child_ids': {},
        \ 'logger': kiview#logger#new('command'),
    \ }

    function! command.start() abort
        if s:running && empty(self.parent_id)
            call self.logger.log('cannot execute more than one command at the same time')
            return
        endif
        call self.event_service.on_job_finished(self.job.id, { id -> self.on_job_finished(id) })
        call self.job.start()
        let s:running = v:true
    endfunction

    function! command.on_job_finished(id) abort
        let err = v:false
        try
            let json = json_decode(join(self.job.stdout, ''))
            for action in json['actions']
                let child_arg = self.action_handler.handle(action)
                if empty(child_arg)
                    continue
                endif
                call self.start_child(child_arg)
            endfor
        catch
            let err = v:true
            echoerr v:exception
        finally
            let s:running = !empty(self.parent_id) && !err
        endtry

        if !empty(self.parent_id)
            call self.event_service.command_finished(self.id)
        endif

        call self.logger.log('finished callback on job finished')
    endfunction

    function! command.on_child_finished(id) abort
        call remove(self.child_ids, a:id)
        let s:running = !empty(self.parent_id) && !empty(self.child_ids)
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

    function! command.start_child(arg) abort
        let child = kiview#command#new(self.buffer, self.action_handler, self.event_service, a:arg, self.id)
        call self.event_service.on_command_finished(child.id, { id -> self.on_child_finished(id) })
        call add(self.children, child)
        let self.child_ids[child.id] = v:true

        call self.logger.log('child_ids: ' . string(self.child_ids))

        call child.start()
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

    let register = a:buffer.register
    for path in register.paths
        call extend(cmd_options, ['--registered', path])
    endfor
    if register.has_cut
        call add(cmd_options, '--has-cut')
    endif

    return extend(['kiview', 'do'], cmd_options)
endfunction
