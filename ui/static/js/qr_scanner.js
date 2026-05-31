/* ══════════════════════════════════════════════════════════
   QR SCANNER — buyer delivery confirmation
══════════════════════════════════════════════════════════ */
var _qrStream = null;
var _qrAnimFrame = null;
var _qrScanned = false;

window.openQRScanner = function () {
  var overlay = document.getElementById("qr-scanner-overlay");
  overlay.style.display = "flex";
  document.body.style.overflow = "hidden";
  _qrScanned = false;

  // Reset UI
  document.getElementById("qr-success-overlay").style.display = "none";
  document.getElementById("qr-confirm-btn").style.display = "none";
  document.getElementById("qr-camera-error").style.display = "none";
  document.getElementById("qr-status").textContent = "Starting camera…";
  document.getElementById("qr-scan-line").style.display = "block";

  // Start camera
  if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
    showCameraError();
    return;
  }

  navigator.mediaDevices
    .getUserMedia({
      video: {
        facingMode: { ideal: "environment" },
        width: { ideal: 640 },
        height: { ideal: 640 },
      },
    })
    .then(function (stream) {
      _qrStream = stream;
      var video = document.getElementById("qr-video");
      video.srcObject = stream;
      video.play();
      document.getElementById("qr-status").textContent =
        "Point camera at the parcel QR code…";
      video.addEventListener("loadedmetadata", function () {
        startScanning(video);
      });
    })
    .catch(function (err) {
      console.warn("Camera error:", err);
      showCameraError();
    });
};

function showCameraError() {
  document.getElementById("qr-camera-error").style.display = "block";
  document.getElementById("qr-status").style.display = "none";
}

function startScanning(video) {
  var canvas = document.getElementById("qr-scan-canvas");
  var ctx = canvas.getContext("2d");

  function tick() {
    if (_qrScanned) return;
    if (video.readyState === video.HAVE_ENOUGH_DATA) {
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
      var imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

      if (window.jsQR) {
        var code = jsQR(imageData.data, imageData.width, imageData.height, {
          inversionAttempts: "dontInvert",
        });
        if (code) {
          onQRFound(code.data);
          return;
        }
      }
    }
    _qrAnimFrame = requestAnimationFrame(tick);
  }
  _qrAnimFrame = requestAnimationFrame(tick);
}

function onQRFound(data) {
  _qrScanned = true;
  cancelAnimationFrame(_qrAnimFrame);

  // Show success overlay
  document.getElementById("qr-scan-line").style.display = "none";
  var successOverlay = document.getElementById("qr-success-overlay");
  successOverlay.style.display = "flex";

  // Extract order ID from URL or use raw data
  var orderId = data.replace("https://cover.mom/verify/", "") || data;
  document.getElementById("qr-result-text").textContent = orderId;
  document.getElementById("qr-status").textContent =
    "QR code matched — tap below to confirm delivery.";
  document.getElementById("qr-confirm-btn").style.display = "block";
}

window.closeQRScanner = function () {
  cancelAnimationFrame(_qrAnimFrame);
  if (_qrStream) {
    _qrStream.getTracks().forEach(function (t) {
      t.stop();
    });
    _qrStream = null;
  }
  var video = document.getElementById("qr-video");
  video.srcObject = null;
  document.getElementById("qr-scanner-overlay").style.display = "none";
  document.body.style.overflow = "";
};

window.confirmQRDelivery = function () {
  closeQRScanner();
  // Navigate to order confirm view (same as manual confirm)
  var radio = document.getElementById("r-ord-confirm");
  var ordersRadio = document.getElementById("r-orders");
  if (ordersRadio) ordersRadio.checked = true;
  if (radio) radio.checked = true;
  alert(
    "Delivery confirmed via QR scan. PKR 250,500 will be released to the seller.",
  );
};
