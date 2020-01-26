
function! kiview#complete#get(current_arg, line, cursor_position) abort
    let arg = a:line[len('Kiview') : ]
    let cmd = ['kiview', 'complete', '--arg=' . arg]
    return system(cmd)
endfunction
