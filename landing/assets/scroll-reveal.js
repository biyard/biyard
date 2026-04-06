(function initReveal() {
    function setupObserver() {
        var els = document.querySelectorAll('.reveal-bounce, .reveal-fade, .reveal-type');
        if (els.length === 0) {
            setTimeout(setupObserver, 300);
            return;
        }
        var io = new IntersectionObserver(function(entries) {
            entries.forEach(function(entry) {
                if (entry.isIntersecting) {
                    var el = entry.target;
                    if (el.classList.contains('reveal-bounce')) {
                        el.classList.remove('scroll-hidden');
                        el.classList.add('scroll-bounce');
                    } else if (el.classList.contains('reveal-fade')) {
                        el.classList.remove('scroll-hidden');
                        el.classList.add('scroll-fade');
                    } else if (el.classList.contains('reveal-type')) {
                        el.classList.add('scroll-type');
                    }
                    io.unobserve(el);
                }
            });
        }, { threshold: 0.1, rootMargin: '0px 0px -50px 0px' });

        els.forEach(function(el) {
            if (el.classList.contains('reveal-bounce') || el.classList.contains('reveal-fade')) {
                el.classList.add('scroll-hidden');
            }
            io.observe(el);
        });
    }
    if (document.readyState === 'complete') {
        setTimeout(setupObserver, 800);
    } else {
        window.addEventListener('load', function() { setTimeout(setupObserver, 800); });
    }
    // Also retry on any DOM changes (SPA navigation)
    var mo = new MutationObserver(function() {
        var fresh = document.querySelectorAll('.reveal-bounce:not(.scroll-bounce):not(.scroll-hidden), .reveal-fade:not(.scroll-fade):not(.scroll-hidden)');
        if (fresh.length > 0) { setupObserver(); }
    });
    mo.observe(document.body || document.documentElement, { childList: true, subtree: true });
})();
