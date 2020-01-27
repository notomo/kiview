
function! kiview#complete#get(current_arg, line, cursor_position) abort
    let line = a:line[len('Kiview') : ]
    let cmd = ['kiview', 'complete', '--arg=' . a:current_arg, '--line=' . line]
    return system(cmd)
endfunction
