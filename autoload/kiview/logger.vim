
function! kiview#logger#clear() abort
    let s:logger_func = v:null
endfunction

call kiview#logger#clear()


function! kiview#logger#set_func(func) abort
    let s:logger_func = { message -> a:func(message) }
endfunction

function! kiview#logger#new() abort
    if empty(s:logger_func)
        return s:nop_logger()
    endif
    let logger = {'func': s:logger_func}

    function! logger.label(label) abort
        let self._label = printf('[%s] ', a:label)
        return self
    endfunction
    call logger.label('log')

    function! logger.logs(messages) abort
        for msg in a:messages
            call self.log(msg)
        endfor
    endfunction

    function! logger.log(message) abort
        " FIXME: REMOVE ANSI
        let message = substitute(a:message, "\<ESC>\\[\\d*[a-zA-Z]", '', 'g')
        call self.func(self._label . message)
    endfunction

    return logger
endfunction

function! s:nop_logger() abort
    let logger = {}

    function! logger.label(label) abort
        return self
    endfunction

    function! logger.logs(messages) abort
    endfunction

    function! logger.log(message) abort
    endfunction

    return logger
endfunction
