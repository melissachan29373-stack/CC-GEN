const express = require('express');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;
const FRONTEND_DIR = path.join(__dirname, 'frontend');

// WASM MIME type
express.static.mime.define({ 'application/wasm': ['wasm'] });

// Security headers
app.use((req, res, next) => {
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
    next();
});

// Serve static files with proper caching
app.use(express.static(FRONTEND_DIR, {
    maxAge: '1d',
    setHeaders: (res, filePath) => {
        if (filePath.endsWith('.wasm')) {
            res.setHeader('Content-Type', 'application/wasm');
            res.setHeader('Cache-Control', 'public, max-age=604800');
        }
        if (filePath.endsWith('.js')) {
            res.setHeader('Content-Type', 'application/javascript');
        }
    }
}));

// SPA fallback — serve 404.html for unknown routes
app.use((req, res) => {
    res.status(404).sendFile(path.join(FRONTEND_DIR, '404.html'));
});

app.listen(PORT, '0.0.0.0', () => {
    console.log(`CC-GEN server running on port ${PORT}`);
});
