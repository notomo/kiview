
let s:suite = themis#suite('plugin.kiview')
let s:assert = themis#helper('assert')

function! s:suite.before_each()
    call KiviewTestBeforeEach()
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
endfunction

function! s:suite.create()
    let node = kiview#main('')
    call node.wait()

    let lines = node.lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'autoload/'), 0, '`autoload/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)
endfunction

function! s:suite.do_parent()
    cd ./test/plugin

    let node = kiview#main('')
    call node.wait()

    let lines = node.lines()
    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'kiview.vim'), 0, '`kiview.vim` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    let node = kiview#main('do parent')
    call node.wait()

    let test_lines = node.lines()

    call s:assert.not_empty(test_lines)
    call s:assert.not_equals(count(test_lines, 'plugin/'), 0, '`plugin/` must be in the lines')
    call s:assert.equals(count(test_lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    let node = kiview#main('do parent')
    call node.wait()

    let lines = node.lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'autoload/'), 0, '`autoload/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    call search('test/')
    let node = kiview#main('do child')
    call node.wait()

    let lines = node.lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'plugin/'), 0, '`plugin/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)
    call s:assert.equals(lines, test_lines)
endfunction
