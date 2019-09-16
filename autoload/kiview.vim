
let s:logger = kiview#logger#new().label('kiview')

function! kiview#main(arg) abort
    call s:logger.log('arg: ' . a:arg)

    let factors = split(a:arg, '\v\s+', v:true)
    let action_name = factors[0]
    let arg = join(factors[1:], ' ')

    if action_name ==# 'do'
        return kiview#command#do(arg)
    endif
    return kiview#command#create(arg)
endfunction
