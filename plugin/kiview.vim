if exists('g:loaded_kiview')
    finish
endif
let g:loaded_kiview = 1

command! -range -nargs=* -complete=custom,kiview#complete#get Kiview call kiview#main(<q-args>, {'range': [<line1>, <line2>]})
command! -range -nargs=* -complete=custom,kiview#complete#get KiviewDev call kiview#main_dev(<q-args>, {'range': [<line1>, <line2>]})
