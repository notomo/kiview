
let s:helper = KiviewTestHelper('plugin.kiview')
let s:suite = s:helper.suite()
let s:assert = s:helper.assert()

function! s:suite.create_one()
    let cwd = getcwd()

    call s:helper.sync_execute('')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.line_number(2)
    call s:assert.working_dir(cwd)
    call s:assert.buffer_name('kiview')

    normal! gg
    call s:assert.syntax_name('KiviewNodeClosed')

    call s:helper.search('autoload\/')
    call s:assert.syntax_name('KiviewNodeClosed')
    call s:assert.ends_with(kiview#get().path, 'autoload')
endfunction

function! s:suite.cursor_position_on_create()
    edit Makefile
    call s:helper.sync_execute('')

    call s:assert.current_line('Makefile')
endfunction

function! s:suite.multiple_create()
    call s:helper.sync_execute('')
    call s:helper.sync_execute('') " nop
    call s:helper.sync_execute('-create')

    call s:assert.window_count(3)
endfunction

function! s:suite.do_parent_child()
    let cwd = getcwd()
    cd ./test/plugin

    call s:helper.sync_execute('')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.working_dir(cwd . '/test/plugin')

    call s:helper.sync_execute('parent')

    let test_lines = s:helper.lines()
    call s:assert.not_empty(test_lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.working_dir(cwd . '/test')

    call s:helper.sync_execute('parent')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.working_dir(cwd)

    call s:helper.search('test/')
    call s:helper.sync_execute('child')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.filetype('kiview')
    call s:assert.lines(test_lines)

    call s:helper.search('\.themisrc')
    call s:helper.sync_execute('child')

    call s:assert.file_name('.themisrc')
    call s:assert.filetype('vim')
endfunction

function! s:suite.quit()
    call s:helper.sync_execute('')

    call s:assert.filetype('kiview')
    call s:assert.window_count(2)

    call s:helper.sync_execute('quit')

    call s:assert.filetype('')
    call s:assert.window_count(1)
endfunction

function! s:suite.quit_option()
    call s:helper.sync_execute('')

    call s:helper.search('Makefile')
    call s:helper.sync_execute('child -quit')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(1)
endfunction

function! s:suite.tab_open()
    call s:helper.sync_execute('')

    call s:helper.search('Makefile')
    call s:helper.sync_execute('child -layout=tab')

    call s:assert.file_name('Makefile')
    call s:assert.tab_count(2)
endfunction

function! s:suite.vertical_open()
    call s:helper.sync_execute('')

    call s:helper.search('Makefile')
    call s:helper.sync_execute('child -layout=vertical')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(3)

    wincmd h
    call s:assert.filetype('kiview')
endfunction

function! s:suite.horizontal_open()
    call s:helper.sync_execute('')

    call s:helper.search('Makefile')
    call s:helper.sync_execute('child -layout=horizontal')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(3)

    wincmd h
    call s:assert.filetype('kiview')
endfunction

function! s:suite.history()
    cd ./src

    call s:helper.sync_execute('')

    call s:helper.search('src')
    call s:helper.sync_execute('child')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'repository/')

    call s:helper.search('repository')
    call s:helper.sync_execute('child')

    call s:helper.sync_execute('parent')
    call s:helper.sync_execute('parent')
    call s:helper.sync_execute('parent')

    call s:helper.sync_execute('child')
    call s:helper.sync_execute('child')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'repository/')
endfunction

function! s:suite.no_error_with_continuous()
    cd ./src/src/repository

    let create_command = s:helper.execute('')
    let parent_command1 = s:helper.execute('parent')
    let parent_command2 = s:helper.execute('parent')

    call create_command.wait()
    call parent_command1.wait()
    call parent_command2.wait()
endfunction

function! s:suite.nop_logger()
    call kiview#logger#clear()

    call s:helper.sync_execute('')

    call s:helper.sync_execute('parent')
endfunction

function! s:suite.range()
    cd ./src

    call s:helper.sync_execute('')

    let line = s:helper.search('Cargo\.toml')
    let command = s:helper.sync_execute('child -layout=tab', {'range': [line, line + 1]})

    call s:assert.tab_count(3)
endfunction

function! s:suite.parent_marker()
    cd ./src

    call s:helper.sync_execute('')

    normal! gg
    call s:helper.sync_execute('child')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'autoload/')
    call s:assert.line_number(2)
endfunction

function! s:suite.go()
    call s:helper.sync_execute('')
    call s:helper.sync_execute('go -path=./autoload')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'kiview/')
endfunction

function! s:suite.__new__() abort
    let suite = s:helper.sub_suite('new')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')
        call s:helper.new_directory('tree2')

        call s:helper.new_file_with_content('already', ['has contents'])
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.new_one()
        let cwd = getcwd()
        cd ./test/plugin/_test_data

        call s:helper.set_input('new/')

        call s:helper.sync_execute('')
        call s:helper.sync_execute('new')

        call s:helper.search('new\/')
        call s:helper.sync_execute('child')
        call s:assert.working_dir(cwd . '/test/plugin/_test_data/new')

        call s:helper.set_input('new_file')

        call s:helper.sync_execute('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'new_file')

        call s:helper.search('new_file')
        call s:helper.sync_execute('child')

        call s:assert.file_name('new_file')
    endfunction

    function! suite.cancel_new()
        call s:helper.set_input('')

        call s:helper.sync_execute('')
        call s:helper.sync_execute('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'autoload/')
    endfunction

    function! suite.new_already_exists()
        cd ./test/plugin/_test_data

        call s:helper.set_input('already')

        call s:helper.sync_execute('')
        call s:helper.sync_execute('new')

        call s:assert.file_not_empty('already')
    endfunction

    function! suite.new_in_tree()
        cd ./test/plugin/_test_data

        call s:helper.sync_execute('')

        call s:helper.search('tree/')
        call s:helper.sync_execute('toggle_tree')
        call s:helper.search('file_in_tree')

        call s:helper.set_input('new_in_tree')
        call s:helper.sync_execute('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  new_in_tree')
        call s:assert.contains(lines, 'tree2/')
        call s:assert.count_contains(lines, '  file_in_tree', 1)
    endfunction

    function! suite.new_multiple()
        let cwd = getcwd()
        cd ./test/plugin/_test_data
        call s:helper.set_input('new_file1 new_file2')

        call s:helper.sync_execute('')
        call s:helper.sync_execute('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'new_file1')
        call s:assert.contains(lines, 'new_file2')
    endfunction

endfunction

function! s:suite.__remove__() abort
    let suite = s:helper.sub_suite('remove')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('removed_file1')
        call s:helper.new_file('removed_file2')
        call s:helper.new_file('removed_cancel_file')

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree1')
        call s:helper.new_file('tree/file_in_tree2')

        call s:helper.new_directory('tree2')
        call s:helper.new_file('tree2/file_in_tree')

        call s:helper.new_directory('removed_dir')
        call s:helper.new_file('removed_dir/file')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.remove_one()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y', 'y')

        call s:helper.sync_execute('')

        let first_line = s:helper.search('removed_file1')
        let last_line = s:helper.search('removed_file2')
        let command = s:helper.sync_execute('remove', {'range': [first_line, last_line]})

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'removed_file1')
        call s:assert.not_contains(lines, 'removed_file2')

        call s:helper.search('removed_dir\/')
        call s:helper.sync_execute('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'removed_dir/')
    endfunction

    function! suite.remove_in_tree()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y')

        call s:helper.sync_execute('')

        call s:helper.search('tree\/')
        call s:helper.sync_execute('toggle_tree')
        call s:helper.search('file_in_tree1')
        call s:helper.sync_execute('toggle_selection')

        call s:helper.sync_execute('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, '  file_in_tree1')
        call s:assert.contains(lines, '  file_in_tree2')
    endfunction

    function! suite.remove_parent_and_child()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y')

        call s:helper.sync_execute('')

        call s:helper.search('tree\/')
        call s:helper.sync_execute('toggle_tree')
        call s:helper.search('tree2\/')
        call s:helper.sync_execute('toggle_tree')

        call s:helper.search('tree\/')
        call s:helper.sync_execute('toggle_selection')
        call s:helper.search('file_in_tree1')
        call s:helper.sync_execute('toggle_selection')
        call s:helper.search('file_in_tree$')
        call s:helper.sync_execute('toggle_selection')

        call s:helper.sync_execute('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, '  file_in_tree1')
        call s:assert.not_contains(lines, '  file_in_tree2')
        call s:assert.not_contains(lines, 'tree/')
        call s:assert.not_contains(lines, '  file_in_tree')
    endfunction

    function! suite.cancel_remove()
        cd ./test/plugin/_test_data
        call s:helper.set_input('')

        call s:helper.sync_execute('')

        call s:helper.search('removed_cancel_file')
        call s:helper.sync_execute('remove')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

    function! suite.no_remove()
        cd ./test/plugin/_test_data
        call s:helper.set_input('n')

        call s:helper.sync_execute('')

        call s:helper.search('removed_cancel_file')
        call s:helper.sync_execute('remove')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

endfunction

function! s:suite.__copy_cut_paste__() abort
    let suite = s:helper.sub_suite('copy_cut_paste')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('copy_file')
        call s:helper.new_file('cut_file')
        call s:helper.new_directory('paste')
        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')

        call s:helper.new_file_with_content('already', ['has contents'])

        call s:helper.new_directory('has_already')
        call s:helper.new_file('has_already/already')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.copy_and_paste()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        let messenger = s:helper.messenger()

        call s:helper.search('copy_file')
        call s:helper.sync_execute('cut')
        call s:helper.sync_execute('copy') " copy disables cut

        call s:assert.contains(messenger.msg, 'Copied')
        call s:assert.contains(messenger.msg, 'test/plugin/_test_data/copy_file')

        call s:helper.search('paste\/')
        call s:helper.sync_execute('child')

        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')

        call s:helper.search('copy_file')
        call s:helper.sync_execute('child')

        call s:assert.file_name('copy_file')

        wincmd p
        call s:helper.sync_execute('parent')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.cut_and_paste()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        let messenger = s:helper.messenger()

        call s:helper.search('cut_file')
        call s:helper.sync_execute('cut')

        call s:assert.contains(messenger.msg, 'Cut')
        call s:assert.contains(messenger.msg, 'test/plugin/_test_data/cut_file')

        call s:helper.search('paste\/')
        call s:helper.sync_execute('child')

        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'cut_file')

        call s:helper.search('cut_file')
        call s:helper.sync_execute('child')

        call s:assert.file_name('cut_file')

        wincmd p
        call s:helper.sync_execute('parent')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'cut_file')
    endfunction

    function! suite.paste_in_tree()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        call s:helper.search('copy_file')
        call s:helper.sync_execute('cut')
        call s:helper.sync_execute('copy') " copy disables cut

        call s:helper.search('tree/')
        call s:helper.sync_execute('toggle_tree')

        call s:helper.search('file_in_tree')
        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  copy_file')
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.copy_and_renamed_paste()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        call s:helper.search('copy_file')
        call s:helper.sync_execute('copy')

        call s:helper.set_input('r', 'renamed_copy_file')
        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')
        call s:assert.contains(lines, 'renamed_copy_file')
    endfunction

    function! suite.cancel_renamed_paste()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')
        let test_lines = s:helper.lines()

        call s:helper.search('copy_file')
        call s:helper.sync_execute('copy')

        call s:helper.set_input('r', '')
        call s:helper.sync_execute('paste')

        call s:assert.lines(test_lines)
    endfunction

    function! suite.cancel_paste_on_already_exists()
        call s:helper.sync_execute('go -path=test/plugin/_test_data/has_already')

        call s:helper.search('already')
        call s:helper.sync_execute('copy')
        call s:helper.sync_execute('parent')

        call s:helper.set_input('n')
        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'already')
        call s:assert.file_not_empty('already')
    endfunction

    function! suite.force_paste_on_already_exists()
        call s:helper.sync_execute('go -path=test/plugin/_test_data/has_already')

        call s:helper.search('already')
        call s:helper.sync_execute('copy')
        call s:helper.sync_execute('parent')

        call s:helper.set_input('f')
        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'already')
        call s:assert.file_empty('already')
    endfunction

    function! suite.share_clipboard()
        let cwd = getcwd()

        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        call s:helper.search('copy_file')
        call s:helper.sync_execute('copy')

        call s:helper.sync_execute('-create')
        call s:helper.search('paste\/')
        call s:helper.sync_execute('child')

        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.working_dir(cwd . '/test/plugin/_test_data/paste')
        call s:assert.contains(lines, 'copy_file')

        wincmd p
        let lines = s:helper.lines()
        call s:assert.working_dir(cwd . '/test/plugin/_test_data')
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.directory_copy()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        call s:helper.search('tree\/')
        call s:helper.sync_execute('copy')

        call s:helper.search('paste\/')
        call s:helper.sync_execute('child')

        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'tree/')
    endfunction

    function! suite.directory_cut()
        call s:helper.sync_execute('go -path=test/plugin/_test_data')

        call s:helper.search('tree\/')
        call s:helper.sync_execute('cut')

        call s:helper.search('paste\/')
        call s:helper.sync_execute('child')

        call s:helper.sync_execute('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'tree/')

        call s:helper.sync_execute('parent')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'tree/')
    endfunction

endfunction

function! s:suite.__rename__() abort
    let suite = s:helper.sub_suite('rename')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('rename_file')

        call s:helper.new_file_with_content('already', ['has contents'])

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.rename_one()
        cd ./test/plugin/_test_data

        call s:helper.set_input('renamed_file')

        call s:helper.sync_execute('')

        call s:helper.search('rename_file')
        call s:helper.sync_execute('rename')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')
        call s:assert.not_contains(lines, 'rename_file')
    endfunction

    function! suite.rename_in_tree()
        cd ./test/plugin/_test_data

        call s:helper.set_input('renamed_file')

        call s:helper.sync_execute('')

        call s:helper.search('tree/')
        call s:helper.sync_execute('toggle_tree')

        call s:helper.search('file_in_tree')
        call s:helper.sync_execute('rename')

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  renamed_file')
        call s:assert.not_contains(lines, '  rename_file')
    endfunction

    function! suite.rename_already_exists()
        cd ./test/plugin/_test_data

        call s:helper.sync_execute('')

        call s:helper.search('rename_file')
        call s:helper.set_input('already')
        call s:helper.sync_execute('rename')

        call s:assert.file_not_empty('already')
    endfunction

    function! suite.multiple_rename_one()
        cd ./test/plugin/_test_data

        call s:helper.sync_execute('')

        call s:helper.search('rename_file')
        call s:helper.sync_execute('multiple_rename')
        call s:assert.modified(v:false)
        call s:assert.line_number(2)

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'rename_file')

        call setbufline('%', 2, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.modified(v:false)
        call s:assert.window_count(3)

        quit
        call s:helper.sync_execute('')
        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')
        call s:assert.not_contains(lines, 'rename_file')
    endfunction

    function! suite.multiple_rename_already_exists()
        cd ./test/plugin/_test_data

        call s:helper.sync_execute('')

        call s:helper.search('rename_file')
        call s:helper.sync_execute('toggle_selection')
        call s:helper.search('already')
        call s:helper.sync_execute('toggle_selection')
        call s:helper.sync_execute('multiple_rename')

        call setbufline('%', 2, 'renamed_file')
        call setbufline('%', 3, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.window_count(3)
        call s:assert.modified(v:true)
        q!
    endfunction

    function! suite.multiple_rename_twice()
        cd ./test/plugin/_test_data

        call s:helper.sync_execute('')

        call s:helper.search('rename_file')
        call s:helper.sync_execute('multiple_rename')

        call setbufline('%', 2, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.modified(v:false)
        call s:assert.window_count(3)

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')

        call s:helper.search('renamed_file')
        call setbufline('%', 2, 'twice_renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)
    endfunction

endfunction

function! s:suite.go_error()
    call s:helper.sync_execute('')

    let f = {'called': ''}
    function! f.echo(message) abort
        call kiview#logger#new('output').log(': ' . a:message)
        let self.called = a:message
    endfunction
    call kiview#messenger#set_func({ msg -> f.echo(msg) })

    call s:helper.sync_execute('go -path=./not_found')
    call s:assert.not_empty(f.called)
endfunction

function! s:suite.unknown_command()
    let f = {'called': ''}
    function! f.echo(message) abort
        call kiview#logger#new('output').log(': ' . a:message)
        let self.called = a:message
    endfunction
    call kiview#messenger#set_func({ msg -> f.echo(msg) })

    call s:helper.sync_execute('invalid_command_name')
    call s:assert.not_empty(f.called)
endfunction

function! s:suite.toggle_tree()
    call s:helper.sync_execute('')

    call s:helper.search('autoload/')

    let lines = s:helper.lines()
    call s:helper.sync_execute('toggle_tree')
    call s:helper.sync_execute('toggle_tree')
    call s:assert.lines(lines)

    call s:helper.sync_execute('toggle_tree')

    call s:helper.search('kiview.vim')
    call s:helper.sync_execute('child')

    call s:assert.file_name('kiview.vim')
endfunction

function! s:suite.cannot_toggle_parent_node()
    call s:helper.sync_execute('')

    let lines = s:helper.lines()

    normal! gg
    call s:helper.sync_execute('toggle_tree')

    call s:assert.lines(lines)
endfunction

function! s:suite.toggle_multi_trees()
    call s:helper.sync_execute('')

    call s:helper.search('autoload\/')
    call s:helper.sync_execute('toggle_tree')

    call s:helper.search('plugin\/')
    call s:helper.sync_execute('toggle_tree')

    call s:helper.search('kiview\.vim')
    call s:helper.sync_execute('child')

    call s:assert.path('plugin/kiview.vim')
endfunction

function! s:suite.toggle_parent_and_child()
    call s:helper.sync_execute('')

    call s:helper.search('autoload\/')
    call s:helper.sync_execute('toggle_selection')
    call s:helper.sync_execute('toggle_tree')

    call s:helper.search('kiview\/')
    call s:helper.sync_execute('toggle_selection')

    call s:helper.sync_execute('toggle_tree')
endfunction

function! s:suite.__toggle_deep__() abort
    let suite = s:helper.sub_suite('toggle_deep')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_directory('depth0')
        call s:helper.new_directory('depth0/depth1')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.toggle_last_dir()
        call s:helper.sync_execute('go -path=./test/plugin/_test_data/depth0')

        let lines = s:helper.lines()

        call s:helper.search('depth1\/')
        call s:helper.sync_execute('toggle_tree')
        call s:helper.sync_execute('toggle_tree')

        call s:assert.lines(lines)
    endfunction

    function! suite.toggle_single_dir()
        call s:helper.sync_execute('go -path=./test/plugin/_test_data')

        call s:helper.search('depth0\/')
        call s:helper.sync_execute('toggle_tree')

        let lines = s:helper.lines()
        call s:helper.search('depth1\/')
        call s:helper.sync_execute('toggle_tree')
        call s:helper.sync_execute('toggle_tree')

        call s:assert.lines(lines)
    endfunction

endfunction

function! s:suite.open_root()
    call s:helper.sync_execute('go -path=/')
    let lines = s:helper.lines()

    normal! 2j
    let line_number = line('.')

    call s:helper.sync_execute('parent')

    call s:assert.lines(lines)
    call s:assert.line_number(line_number)
endfunction

function! s:suite.toggle_selection()
    call s:helper.sync_execute('go -path=src')

    call s:helper.search('\.gitignore')
    call s:helper.sync_execute('toggle_selection')

    call s:helper.search('Cargo\.toml')
    call s:helper.sync_execute('toggle_selection')

    call s:helper.sync_execute('child -layout=tab')

    call s:assert.tab_count(3)
    call s:assert.file_name('Cargo.toml')
endfunction

function! s:suite.toggle_all_selection()
    call s:helper.sync_execute('go -path=src')

    call s:helper.search('src\/')
    call s:helper.sync_execute('toggle_selection')
    call s:helper.search('target\/')
    call s:helper.sync_execute('toggle_selection')

    call s:helper.sync_execute('toggle_all_selection')

    call s:helper.sync_execute('child -layout=tab')

    call s:assert.tab_count(5)
    call s:assert.file_name('Makefile')

    call s:helper.sync_execute('parent')
    call s:helper.sync_execute('toggle_all_selection')
endfunction

function! s:suite.vertical_rightbelow()
    call s:helper.sync_execute('-split=vertical:rightbelow')

    call s:assert.filetype('kiview')

    wincmd h
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_horizontal_leftabove()
    call s:helper.sync_execute('-split=horizontal:leftabove')

    call s:assert.filetype('kiview')

    wincmd j
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_horizontal_rightbelow()
    call s:helper.sync_execute('-split=horizontal:rightbelow')

    call s:assert.filetype('kiview')

    wincmd k
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_tab()
    call s:helper.sync_execute('-split=tab')

    call s:assert.filetype('kiview')
    call s:assert.window_count(1)
    call s:assert.tab_count(2)
endfunction

function! s:suite.no_split()
    call s:helper.sync_execute('-split=no')

    call s:assert.filetype('kiview')
    call s:assert.window_count(1)
endfunction

function! s:suite.clear_selection_on_error()
    call s:helper.sync_execute('go -path=./src')

    call s:helper.search('\.gitignore')
    call s:helper.sync_execute('toggle_selection')

    call s:helper.sync_execute('go -path=./not_found')

    call s:helper.search('Makefile')
    call s:helper.sync_execute('child')

    call s:assert.file_name('Makefile')
endfunction

function! s:suite.open_same_path()
    call s:helper.sync_execute('')

    let line_number = line('.')
    call s:helper.sync_execute('')

    call s:assert.line_number(line_number)

    normal! 2j
    let line_number = line('.')
    call s:helper.sync_execute('')

    call s:assert.line_number(line_number)
endfunction

function! s:suite.tab_open_group_node()
    let cwd = getcwd()

    call s:helper.sync_execute('')
    call s:helper.search('test\/')
    call s:helper.sync_execute('toggle_selection')
    call s:helper.search('Makefile')
    call s:helper.sync_execute('toggle_selection')
    call s:helper.sync_execute('child -layout=tab')

    call s:assert.tab_count(3)

    call s:assert.file_name('Makefile')

    tabprevious
    call s:assert.filetype('kiview')
    call s:assert.working_dir(cwd . '/test')

    call s:helper.search('plugin\/')
    call s:helper.sync_execute('child')

    call s:assert.working_dir(cwd . '/test/plugin')

    tabprevious
    call s:assert.working_dir(cwd)
endfunction

function! s:suite.history_back()
    call s:helper.sync_execute('')
    let working_dir = getcwd()

    call s:helper.search('src\/')
    call s:helper.sync_execute('child')
    call s:helper.search('target\/')
    call s:helper.sync_execute('child')

    call s:helper.sync_execute('back')
    call s:assert.working_dir(working_dir . '/src')

    call s:helper.search('src\/')
    call s:helper.sync_execute('child')
    call s:helper.search('command\/')
    call s:helper.sync_execute('child')

    call s:helper.sync_execute('back')
    call s:assert.working_dir(working_dir . '/src/src')

    call s:helper.sync_execute('back')
    call s:assert.working_dir(working_dir . '/src')

    call s:helper.sync_execute('back')
    call s:assert.working_dir(working_dir)

    call s:helper.sync_execute('back')
    call s:assert.working_dir(working_dir)
endfunction
