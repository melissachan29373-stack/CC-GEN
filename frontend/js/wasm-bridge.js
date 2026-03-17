// ================================================================
// CC-GEN v2.0 — WASM Bridge
// ================================================================

const WasmBridge = {
    module: null,
    ready: false,

    async init() {
        try {
            const wasm = await import('../pkg/ccgen.js');
            await wasm.default();
            wasm.init();
            this.module = wasm;
            this.ready = true;
            console.log('WASM module initialized');
            return true;
        } catch (err) {
            console.error('WASM initialization failed:', err);
            this.ready = false;
            return false;
        }
    },

    generate(binPattern, count, includeExpiry, includeCvv, format, minYears, maxYears, unique, cardLength) {
        if (!this.ready) return { error: 'WASM not initialized' };
        try {
            const json = this.module.generate(
                binPattern, count, includeExpiry, includeCvv,
                format, minYears, maxYears, unique, cardLength
            );
            return JSON.parse(json);
        } catch (err) {
            return { error: err.message };
        }
    },

    validateCard(cardNumber) {
        if (!this.ready) return { error: 'WASM not initialized' };
        try {
            const json = this.module.validate_card(cardNumber);
            return JSON.parse(json);
        } catch (err) {
            return { error: err.message };
        }
    },

    lookupBin(bin) {
        if (!this.ready) return { error: 'WASM not initialized' };
        try {
            const json = this.module.lookup_bin(bin);
            return JSON.parse(json);
        } catch (err) {
            return { error: err.message };
        }
    },

    detectBrand(number) {
        if (!this.ready) return { brand: null };
        try {
            const json = this.module.detect_brand(number);
            return JSON.parse(json);
        } catch (err) {
            return { brand: null };
        }
    },

    getBrands() {
        if (!this.ready) return [];
        try {
            return JSON.parse(this.module.get_brands());
        } catch {
            return [];
        }
    },

    getDefaultBin(brandCode) {
        if (!this.ready) return '';
        try {
            return this.module.get_default_bin(brandCode);
        } catch {
            return '';
        }
    }
};

export default WasmBridge;
