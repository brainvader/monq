import init from '/pkg/package.js';
import renderMathInElement from "https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/contrib/auto-render.mjs";

init('/pkg/package_bg.wasm').then(() => {
    renderMathInElement(document.body);
    document.querySelectorAll('pre code').forEach((block) => {
        hljs.highlightBlock(block);
    });
});