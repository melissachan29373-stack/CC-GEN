// ================================================================
// CC-GEN v2.0 — Theme Manager
// ================================================================

const ThemeManager = {
    STORAGE_KEY: 'ccgen-theme',

    init() {
        const saved = localStorage.getItem(this.STORAGE_KEY);
        if (saved) {
            this.set(saved);
        } else {
            // System preference
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            this.set(prefersDark ? 'dark' : 'light');
        }

        // Bind toggle button click
        const toggleBtn = document.getElementById('theme-toggle');
        if (toggleBtn) {
            toggleBtn.addEventListener('click', () => this.toggle());
        }

        // Listen for system changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
            if (!localStorage.getItem(this.STORAGE_KEY)) {
                this.set(e.matches ? 'dark' : 'light');
            }
        });
    },

    get() {
        return document.documentElement.getAttribute('data-theme') || 'dark';
    },

    set(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        localStorage.setItem(this.STORAGE_KEY, theme);
        this.updateToggleButton();
    },

    toggle() {
        this.set(this.get() === 'dark' ? 'light' : 'dark');
    },

    updateToggleButton() {
        const btn = document.getElementById('theme-toggle');
        if (btn) {
            btn.textContent = this.get() === 'dark' ? '☀️' : '🌙';
            btn.title = this.get() === 'dark' ? 'Switch to light mode' : 'Switch to dark mode';
        }
    }
};
