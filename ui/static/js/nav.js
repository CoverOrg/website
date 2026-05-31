document.addEventListener("DOMContentLoaded", function () {
  /* ── User menu ── */
  var chip = document.querySelector(".user-chip");
  var mobileBtn = document.querySelector(".mobile-nav-item-user");
  var desktopMenu = document.querySelector(".user-menu");
  var mobileSheet = document.querySelector(".mobile-user-menu");
  var mobileBdrop = document.querySelector(".mobile-user-menu-backdrop");

  function isMobile() {
    return window.innerWidth < 768;
  }

  function openMenu() {
    if (isMobile()) {
      if (mobileSheet) mobileSheet.classList.add("is-open");
      if (mobileBdrop) mobileBdrop.classList.add("is-open");
    } else {
      if (desktopMenu) desktopMenu.classList.add("is-open");
      if (chip) chip.classList.add("active");
    }
  }

  function closeMenu() {
    if (desktopMenu) desktopMenu.classList.remove("is-open");
    if (mobileSheet) mobileSheet.classList.remove("is-open");
    if (mobileBdrop) mobileBdrop.classList.remove("is-open");
    if (chip) chip.classList.remove("active");
  }

  function menuIsOpen() {
    return (
      (desktopMenu && desktopMenu.classList.contains("is-open")) ||
      (mobileSheet && mobileSheet.classList.contains("is-open"))
    );
  }

  /* Desktop chip */
  if (chip) {
    chip.addEventListener("click", function (e) {
      e.stopPropagation();
      menuIsOpen() ? closeMenu() : openMenu();
    });
  }

  /* Mobile bottom-nav avatar */
  if (mobileBtn) {
    mobileBtn.addEventListener("click", function (e) {
      e.stopPropagation();
      menuIsOpen() ? closeMenu() : openMenu();
    });
  }

  /* Mobile backdrop tap closes sheet */
  if (mobileBdrop) {
    mobileBdrop.addEventListener("click", closeMenu);
  }

  /* Desktop: click outside closes popup */
  document.addEventListener("click", function (e) {
    if (
      !isMobile() &&
      desktopMenu &&
      desktopMenu.classList.contains("is-open")
    ) {
      if (!desktopMenu.contains(e.target) && chip && !chip.contains(e.target)) {
        closeMenu();
      }
    }
  });

  /* Stop clicks inside menus from bubbling to outside-click handler */
  if (desktopMenu)
    desktopMenu.addEventListener("click", function (e) {
      e.stopPropagation();
    });
  if (mobileSheet)
    mobileSheet.addEventListener("click", function (e) {
      e.stopPropagation();
    });
});
