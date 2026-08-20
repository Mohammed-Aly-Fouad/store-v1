/* ==========================================================================
   1. Dynamic Toast & Alert System
   ========================================================================== */

/**
 * Dismisses a toast element when the close button is clicked.
 * @param {HTMLElement} btn - The close button element inside the toast.
 */
function dismissToast(btn) {
  const toast = btn.closest('[data-toast]') || btn.closest('.alert-toast') || btn.closest('.toast');
  if (toast) {
    dismissToastElement(toast);
  }
}

/**
 * Handles smooth fade-out animation and removes the toast element from DOM.
 * @param {HTMLElement} toastEl - The target toast element.
 */
function dismissToastElement(toastEl) {
  if (!toastEl) return;
  
  // Apply CSS transition classes (compatible with CSS fade-out)
  toastEl.classList.add('fade-out', 'toast-hide', 'toast-hiding');
  
  setTimeout(() => {
    toastEl.remove();
  }, 400); // Matches CSS transition duration
}

/* ==========================================================================
   2. Global Event Listeners & Initializations
   ========================================================================== */

document.addEventListener('DOMContentLoaded', () => {
  // ------------------------------------------------------------------------
  // A) Clean URL Query String (e.g., ?ok=... or ?error=...)
  // Prevents re-triggering toast alerts on page refresh.
  // ------------------------------------------------------------------------
  if (window.location.search) {
    const cleanUrl = window.location.pathname;
    window.history.replaceState({}, document.title, cleanUrl);
  }

  // ------------------------------------------------------------------------
  // B) Auto-dismiss Flash Toasts with Progress Bar Support
  // ------------------------------------------------------------------------
  const toasts = document.querySelectorAll('[data-toast], .alert-toast, .toast, #flash-alert');

  toasts.forEach((toast) => {
    // Optional: Animate internal progress bar if present
    const barFill = toast.querySelector('.toast-bar-fill');
    if (barFill) {
      barFill.style.transition = 'width 5s linear';
      setTimeout(() => {
        barFill.style.width = '0%';
      }, 50);
    }

    // Dismiss toast after 5 seconds
    setTimeout(() => {
      dismissToastElement(toast);
    }, 5000);
  });
});

/* ==========================================================================
   3. Global Outside Click Handler (Dropdowns & Active Overlays)
   ========================================================================== */

window.addEventListener('click', (e) => {
  // Close any search results dropdown when clicking outside the container
  const searchContainer = document.querySelector('.search-container');
  const searchResults = document.getElementById('search-results');
  
  if (searchContainer && searchResults && !searchContainer.contains(e.target)) {
    searchResults.innerHTML = '';
  }
});