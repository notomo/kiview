
function! kiview#limitter#new() abort
    let limitter = {
        \ '_running': v:false,
        \ '_children': {},
        \ 'logger': kiview#logger#new('limitter'),
    \ }

    function! limitter.start(func, id, parent_id, set_receiver) abort
        if self._running && empty(a:parent_id)
            call self.logger.log('could not start')
            return
        endif

        let parent_id = a:parent_id
        call a:set_receiver(a:id, { id, _ -> self._on_finished(id, parent_id) })

        if !empty(a:parent_id)
            if !has_key(self._children, a:parent_id)
                let self._children[a:parent_id] = {}
            endif
            let self._children[a:parent_id][a:id] = v:true
        endif

        call a:func()

        let self._running = v:true
    endfunction

    function! limitter._on_finished(id, parent_id) abort
        if empty(a:parent_id)
            let self._running = self._has_child(a:id)
            return
        endif

        call remove(self._children[a:parent_id], a:id)
        let self._running = self._has_child(a:parent_id)

        if !self._running
            call remove(self._children, a:parent_id)
        endif
    endfunction

    function! limitter._has_child(id) abort
        return !empty(a:id) && has_key(self._children, a:id) && !empty(self._children[a:id])
    endfunction

    return limitter
endfunction
