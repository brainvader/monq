import { useEffect, useRef } from 'react'

import hljs from 'highlight.js';

function HighlightJS({ codeString, language }) {
    const preElement = useRef(null);

    useEffect(() => {
        hljs.highlightBlock(preElement.current);
    });

    if (codeString) {
        return (
            <pre ref={preElement}>
                <code className={language}>
                    {codeString}
                </code>
            </pre>
        );
    }

    return (
        <pre>No Content</pre>
    );
}

export default HighlightJS;