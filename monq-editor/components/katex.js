import { useEffect, useRef } from 'react'

import katex from 'katex';

function Katex({ source }) {
    const rawElement = useRef(null);

    useEffect(() => {
        katex.render(source, rawElement.current);
    });

    return (
        <div ref={rawElement}></div>
    );
}

export default Katex;