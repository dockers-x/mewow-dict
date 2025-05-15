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
            toggleLogo();
        },
        error: function(xhr, status, error) {
            $('#mdx-resp').html('查询出错：' + error).show();
            toggleLogo();
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
        toggleLogo();
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
                // 优先从 .hw_txt.gfont 提取词头
                let tempDiv = document.createElement('div');
                tempDiv.innerHTML = data;
                let word = '';
                let hw = tempDiv.querySelector('.hw_txt.gfont');
                if (hw) {
                    // 去除所有子元素后的纯文本（如 <sup>）
                    let nodes = Array.from(hw.childNodes).filter(n => n.nodeType === 3); // 只取文本节点
                    let txt = nodes.map(n => n.textContent).join('').trim();
                    let match = txt.match(/[a-zA-Z]+/);
                    if (match) word = match[0];
                }
                if (!word) {
                    // 兼容旧逻辑
                    let headword = tempDiv.querySelector('b, strong, span, .headword, .hw, .entry .word, .top-g .word');
                    if (headword) {
                        let match = headword.textContent.trim().match(/^[a-zA-Z]+$/);
                        if (match) {
                            word = match[0];
                        } else {
                            let match2 = headword.textContent.trim().match(/[a-zA-Z]+/);
                            if (match2) word = match2[0];
                        }
                    } else {
                        let match = data.match(/[a-zA-Z]+/);
                        if (match) word = match[0];
                    }
                }
                if (word) $('#word').val(word);
            } else {
                $('#mdx-resp').hide();
            }
            toggleLogo(); // 查询后立即判断logo显示
        },
        error: function(xhr, status, error) {
            $('#mdx-resp').html('查询出错：' + error).show();
            toggleLogo();
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

function toggleLogo() {
    const input = document.getElementById('word');
    const logo = document.getElementById('logo-title');
    const result = document.getElementById('mdx-resp');
    const resultContainer = document.querySelector('.result-container');
    // 判断输入框有内容且结果区不是初始提示或未找到时隐藏logo，否则显示
    if (
        input.value.trim() &&
        result &&
        result.innerText.trim() !== '' &&
        !result.innerText.includes('Ctrl + L 开始搜索') &&
        !result.innerText.includes('未找到相关释义')
    ) {
        logo.style.display = 'none';
        if (resultContainer) resultContainer.classList.remove('hidden');
    } else {
        logo.style.display = '';
        // 输入框为空时收起结果区并清空内容
        if (input.value.trim() === '' && resultContainer) {
            resultContainer.classList.add('hidden');
            if (result) result.innerHTML = '';
        } else if (resultContainer) {
            resultContainer.classList.remove('hidden');
        }
    }
}

document.addEventListener('DOMContentLoaded', function() {
    const input = document.getElementById('word');
    if (input) {
        input.addEventListener('input', toggleLogo);
    }
    // 监听搜索结果变化（假设有ajax渲染结果时调用）
    const observer = new MutationObserver(toggleLogo);
    const result = document.getElementById('mdx-resp');
    if (result) {
        observer.observe(result, { childList: true, subtree: true, characterData: true });
    }
    toggleLogo();
});