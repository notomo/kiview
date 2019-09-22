
let s:suite = themis#suite('plugin.kiview')
let s:assert = themis#helper('assert')

function! s:suite.before_each()
    call KiviewTestBeforeEach()
    filetype on
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
    filetype off
endfunction

function! s:lines() abort
    return getbufline('%', 1, '$')
endfunction

function! s:suite.create()
    let command = kiview#main('')
    call command.wait()

    let lines = s:lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'autoload/'), 0, '`autoload/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)
endfunction

function! s:suite.do_parent_child()
    cd ./test/plugin

    let command = kiview#main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'kiview.vim'), 0, '`kiview.vim` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    let command = kiview#main('parent')
    call command.wait()

    let test_lines = s:lines()

    call s:assert.not_empty(test_lines)
    call s:assert.not_equals(count(test_lines, 'plugin/'), 0, '`plugin/` must be in the lines')
    call s:assert.equals(count(test_lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    let command = kiview#main('parent')
    call command.wait()

    let lines = s:lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'autoload/'), 0, '`autoload/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    call search('test/')
    let command = kiview#main('child')
    call command.wait()

    let lines = s:lines()

    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'plugin/'), 0, '`plugin/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)
    call s:assert.equals(lines, test_lines)

    call search('\.themisrc')
    let command = kiview#main('child')
    call command.wait()

    call s:assert.equals(fnamemodify(bufname('%'), ':t'), '.themisrc')
    call s:assert.equals('vim', &filetype)
endfunction

function! s:suite.quit()
    let command = kiview#main('')
    call command.wait()

    call s:assert.equals('kiview', &filetype)
    call s:assert.equals(2, tabpagewinnr(tabpagenr(), '$'))

    let command = kiview#main('quit')
    call command.wait()

    call s:assert.not_equals('kiview', &filetype)
    call s:assert.equals(1, tabpagewinnr(tabpagenr(), '$'))
endfunction

function! s:suite.tab_open()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -layout=tab')
    call command.wait()

    call s:assert.equals(fnamemodify(bufname('%'), ':t'), 'Makefile')
    call s:assert.equals(2, tabpagenr())
endfunction
