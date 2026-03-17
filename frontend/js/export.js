// ================================================================
// CC-GEN v2.0 — Export Utilities
// ================================================================

const ExportUtil = {
    copyToClipboard(text) {
        if (!text) return false;
        return navigator.clipboard.writeText(text).then(() => true).catch(() => {
            // Fallback
            const ta = document.createElement('textarea');
            ta.value = text;
            ta.style.cssText = 'position:fixed;left:-9999px';
            document.body.appendChild(ta);
            ta.select();
            const ok = document.execCommand('copy');
            document.body.removeChild(ta);
            return ok;
        });
    },

    download(content, filename) {
        const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    },

    getFileExtension(format) {
        const map = {
            'pipe': 'txt',
            'csv': 'csv',
            'tsv': 'tsv',
            'json': 'json',
            'json_array': 'json',
            'jsonarray': 'json',
            'xml': 'xml',
            'yaml': 'yaml',
            'sql': 'sql',
            'card_only': 'txt',
            'cardonly': 'txt',
            'formatted': 'txt',
            'stripe': 'txt',
            'stripetest': 'txt',
            'paypal': 'json',
            'paypalsandbox': 'json',
        };
        return map[format?.toLowerCase()] || 'txt';
    }
};
