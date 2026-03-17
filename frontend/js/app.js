// ================================================================
// CC-GEN v2.0 — Main Application
// ================================================================

import WasmBridge from './wasm-bridge.js';

// Expose WasmBridge globally so non-module scripts (ui.js) can use it
window.WasmBridge = WasmBridge;

const App = {
    history: [],
    MAX_HISTORY: 50,

    async init() {
        ThemeManager.init();

        const wasmReady = await WasmBridge.init();
        if (!wasmReady) {
            UI.showToast('WASM failed to load. Check console.', 'error');
            return;
        }

        UI.init();
        Shortcuts.init();
        this.bindEvents();
        this.loadHistory();

        UI.showToast('Engine ready — powered by Rust/WASM', 'success');
    },

    bindEvents() {
        // Generate button
        document.getElementById('btn-generate')?.addEventListener('click', () => this.generate());

        // Copy
        document.getElementById('btn-copy')?.addEventListener('click', () => this.copyOutput());

        // Download
        document.getElementById('btn-download')?.addEventListener('click', () => this.downloadOutput());

        // Clear
        document.getElementById('btn-clear')?.addEventListener('click', () => this.clearOutput());

        // BIN input change → brand detection
        document.getElementById('bin-input')?.addEventListener('input', (e) => {
            UI.updateBinDetection(e.target.value);
            // Deactivate preset
            document.querySelectorAll('.preset-btn').forEach(b => b.classList.remove('active'));
        });

        // Preset buttons
        document.querySelectorAll('.preset-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.preset-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                const binInput = document.getElementById('bin-input');
                const lengthInput = document.getElementById('card-length');
                if (binInput) binInput.value = btn.dataset.bin;
                if (lengthInput) lengthInput.value = btn.dataset.length || '';
                UI.updateBinDetection(btn.dataset.bin);
            });
        });

        // Quantity slider
        const slider = document.getElementById('quantity-slider');
        const display = document.getElementById('quantity-display');
        if (slider && display) {
            slider.addEventListener('input', () => {
                display.textContent = slider.value;
            });
        }

        // BIN Lookup
        document.getElementById('btn-lookup')?.addEventListener('click', () => this.lookupBin());
        document.getElementById('lookup-input')?.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') this.lookupBin();
        });

        // Validator
        document.getElementById('btn-validate')?.addEventListener('click', () => this.validateCard());
        document.getElementById('validate-input')?.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') this.validateCard();
        });

        // Clear history
        document.getElementById('btn-clear-history')?.addEventListener('click', () => this.clearHistory());
    },

    // ─── Generate Cards ───
    generate() {
        const binPattern = document.getElementById('bin-input')?.value || '4xxxxxxxxxxxxxxx';
        const count = parseInt(document.getElementById('quantity-slider')?.value || '10');
        const format = document.getElementById('format-select')?.value || 'pipe';
        const month = document.getElementById('exp-month')?.value || '';
        const year = document.getElementById('exp-year')?.value || '';
        const cvv = document.getElementById('exp-cvv')?.value || '';
        const cardLengthInput = document.getElementById('card-length')?.value;
        const cardLength = cardLengthInput ? parseInt(cardLengthInput) : -1;

        const includeExpiry = true;
        const includeCvv = true;

        const result = WasmBridge.generate(
            binPattern, count, includeExpiry, includeCvv,
            format, 1, 5, true, cardLength
        );

        if (result.error) {
            UI.showToast(`Error: ${result.error}`, 'error');
            return;
        }

        // Overwrite expiry/cvv if user specified
        if (month || year || cvv) {
            result.cards = result.cards.map(card => ({
                ...card,
                expiration_month: month || card.expiration_month,
                expiration_year: year || card.expiration_year,
                cvv: cvv || card.cvv,
            }));
            // Reformat if user specified fields
            if (month || year || cvv) {
                result.formatted_output = this.reformatOutput(result.cards, format);
            }
        }

        // Display output
        const outputEl = document.getElementById('output-area');
        if (outputEl) outputEl.value = result.formatted_output;

        // Update stats
        UI.updateStats(result.stats);

        // Update card preview with first card
        if (result.cards.length > 0) {
            UI.updateCardPreview(result.cards[0]);
        }

        // Add to history
        this.addToHistory(result.cards);

        UI.showToast(`Generated ${result.cards.length} cards`, 'success');
    },

    reformatOutput(cards, format) {
        switch (format.toLowerCase()) {
            case 'pipe':
                return cards.map(c => `${c.number}|${c.expiration_month}|${c.expiration_year}|${c.cvv}`).join('\n');
            case 'csv':
                return 'number,month,year,cvv,brand\n' +
                    cards.map(c => `${c.number},${c.expiration_month},${c.expiration_year},${c.cvv},${c.brand}`).join('\n');
            case 'json':
                return cards.map(c => JSON.stringify({
                    card: c.number, month: c.expiration_month,
                    year: c.expiration_year, cvv: c.cvv, brand: c.brand
                })).join('\n');
            case 'json_array': case 'jsonarray':
                return JSON.stringify(cards, null, 2);
            case 'card_only': case 'cardonly':
                return cards.map(c => c.number).join('\n');
            case 'formatted':
                return cards.map(c => `${c.number_formatted} | ${c.expiration_month}/${c.expiration_year} | ${c.cvv}`).join('\n');
            default:
                return cards.map(c => `${c.number}|${c.expiration_month}|${c.expiration_year}|${c.cvv}`).join('\n');
        }
    },

    // ─── Copy Output ───
    async copyOutput() {
        const output = document.getElementById('output-area')?.value;
        if (!output) {
            UI.showToast('Nothing to copy', 'error');
            return;
        }
        await ExportUtil.copyToClipboard(output);
        const btn = document.getElementById('btn-copy');
        if (btn) {
            btn.classList.add('copied');
            const origText = btn.innerHTML;
            btn.innerHTML = '<span>✓</span> Copied';
            setTimeout(() => {
                btn.classList.remove('copied');
                btn.innerHTML = origText;
            }, 2000);
        }
        UI.showToast('Copied to clipboard', 'success');
    },

    // ─── Download Output ───
    downloadOutput() {
        const output = document.getElementById('output-area')?.value;
        if (!output) {
            UI.showToast('Nothing to download', 'error');
            return;
        }
        const format = document.getElementById('format-select')?.value || 'pipe';
        const ext = ExportUtil.getFileExtension(format);
        ExportUtil.download(output, `ccgen-output.${ext}`);
        UI.showToast('File downloaded', 'success');
    },

    // ─── Clear Output ───
    clearOutput() {
        const outputEl = document.getElementById('output-area');
        if (outputEl) outputEl.value = '';
        UI.updateStats({ total_generated: 0, valid_count: 0, generation_time_us: 0 });
        UI.updateCardPreview({});
    },

    // ─── BIN Lookup ───
    lookupBin() {
        const bin = document.getElementById('lookup-input')?.value;
        if (!bin || bin.length < 6) {
            UI.showToast('Enter at least 6 digits', 'error');
            return;
        }
        const result = WasmBridge.lookupBin(bin);
        const container = document.getElementById('lookup-results');
        if (!container) return;

        if (result.error) {
            container.innerHTML = `<p style="color:var(--text-muted);text-align:center;padding:16px;">${UI.escapeHtml(result.error)}</p>`;
            return;
        }

        container.innerHTML = `
            <div class="lookup-grid">
                <div class="lookup-item">
                    <div class="lookup-item-label">Brand</div>
                    <div class="lookup-item-value">${UI.escapeHtml(result.brand || 'Unknown')}</div>
                </div>
                <div class="lookup-item">
                    <div class="lookup-item-label">Type</div>
                    <div class="lookup-item-value">${UI.escapeHtml(result.card_type || 'Unknown')}</div>
                </div>
                <div class="lookup-item">
                    <div class="lookup-item-label">Issuer</div>
                    <div class="lookup-item-value">${UI.escapeHtml(result.issuer_name || 'Unknown')}</div>
                </div>
                <div class="lookup-item">
                    <div class="lookup-item-label">Country</div>
                    <div class="lookup-item-value">${UI.escapeHtml(result.country_name || result.country_code || 'Unknown')}</div>
                </div>
            </div>
        `;
    },

    // ─── Card Validator ───
    validateCard() {
        const number = document.getElementById('validate-input')?.value;
        if (!number || number.replace(/\s/g, '').length < 8) {
            UI.showToast('Enter a valid card number', 'error');
            return;
        }
        const result = WasmBridge.validateCard(number);
        const container = document.getElementById('validation-results');
        if (!container) return;

        if (result.error) {
            container.innerHTML = `<p style="color:var(--accent-danger);">${UI.escapeHtml(result.error)}</p>`;
            return;
        }

        const checks = [
            { label: 'Luhn Algorithm', pass: result.luhn_valid },
            { label: 'Structure (ISO 7812)', pass: result.structure_valid },
            { label: 'BIN Range', pass: result.bin_range_valid },
            { label: 'Length', pass: result.length_valid },
            { label: 'Checksum Consistency', pass: result.checksum_consistent },
        ];

        const brand = result.card_brand
            ? `<div style="margin-bottom:12px;font-size:14px;font-weight:600;">Detected: ${UI.escapeHtml(result.card_brand)}</div>`
            : '';

        container.innerHTML = `
            ${brand}
            ${checks.map(c => `
                <div class="validation-check">
                    <div class="check-icon ${c.pass ? 'pass' : 'fail'}">${c.pass ? '✓' : '✗'}</div>
                    <div class="check-label">${c.label}</div>
                </div>
            `).join('')}
            <div class="confidence-bar">
                <div class="confidence-label">
                    <span>Confidence Score</span>
                    <span>${Math.round(result.confidence_score * 100)}%</span>
                </div>
                <div class="confidence-track">
                    <div class="confidence-fill" style="width:${result.confidence_score * 100}%"></div>
                </div>
            </div>
        `;
    },

    // ─── History ───
    addToHistory(cards) {
        const limit = Math.min(cards.length, 10);
        for (let i = 0; i < limit; i++) {
            this.history.unshift({
                number: cards[i].number,
                brand: cards[i].brand,
                formatted: cards[i].number_formatted,
            });
        }
        if (this.history.length > this.MAX_HISTORY) {
            this.history = this.history.slice(0, this.MAX_HISTORY);
        }
        this.saveHistory();
        this.renderHistory();
    },

    renderHistory() {
        const list = document.getElementById('history-list');
        if (!list) return;

        if (this.history.length === 0) {
            list.innerHTML = '<div class="history-empty">No cards generated yet</div>';
            return;
        }

        list.innerHTML = this.history.map(h => `
            <div class="history-item" data-number="${UI.escapeHtml(h.number)}">
                <span class="history-number">${UI.escapeHtml(h.formatted || h.number)}</span>
                <span class="history-brand">${UI.escapeHtml(h.brand)}</span>
            </div>
        `).join('');

        list.querySelectorAll('.history-item').forEach(item => {
            item.addEventListener('click', () => {
                ExportUtil.copyToClipboard(item.dataset.number);
                UI.showToast('Card number copied', 'success');
            });
        });
    },

    clearHistory() {
        this.history = [];
        this.saveHistory();
        this.renderHistory();
        UI.showToast('History cleared', 'success');
    },

    saveHistory() {
        try {
            localStorage.setItem('ccgen-history', JSON.stringify(this.history));
        } catch { /* ignore */ }
    },

    loadHistory() {
        try {
            const saved = localStorage.getItem('ccgen-history');
            if (saved) {
                this.history = JSON.parse(saved);
            }
        } catch { /* ignore */ }
        this.renderHistory();
    },
};

// ─── Bootstrap ───
document.addEventListener('DOMContentLoaded', () => App.init());
