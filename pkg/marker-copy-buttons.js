
function addMarkerCopyButtons() {
    var textareas = document.querySelectorAll('textarea');
    textareas.forEach(function (textarea) {
        var button = document.createElement('button');
        button.className = 'output-copy-button';
        button.type = 'button';
        button.innerText = 'Copy';
        button.addEventListener('click', function () {
            if (navigator.clipboard) {
                navigator.clipboard.writeText(textarea.value).then(function() {
                    button.blur();
                    button.innerText = 'Copied!';
                    setTimeout(function () {
                        button.innerText = 'Copy';
                    }, 2000);
                }).catch(function(err) {
                    console.error('Could not copy text using Clipboard API: ', err);
                    fallbackCopy(textarea, button);
                });
            } else {
                fallbackCopy(textarea, button);
            }
        });
        textarea.parentNode.insertBefore(button, textarea);
    });
}

function fallbackCopy(textarea, button) {
    textarea.select();
    document.execCommand('copy');
    button.blur();
    button.innerText = 'Copied!';
    setTimeout(function () {
        button.innerText = 'Copy';
    }, 2000);
}

addMarkerCopyButtons();


