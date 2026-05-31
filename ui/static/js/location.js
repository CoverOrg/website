document.addEventListener("DOMContentLoaded", function () {
  var btn = document.getElementById("loc-allow-btn");
  var deniedMsg = document.getElementById("loc-denied-msg");
  var partialMsg = document.getElementById("loc-partial-msg");
  var partialDetail = document.getElementById("loc-partial-detail");
  var checkbox = document.getElementById("loc-granted");

  if (!btn || !checkbox) return;

  /* ── Check if both permissions were already granted ── */
  function checkExistingPermissions() {
    if (!navigator.permissions) return;

    Promise.all([
      navigator.permissions.query({ name: "geolocation" }),
      navigator.permissions.query({ name: "camera" }),
    ])
      .then(function (results) {
        var locState = results[0].state;
        var camState = results[1].state;
        if (locState === "granted" && camState === "granted") {
          checkbox.checked = true;
        }
      })
      .catch(function () {
        /* permissions API not fully supported — do nothing,
               user will be prompted on button click */
      });
  }

  checkExistingPermissions();

  /* ── Button click: request both at once ── */
  btn.addEventListener("click", function () {
    btn.disabled = true;
    btn.innerHTML = [
      '<svg width="14" height="14" fill="none" stroke="currentColor"',
      '     stroke-width="2" viewBox="0 0 24 24" style="animation:',
      '     locSpin 1s linear infinite">',
      '  <path stroke-linecap="round" stroke-linejoin="round"',
      '        d="M12 2a10 10 0 110 20A10 10 0 0112 2z" opacity="0.3"/>',
      '  <path stroke-linecap="round" d="M12 2a10 10 0 018.66 5"/>',
      "</svg>",
      "Requesting…",
    ].join("");

    /* Hide any previous error messages */
    if (deniedMsg) deniedMsg.style.display = "none";
    if (partialMsg) partialMsg.style.display = "none";

    var locGranted = false;
    var camGranted = false;

    /* Request geolocation */
    var geoPromise = new Promise(function (resolve) {
      if (!navigator.geolocation) {
        locGranted = true; /* not available — don't block */
        resolve();
        return;
      }
      navigator.geolocation.getCurrentPosition(
        function () {
          locGranted = true;
          resolve();
        },
        function () {
          locGranted = false;
          resolve();
        },
        { timeout: 12000, maximumAge: 60000 },
      );
    });

    /* Request camera */
    var camPromise = new Promise(function (resolve) {
      if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
        camGranted = true; /* not available — don't block */
        resolve();
        return;
      }
      navigator.mediaDevices
        .getUserMedia({ video: { facingMode: { ideal: "environment" } } })
        .then(function (stream) {
          /* Stop the stream immediately — we only needed the grant */
          stream.getTracks().forEach(function (t) {
            t.stop();
          });
          camGranted = true;
          resolve();
        })
        .catch(function () {
          camGranted = false;
          resolve();
        });
    });

    /* Wait for both, then decide */
    Promise.all([geoPromise, camPromise]).then(function () {
      btn.disabled = false;

      if (locGranted && camGranted) {
        /* Both granted — dismiss gate */
        checkbox.checked = true;
        return;
      }

      if (!locGranted && !camGranted) {
        /* Both denied */
        if (deniedMsg) deniedMsg.style.display = "block";
        btn.innerHTML = "Try Again";
        return;
      }

      /* Partial grant — tell them exactly which one is missing */
      if (partialMsg && partialDetail) {
        var missing = !locGranted ? "Location" : "Camera";
        partialDetail.textContent =
          missing +
          " access was denied. Please enable it in " +
          "your browser settings, then tap Try Again.";
        partialMsg.style.display = "block";
      }
      btn.innerHTML = "Try Again";
    });
  });
});
