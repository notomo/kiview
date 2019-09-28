
let s:suite = themis#suite('plugin.kiview')
let s:assert = themis#helper('assert')

function! s:suite.before_each()
    call KiviewTestBeforeEach()
    filetype on
    syntax enable
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
    filetype off
    syntax off
endfunction

function! s:lines() abort
    return getbufline('%', 1, '$')
endfunction

function! s:input_reader(answer) abort
    let input_reader = {'answer': a:answer}
    function! input_reader.read(msg) abort
        call themis#log('[prompt] ' . a:msg . self.answer)
        return self.answer
    endfunction
    call kiview#input_reader#set(input_reader)

    return input_reader
endfunction

function! s:syntax_name() abort
    return synIDattr(synID(line('.'), col('.'), v:true), 'name')
endfunction

function! s:count_window() abort
    return tabpagewinnr(tabpagenr(), '$')
endfunction

function! s:count_tab() abort
    return tabpagenr('$')
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

function! s:main(arg) abort
    let line = line('.')
    return kiview#main([line, line], a:arg)
endfunction

function! s:sync_main(arg) abort
    let command = s:main(a:arg)
    call command.wait()
    return command
endfunction

function! s:suite.create()
    call s:sync_main('')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    call s:assert.equals(s:syntax_name(), 'KiviewNode')
    call search('autoload\/')
    call s:assert.equals(s:syntax_name(), 'KiviewNode')
endfunction

function! s:suite.do_parent_child()
    cd ./test/plugin

    call s:sync_main('')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    call s:sync_main('parent')

    let test_lines = s:lines()
    call s:assert.not_empty(test_lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    call s:sync_main('parent')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    call search('test/')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(lines, test_lines)

    call search('\.themisrc')
    call s:sync_main('child')

    call s:assert.equals(s:file_name(), '.themisrc')
    call s:assert.equals(&filetype, 'vim')
endfunction

function! s:suite.quit()
    call s:sync_main('')

    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(s:count_window(), 2)

    call s:sync_main('quit')

    call s:assert.not_equals('kiview', &filetype)
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.quit_option()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -quit')

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.tab_open()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -layout=tab')

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_tab(), 2)
endfunction

function! s:suite.vertical()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -layout=vertical')

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 3)
endfunction

function! s:suite.history()
    cd ./src

    call s:sync_main('')

    call search('src')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.contains(lines, 'repository/')

    call search('repository')
    call s:sync_main('child')

    call s:sync_main('parent')
    call s:sync_main('parent')
    call s:sync_main('parent')

    call s:sync_main('child')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.contains(lines, 'repository/')
endfunction

function! s:suite.no_error_with_continuous()
    cd ./src/src/repository

    let create_command = s:main('')
    let parent_command1 = s:main('parent')
    let parent_command2 = s:main('parent')

    call create_command.wait()
    call parent_command1.wait()
    call parent_command2.wait()
endfunction

function! s:suite.nop_logger()
    call kiview#logger#clear()

    call s:sync_main('')

    call s:sync_main('parent')
endfunction

function! s:suite.range()
    cd ./src

    call s:sync_main('')

    let line = search('Cargo\.toml')
    let command = kiview#main([line, line + 1], 'child -layout=tab')
    call command.wait()

    call s:assert.equals(s:count_tab(), 3)
endfunction

function! s:suite.parent_marker()
    cd ./src

    call s:sync_main('')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.contains(lines, 'autoload/')
endfunction

function! s:suite.go()
    call s:sync_main('')
    call s:sync_main('go -path=./autoload')

    let lines = s:lines()
    call s:assert.contains(lines, 'kiview/')
endfunction

function! s:suite.new()
    cd ./test/plugin/_test_data

    let input_reader = s:input_reader('new/')

    call s:sync_main('')
    call s:sync_main('new')

    call search('new\/')
    call s:sync_main('child')

    let input_reader = s:input_reader('new_file')

    call s:sync_main('new')

    let lines = s:lines()
    call s:assert.contains(lines, 'new_file')

    call search('new_file')
    call s:sync_main('child')

    call s:assert.equals(s:file_name(), 'new_file')
endfunction

function! s:suite.cancel_new()
    let input_reader = s:input_reader('')

    call s:sync_main('')
    call s:sync_main('new')

    let lines = s:lines()
    call s:assert.contains(lines, 'autoload/')
endfunction

function! s:suite.remove()
    cd ./test/plugin/_test_data

    let input_reader = s:input_reader('Y')

    call s:sync_main('')

    let first_line = search('removed_file1')
    let last_line = search('removed_file2')
    let command = kiview#main([first_line, last_line], 'remove')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_contains(lines, 'removed_file1')
    call s:assert.not_contains(lines, 'removed_file2')

    call search('removed_dir\/')
    call s:sync_main('remove')

    let lines = s:lines()
    call s:assert.not_contains(lines, 'removed_dir/')
endfunction

function! s:suite.cancel_remove()
    cd ./test/plugin/_test_data

    let input_reader = s:input_reader('')

    call s:sync_main('')

    call search('removed_cancel_file')
    call s:sync_main('remove')

    let lines = s:lines()
    call s:assert.contains(lines, 'removed_cancel_file')
endfunction

function! s:suite.no_remove()
    cd ./test/plugin/_test_data

    let input_reader = s:input_reader('n')

    call s:sync_main('')

    call search('removed_cancel_file')
    call s:sync_main('remove')

    let lines = s:lines()
    call s:assert.contains(lines, 'removed_cancel_file')
endfunction

function! s:suite.copy_and_paste()
    cd ./test/plugin/_test_data

    call s:sync_main('')

    call search('copy_file')
    call s:sync_main('cut')
    call s:sync_main('copy') " copy disables cut

    call search('paste\/')
    call s:sync_main('child')

    call s:sync_main('paste')

    let lines = s:lines()
    call s:assert.contains(lines, 'copy_file')

    call search('copy_file')
    call s:sync_main('child')

    call s:assert.equals(s:file_name(), 'copy_file')

    call s:sync_main('')

    let lines = s:lines()
    call s:assert.contains(lines, 'copy_file')
endfunction

function! s:suite.cut_and_paste()
    cd ./test/plugin/_test_data

    call s:sync_main('')

    call search('cut_file')
    call s:sync_main('cut')

    call search('paste\/')
    call s:sync_main('child')

    call s:sync_main('paste')

    let lines = s:lines()
    call s:assert.contains(lines, 'cut_file')

    call search('cut_file')
    call s:sync_main('child')

    call s:assert.equals(s:file_name(), 'cut_file')

    call s:sync_main('')

    let lines = s:lines()
    call s:assert.not_contains(lines, 'cut_file')
endfunction

function! s:suite.rename()
    cd ./test/plugin/_test_data

    let input_reader = s:input_reader('renamed_file')

    call s:sync_main('')

    call search('rename_file')
    call s:sync_main('rename')

    let lines = s:lines()
    call s:assert.contains(lines, 'renamed_file')
    call s:assert.not_contains(lines, 'rename_file')
endfunction
