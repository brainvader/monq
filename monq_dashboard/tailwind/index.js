import "core-js/stable";
import "regenerator-runtime/runtime";

import "./styles.css";

import init from '../pkg/package.js';
// import renderMathInElement from "h?ttps://cdn.jsdelivr.net/npm/katex@0.12.0/dist/contrib/auto-render.mjs";
import renderMathInElement from "katex";
import hljs from 'highlight.js';

init('../pkg/package_bg.wasm').then(() => {
    renderMathInElement(document.body);
    document.querySelectorAll('pre code').forEach((block) => {
        hljs.highlightBlock(block);
    });
});