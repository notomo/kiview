if exists('g:loaded_kiview')
    finish
endif
let g:loaded_kiview = 1

command! Kiview call kiview#main(<q-args>)
