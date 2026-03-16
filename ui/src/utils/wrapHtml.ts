/** Wrap raw HTML content in a styled document for iframe preview */
export function wrapHtmlDocument(body: string): string {
  return `<!DOCTYPE html>
<html><head><meta charset="utf-8"><style>
body { background:#0f172b; color:#e0e0e0; font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif; font-size:14px; line-height:1.6; padding:16px; margin:0; word-wrap:break-word; }
img { max-width:100%; height:auto; }
a { color:#3498db; cursor:pointer; }
table { border-collapse:collapse; }
th, td { padding:4px 12px; }
blockquote { border-left:3px solid #555; padding:8px 16px; margin:8px 0; background:#1a2744; }
hr { border:1px solid #555; margin:1em 0; }
h1,h2,h3 { margin:8px 0; }
p { margin:4px 0; }
details { margin:8px 0; }
summary { cursor:pointer; font-weight:bold; }
</style></head><body>${body}
<script>document.addEventListener('click',function(e){var l=e.target.closest('a');if(l&&l.href){e.preventDefault();window.parent.postMessage({type:'open-url',url:l.getAttribute('href')},'*')}});</script>
</body></html>`;
}
