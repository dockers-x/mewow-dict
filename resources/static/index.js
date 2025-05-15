// 光标默认可输入
$(document).ready(function (e) {
        $('#word').focus();
    }
);

// 查询mdx
function queryMdx(word) {
    $('#mdx-resp').html('查询中...');
    $.ajax({
        url: './query',
        type: 'POST',
        data: {'word': word},
        dataType: 'html',
        success: function (data) {
            if (data === 'not found') {
                $('#mdx-resp').html('未找到相关释义').show();
            } else if (data.startsWith('Error:')) {
                $('#mdx-resp').html(data).show();
            } else if (data !== '') {
                $('#mdx-resp').html(data).show();
            } else {
                $('#mdx-resp').hide();
            }
        },
        error: function(xhr, status, error) {
            $('#mdx-resp').html('查询出错：' + error).show();
        }
    });
}

function postQuery() {
    let word = $('#word').val().trim();
    if (!validInput(word)) {
        return;
    }
    queryMdx(word);
}

// 特殊字符不查询
function validInput(word) {
    return word
        && word !== '.'
        && word !== '#'
        && word !== '?'
        && word !== '/';
}

// 监听回车键
$(document).keydown(function (e) {
    if (e.keyCode === 13) {
        postQuery();
    }
});

// 监听牛津8解释页面的外部单词链接
$(document).on('click', 'a', function (e) {
    console.log($(this).attr('href'));
    let href = $(this).attr('href');// '/cool'
    if (href.startsWith('/') && !href.startsWith('/#')) {
        $('#word').val(href.slice(1)) // 'cool'
        postQuery();
        e.preventDefault()
    }
});

// 捕获ctrl+L快捷键
$(window).bind('keyup keydown', function (e) {
    if ((e.ctrlKey || e.metaKey)
        && String.fromCharCode(e.which).toLowerCase() === 'l') {
        e.preventDefault();
        $('#word').val('').focus();
        scrollTo(0, 0);
    }
});

// 试试手气按钮
$(document).on('click', '#lucky-btn', function (e) {
    $('#mdx-resp').html('查询中...');
    $.ajax({
        url: './lucky',
        type: 'GET',
        dataType: 'html',
        success: function (data) {
            if (data === 'not found') {
                $('#mdx-resp').html('未找到相关释义').show();
            } else if (data.startsWith('Error:')) {
                $('#mdx-resp').html(data).show();
            } else if (data !== '') {
                $('#mdx-resp').html(data).show();
            } else {
                $('#mdx-resp').hide();
            }
        },
        error: function(xhr, status, error) {
            $('#mdx-resp').html('查询出错：' + error).show();
        }
    });
});

// 不同词典返回html不一样，无法通用
// function parserWordFromResp(data) {
//     let el = document.createElement('html');
//     el.innerHTML = data;
//     let top_g = el.getElementsByClassName("top-g")[0]
//     if (top_g == null) {
//         console.log("top-g is null");
//         return "";
//     }
//
//     return top_g.firstElementChild.innerHTML.split('·').join('')
//
// }