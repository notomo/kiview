
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

function! s:syntax_name() abort
    return synIDattr(synID(line('.'), col('.'), v:true), 'name')
endfunction

function! s:count_window() abort
    return tabpagewinnr(tabpagenr(), '$')
endfunction

function! s:file_name() abort
    return fnamemodify(bufname('%'), ':t')
endfunction

function! s:suite.create()
    let command = kiview#main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.not_equals(count(lines, 'autoload/'), 0, '`autoload/` must be in the lines')
    call s:assert.equals(count(lines, ''), 0, ''' must not be in the lines')
    call s:assert.equals('kiview', &filetype)

    call search('autoload\/')
    call s:assert.equals('KiviewNode', s:syntax_name())
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

    call s:assert.equals(s:file_name(), '.themisrc')
    call s:assert.equals('vim', &filetype)
endfunction

function! s:suite.quit()
    let command = kiview#main('')
    call command.wait()

    call s:assert.equals('kiview', &filetype)
    call s:assert.equals(2, s:count_window())

    let command = kiview#main('quit')
    call command.wait()

    call s:assert.not_equals('kiview', &filetype)
    call s:assert.equals(1, s:count_window())
endfunction

function! s:suite.quit_option()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -quit')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(1, s:count_window())
endfunction

function! s:suite.tab_open()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -layout=tab')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(2, tabpagenr('$'))
endfunction

function! s:suite.history()
    cd ./src

    let command = kiview#main('')
    call command.wait()

    call search('src')
    let command = kiview#main('child')
    call command.wait()

    call search('repository')
    let command = kiview#main('child')
    call command.wait()

    let command = kiview#main('parent')
    call command.wait()
    let command = kiview#main('parent')
    call command.wait()
    let command = kiview#main('parent')
    call command.wait()

    let command = kiview#main('child')
    call command.wait()
    let command = kiview#main('child')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_equals(count(lines, 'repository/'), 0, '`repository/` must be in the lines')
endfunction
