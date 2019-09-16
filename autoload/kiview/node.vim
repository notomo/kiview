
let s:id = 0

function! kiview#node#new(arg, event_service, options) abort
    let s:id += 1

    let cmd_options = []
    for [k, v] in items(a:options)
        call extend(cmd_options, ['--' . k, v])
    endfor
    let cmd = extend(['kiview', 'run', '--arg', a:arg], cmd_options)

    let node = {
        \ 'id': s:id,
        \ 'job': kiview#job#new(cmd, a:event_service),
        \ 'event_service': a:event_service,
        \ 'logger': kiview#logger#new().label('node'),
        \ '_lines': [],
        \ 'options': a:options,
    \ }

    function! node.lines() abort
        return self._lines
    endfunction

    function! node.on_job_finished(id) abort
        let json = json_decode(join(self.job.stdout, ''))
        let self._lines = json['lines']
        let self.options = {'cwd': json['cwd']}
        call self.event_service.node_updated(self.id)
        call self.logger.log('finished callback on job finished')
    endfunction

    function! node.collect() abort
        call self.event_service.on_job_finished(self.job.id, { id -> self.on_job_finished(id) })
        call self.job.start()
    endfunction

    function! node.wait(...) abort
        if empty(a:000)
            let timeout_msec = 100
        else
            let timeout_msec = a:000[0]
        endif

        return self.job.wait(timeout_msec)
    endfunction

    return node
endfunction
