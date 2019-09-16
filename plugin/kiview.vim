if exists('g:loaded_kiview')
    finish
endif
let g:loaded_kiview = 1

command! -nargs=* Kiview call kiview#main(<q-args>)
