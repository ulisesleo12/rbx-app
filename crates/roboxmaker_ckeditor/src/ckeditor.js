// import React from '../../static/ckeditor.react';
// import ReactDOM from '../../static/react-dom.development';

export function render_ckeditor(node, content, userId, uploadURL, onData) {
    let element = React.createElement(CKEditor, {
        editor: ClassicEditor,
        data: content,
        config: {
            link: {
                decorators: {
                    isExternal: {
                        mode: 'automatic',
                        callback: url => !(url.startsWith('https://app.roboxmaker.network') || url.startsWith('https://app.roboxmaker.com')),
                        attributes: {
                            target: '_blank',
                            rel: 'noopener noreferrer'
                        }
                    },
                }
            },
            mediaEmbed: {
                previewsInData: true,
            },
            mention: {
                feeds: [
                    {
                        marker: '@',
                        feed: ['@annonymous'],
                        minimumCharacters: 1
                    }
                ]
            },
            simpleUpload: {
                uploadUrl: uploadURL,
                headers: {
                    'aker-user-id': userId,
                },
                fileTypes: [
                    '.pdf',
                    '.doc',
                    '.docx',
                    '.xls',
                    '.xlsx'
                ]
            },
            htmlEmbed: {
                showPreviews: true,
            },
        },
        onInit: (editor) => {
            console.log('event: onInit', { editor });
        },
        onChange: (event, editor) => {
            console.log('event: onChange', { event, editor });
            onData(editor.getData());
        },
        onBlur: (event, editor) => {
            console.log('event: onBlur', { event, editor });
        },
        onFocus: (event, editor) => {
            console.log('event: onFocus', { event, editor });
        }
    });
    ReactDOM.render(element, node);
}
