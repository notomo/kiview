
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

function! s:assert.contains(haystack, needle) abort
    call s:assert.true(count(a:haystack, a:needle) != 0, a:needle . ' must be in the haystack')
endfunction

function! s:assert.not_contains(haystack, needle) abort
    call s:assert.false(count(a:haystack, a:needle) != 0, a:needle . ' must not be in the haystack')
endfunction

function! s:suite.create()
    let command = kiview#main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    call search('autoload\/')
    call s:assert.equals(s:syntax_name(), 'KiviewNode')
endfunction

function! s:suite.do_parent_child()
    cd ./test/plugin

    let command = kiview#main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    let command = kiview#main('parent')
    call command.wait()

    let test_lines = s:lines()
    call s:assert.not_empty(test_lines)
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    let command = kiview#main('parent')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    call search('test/')
    let command = kiview#main('child')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(lines, test_lines)

    call search('\.themisrc')
    let command = kiview#main('child')
    call command.wait()

    call s:assert.equals(s:file_name(), '.themisrc')
    call s:assert.equals(&filetype, 'vim')
endfunction

function! s:suite.quit()
    let command = kiview#main('')
    call command.wait()

    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(s:count_window(), 2)

    let command = kiview#main('quit')
    call command.wait()

    call s:assert.not_equals('kiview', &filetype)
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.quit_option()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -quit')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.tab_open()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -layout=tab')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(tabpagenr('$'), 2)
endfunction

function! s:suite.vertical()
    let command = kiview#main('')
    call command.wait()

    call search('Makefile')
    let command = kiview#main('child -layout=vertical')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 3)
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
    call s:assert.contains(lines, 'repository/')
endfunction

function! s:suite.no_error_with_continuous()
    cd ./src/src/repository

    let create_command = kiview#main('')
    let parent_command1 = kiview#main('parent')
    let parent_command2 = kiview#main('parent')

    call create_command.wait()
    call parent_command1.wait()
    call parent_command2.wait()
endfunction

function! s:suite.nop_logger()
    call kiview#logger#clear()

    let command = kiview#main('')
    call command.wait()

    let command = kiview#main('parent')
    call command.wait()
endfunction
