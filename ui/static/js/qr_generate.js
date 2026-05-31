/* ══════════════════════════════════════════════════════════
   QR CODE GENERATION — ship page
══════════════════════════════════════════════════════════ */
(function () {
  var ORDER_ID = "COV-A3BX-7KPQ";
  var QR_URL = "https://cover.mom/verify/" + ORDER_ID;

  function generateShipQR() {
    var wrap = document.getElementById("ship-qr-wrap");
    if (!wrap || !window.QRCode) return;
    wrap.innerHTML = "";
    new QRCode(wrap, {
      text: QR_URL,
      width: 180,
      height: 180,
      colorDark: "#0a0a0a",
      colorLight: "#ffffff",
      correctLevel: QRCode.CorrectLevel.H,
    });
  }

  // Try immediately, retry after scripts load
  if (window.QRCode) {
    generateShipQR();
  } else {
    window.addEventListener("load", generateShipQR);
    // Fallback: poll briefly
    var attempts = 0;
    var poll = setInterval(function () {
      if (window.QRCode) {
        generateShipQR();
        clearInterval(poll);
      }
      if (++attempts > 20) clearInterval(poll);
    }, 300);
  }

  window.downloadShipQR = function () {
    var wrap = document.getElementById("ship-qr-wrap");
    if (!wrap) return;
    var canvas = wrap.querySelector("canvas");
    if (!canvas) return;
    var link = document.createElement("a");
    link.download = "Cover-QR-COV-A3BX-7KPQ.png";
    link.href = canvas.toDataURL("image/png");
    link.click();
  };

  window.printShipQR = function () {
    var wrap = document.getElementById("ship-qr-wrap");
    if (!wrap) return;
    var canvas = wrap.querySelector("canvas");
    if (!canvas) return;
    var img = canvas.toDataURL("image/png");
    var win = window.open("", "_blank");
    win.document.write(
      [
        "<!DOCTYPE html><html><head>",
        "<title>Cover Delivery QR — COV-A3BX-7KPQ</title>",
        "<style>",
        "body{display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;margin:0;font-family:monospace;background:#fff;gap:12px;}",
        "img{width:200px;height:200px;}",
        "p{font-size:12px;color:#444;margin:0;letter-spacing:0.08em;}",
        "small{font-size:10px;color:#999;text-align:center;}",
        "@media print{@page{margin:10mm;}}",
        "</style></head><body>",
        '<img src="' + img + '" />',
        "<p>COV-A3BX-7KPQ</p>",
        "<small>Scan to confirm delivery · cover.mom/verify</small>",
        '<small style="margin-top:8px;color:#ccc;">Affix to parcel sealed flap</small>',
        "<script>window.onload=function(){window.print();window.close();};<\/script>",
        "</body></html>",
      ].join(""),
    );
    win.document.close();
  };
})();
