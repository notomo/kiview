
let s:suite = themis#suite('plugin.kiview')
let s:assert = themis#helper('assert')

function! s:suite.before_each()
    call KiviewTestBeforeEach()
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
endfunction

function! s:suite.run()
    let node = kiview#main('')
    call node.wait()

    let lines = node.lines()

    call s:assert.not_empty(lines)
endfunction
