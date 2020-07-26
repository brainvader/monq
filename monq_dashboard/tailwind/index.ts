import "./styles.css";
import renderMathInElement from "katex/dist/contrib/auto-render";
import { highlightBlock } from 'highlight.js';

(async () => {
    // Note: files in `./pkg/` will be created on the first build.
    await import("../pkg/package");

    renderMathInElement(document.body);
    document.querySelectorAll('pre code').forEach((block) => {
        highlightBlock(block);
    });
})();
