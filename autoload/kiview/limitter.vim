
function! kiview#limitter#new() abort
    let limitter = {
        \ '_running': v:false,
        \ '_children': {},
        \ '_event_service': kiview#event#service(),
        \ 'logger': kiview#logger#new('limitter'),
    \ }

    function! limitter.start(func, id, parent_id) abort
        if self._running && empty(a:parent_id)
            call self.logger.log('could not start')
            return
        endif

        if !empty(a:parent_id)
            let parent_id = a:parent_id
            call self._event_service.on_command_finished(a:id, { id -> self._on_child_finished(id, parent_id) })
            if !has_key(self._children, a:parent_id)
                let self._children[a:parent_id] = {}
            endif
            let self._children[a:parent_id][a:id] = v:true
        endif

        call a:func()

        let self._running = v:true
    endfunction

    function! limitter._on_child_finished(id, parent_id) abort
        call remove(self._children[a:parent_id], a:id)
        let self._running = !empty(a:parent_id) && !empty(self._children[a:parent_id])
        if !self._running
            call remove(self._children, a:parent_id)
        endif
    endfunction

    function! limitter.finish() abort
        let self._running = v:false
        call self.logger.log('finished')
    endfunction

    return limitter
endfunction
