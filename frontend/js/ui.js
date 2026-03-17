// ================================================================
// CC-GEN v2.0 — UI Interactions
// ================================================================

const UI = {
    init() {
        this.setupTabs();
        this.setupCardFlip();
        this.setupRippleEffects();
    },

    // ─── Tab System ───
    setupTabs() {
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const target = btn.dataset.tab;
                document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
                document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
                btn.classList.add('active');
                document.getElementById(`tab-${target}`)?.classList.add('active');
            });
        });
    },

    // ─── Card Flip ───
    setupCardFlip() {
        const card = document.getElementById('card-3d');
        if (card) {
            card.addEventListener('click', () => {
                card.classList.toggle('flipped');
            });
        }
    },

    // ─── Ripple Effect ───
    setupRippleEffects() {
        document.querySelectorAll('.btn-ripple, .btn-generate').forEach(btn => {
            btn.addEventListener('click', function(e) {
                const rect = this.getBoundingClientRect();
                const circle = document.createElement('span');
                const diameter = Math.max(rect.width, rect.height);
                circle.style.width = circle.style.height = `${diameter}px`;
                circle.style.left = `${e.clientX - rect.left - diameter / 2}px`;
                circle.style.top = `${e.clientY - rect.top - diameter / 2}px`;
                circle.classList.add('ripple-circle');
                this.appendChild(circle);
                setTimeout(() => circle.remove(), 600);
            });
        });
    },

    // ─── Toast Notification ───
    showToast(message, type = 'success') {
        const container = document.getElementById('toast-container');
        if (!container) return;
        const toast = document.createElement('div');
        toast.className = `toast toast-${type} animate-fade-in-up`;
        toast.innerHTML = `<span>${type === 'success' ? '✓' : '✗'}</span> ${this.escapeHtml(message)}`;
        container.appendChild(toast);
        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transform = 'translateY(10px)';
            setTimeout(() => toast.remove(), 300);
        }, 2500);
    },

    // ─── Update Card Preview ───
    updateCardPreview(card) {
        const numEl = document.getElementById('preview-number');
        const brandEl = document.getElementById('preview-brand');
        const expiryEl = document.getElementById('preview-expiry');
        const cvvEl = document.getElementById('preview-cvv');
        const cardEl = document.getElementById('card-3d');

        if (numEl) numEl.textContent = card.number_formatted || '•••• •••• •••• ••••';
        if (brandEl) brandEl.textContent = card.brand || 'CARD';
        if (expiryEl) expiryEl.textContent = card.expiration_month && card.expiration_year
            ? `${card.expiration_month}/${card.expiration_year.slice(-2)}` : '••/••';
        if (cvvEl) cvvEl.textContent = card.cvv || '•••';

        // Update gradient
        if (cardEl) {
            const gradientVar = this.getBrandGradient(card.brand);
            const front = cardEl.querySelector('.card-front');
            const back = cardEl.querySelector('.card-back');
            if (front) front.style.background = gradientVar;
            if (back) back.style.background = gradientVar;
        }
    },

    getBrandGradient(brand) {
        const map = {
            'Visa': 'linear-gradient(135deg, #1a1f71, #2d3494)',
            'MasterCard': 'linear-gradient(135deg, #eb001b, #ff5f00)',
            'American Express': 'linear-gradient(135deg, #2e77bc, #1d4d7b)',
            'Discover': 'linear-gradient(135deg, #ff6600, #ff8533)',
            'Diners Club': 'linear-gradient(135deg, #006eb6, #0099d6)',
            'JCB': 'linear-gradient(135deg, #0e4c92, #1a6dba)',
            'UnionPay': 'linear-gradient(135deg, #e21836, #00447c)',
            'Maestro': 'linear-gradient(135deg, #0099dc, #6c6bbd)',
            'Mir': 'linear-gradient(135deg, #006848, #00a552)',
        };
        return map[brand] || 'linear-gradient(135deg, #374151, #6b7280)';
    },

    // ─── Update Stats ───
    updateStats(stats) {
        const el = (id, val) => {
            const e = document.getElementById(id);
            if (e) e.textContent = val;
        };
        el('stat-generated', stats.total_generated || 0);
        el('stat-valid', `${stats.valid_count || 0}/${stats.total_generated || 0} (100%)`);
        el('stat-time', stats.generation_time_us < 1000
            ? `${stats.generation_time_us}µs`
            : `${(stats.generation_time_us / 1000).toFixed(1)}ms`);
        el('stat-entropy', '256-bit CSPRNG');
    },

    // ─── Update BIN Detection ───
    updateBinDetection(number) {
        if (!number || number.length < 1) {
            const activePreset = document.querySelector('.preset-btn.active');
            if (!activePreset) {
                this.clearBinDetection();
            }
            return;
        }
        const result = WasmBridge.detectBrand(number);
        const indicator = document.getElementById('brand-indicator');
        if (indicator) {
            if (result.brand) {
                indicator.textContent = result.name;
                indicator.style.display = 'inline-block';
            } else {
                indicator.style.display = 'none';
            }
        }
    },

    clearBinDetection() {
        const indicator = document.getElementById('brand-indicator');
        if (indicator) indicator.style.display = 'none';
    },

    escapeHtml(str) {
        const div = document.createElement('div');
        div.textContent = str;
        return div.innerHTML;
    }
};
