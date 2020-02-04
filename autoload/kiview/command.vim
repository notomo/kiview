
let s:limitter = kiview#limitter#new()
let s:id = 0

" TODO: configurable
let s:executable = expand('<sfile>:h:h:h') . '/src/target/debug/kiview'
function! kiview#command#executable() abort
    return s:executable
endfunction

function! kiview#command#new(buffer, range, event_service, arg, parent_id) abort
    let s:id += 1

    let cmd = [s:executable, 'do', '--arg=' . a:arg]
    let command = {
        \ 'id': s:id,
        \ 'parent_id': a:parent_id,
        \ 'job': kiview#job#new(cmd, a:event_service),
        \ 'buffer': a:buffer,
        \ 'range': a:range,
        \ 'event_service': a:event_service,
        \ 'action_handler': kiview#action#new_handler(a:buffer, a:arg),
        \ 'children': [],
        \ 'logger': kiview#logger#new('command: ' . s:id).label('parent: ' . a:parent_id),
    \ }

    function! command.start() abort
        call s:limitter.start({ -> self._start() }, self.id, self.parent_id, { id, callback -> self.event_service.on_command_finished(id, callback) })
    endfunction

    function! command._start() abort
        call self.logger.log('start')
        call self.event_service.on_job_finished(self.job.id, { id, err -> self.on_job_finished(id, err) })
        let input = s:build_input(self.buffer, self.range)
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

                let child = kiview#command#new(self.buffer, self.range, self.event_service, arg, self.id)
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
            let timeout_msec = 1000
        else
            let timeout_msec = a:000[0]
        endif

        call self.job.wait(timeout_msec)
        sleep 15m " FIXME: workaround for returning -3 by jobwait()
        for child in self.children
            call child.wait(timeout_msec)
        endfor
    endfunction

    return command
endfunction

function! s:build_input(buffer, range) abort
    let cwd = getcwd()
    let path = !empty(cwd) ? cwd : expand('%:p:h')
    let path = substitute(path, '\', '/', 'g')
    let input = {
        \ 'path': path,
        \ 'name': expand('%'),
        \ 'line_number': 2,
        \ 'registered_targets': a:buffer.register.targets,
        \ 'rename_targets': a:buffer.renamer.targets,
        \ 'renamer_opened': a:buffer.renamer.opened(),
        \ 'has_cut': a:buffer.register.has_cut,
    \ }

    if a:buffer.opened
        let input.line_number = line('.')
        let input.target = a:buffer.current.get_target(input.line_number)
        let input.targets = a:buffer.current.get_targets(a:range[0], a:range[1])
        let input.selected_targets = a:buffer.current.get_selected_targets()
        let input.opened = v:true
    endif

    return json_encode(input)
endfunction

function! kiview#command#finished() abort
    return s:limitter.finished()
endfunction

function! kiview#command#abort() abort
    let s:limitter = kiview#limitter#new()
endfunction
