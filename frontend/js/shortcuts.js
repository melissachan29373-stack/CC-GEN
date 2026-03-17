// ================================================================
// CC-GEN v2.0 — Keyboard Shortcuts
// ================================================================

const Shortcuts = {
    init() {
        document.addEventListener('keydown', (e) => {
            const mod = e.ctrlKey || e.metaKey;
            if (!mod) return;

            switch (e.key.toLowerCase()) {
                case 'enter':
                    e.preventDefault();
                    document.getElementById('btn-generate')?.click();
                    break;
                case 'c':
                    if (!window.getSelection()?.toString()) {
                        e.preventDefault();
                        document.getElementById('btn-copy')?.click();
                    }
                    break;
                case 's':
                    e.preventDefault();
                    document.getElementById('btn-download')?.click();
                    break;
                case 'k':
                    e.preventDefault();
                    document.getElementById('bin-input')?.focus();
                    break;
                case 'd':
                    e.preventDefault();
                    ThemeManager.toggle();
                    break;
                case 'l':
                    e.preventDefault();
                    document.getElementById('btn-clear')?.click();
                    break;
            }
        });
    }
};
