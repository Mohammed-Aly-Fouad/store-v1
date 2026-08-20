/* ==========================================================================
   Brand Domain View Logic & Modal Controls
   ========================================================================== */

/**
 * Opens the Brand Creation / Edition Modal Dialog.
 */
function openModal() {
  const modal = document.getElementById('main-modal');
  if (modal) {
    modal.classList.add('active');
  }
}

/**
 * Closes the Brand Modal Dialog and handles URL state cleanup.
 * @param {Event} [e] - Optional click event object.
 */
function closeModal(e) {
  if (e && e.preventDefault) {
    e.preventDefault();
  }

  const modal = document.getElementById('main-modal');
  if (modal) {
    modal.classList.remove('active');
  }

  // If currently on an edit route (/web/brands/edit/:id), restore clean URL without page reload
  if (window.location.pathname.includes('/edit/')) {
    window.history.replaceState({}, document.title, '/web/brands');
  }
}

/* ==========================================================================
   Brand Page Specific Event Listeners
   ========================================================================== */

// Close brand modal when clicking directly on the dark overlay background
window.addEventListener('click', (e) => {
  const modal = document.getElementById('main-modal');
  if (e.target === modal) {
    closeModal(e);
  }
});