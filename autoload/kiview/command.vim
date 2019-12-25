
let s:limitter = kiview#limitter#new()
let s:id = 0

function! kiview#command#new(buffer, event_service, arg, parent_id) abort
    let s:id += 1

    let cmd = ['kiview', 'do', '--arg=' . a:arg]
    let command = {
        \ 'id': s:id,
        \ 'parent_id': a:parent_id,
        \ 'job': kiview#job#new(cmd, a:event_service),
        \ 'buffer': a:buffer,
        \ 'event_service': a:event_service,
        \ 'action_handler': kiview#action#new_handler(a:buffer),
        \ 'children': [],
        \ 'logger': kiview#logger#new('command: ' . s:id).label('parent: ' . a:parent_id),
    \ }

    function! command.start() abort
        call s:limitter.start({ -> self._start() }, self.id, self.parent_id, { id, callback -> self.event_service.on_command_finished(id, callback) })
    endfunction

    function! command._start() abort
        call self.logger.log('start')
        call self.event_service.on_job_finished(self.job.id, { id, err -> self.on_job_finished(id, err) })
        let input = s:build_input(self.buffer)
        call self.job.start(input)
    endfunction

    function! command.on_job_finished(id, err) abort
        if !empty(a:err)
            call kiview#messenger#new().error(a:err)
            call self.buffer.current.clear_selection()
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

                let child = kiview#command#new(self.buffer, self.event_service, arg, self.id)
                call add(self.children, child)
                call child.start()
            endfor
        catch
            call self.logger.trace(v:throwpoint, v:exception)
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

function! s:build_input(buffer) abort
    let input = {
        \ 'path': a:buffer.current.path,
        \ 'line_number': a:buffer.current.line_number,
        \ 'target': a:buffer.current.target,
        \ 'next_sibling_line_number': a:buffer.current.next_sibling_line_number,
        \ 'last_sibling_line_number': a:buffer.current.last_sibling_line_number,
        \ 'targets': a:buffer.current.targets,
        \ 'selected_targets': a:buffer.current.selected_targets,
        \ 'registered_paths': a:buffer.register.paths,
        \ 'has_cut': a:buffer.register.has_cut,
        \ 'created': a:buffer.current.created,
    \ }

    return json_encode(input)
endfunction

function! kiview#command#finished() abort
    return s:limitter.finished()
endfunction

function! kiview#command#abort() abort
    let s:limitter = kiview#limitter#new()
endfunction
