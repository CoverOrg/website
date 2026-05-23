// Fee calculator — 5% of (product + delivery), min Rs. 100
const amtInput = document.getElementById("amountInput");
const deliveryInput = document.getElementById("deliveryInput");
const amtRange = document.getElementById("amountRange");
const amtDisplay = document.getElementById("amountDisplay");
const deliveryDisplay = document.getElementById("deliveryDisplay");
const productBreakdown = document.getElementById("productBreakdown");
const deliveryBreakdown = document.getElementById("deliveryBreakdown");
const feeDisplay = document.getElementById("feeDisplay");
const totalDisplay = document.getElementById("totalDisplay");
const sellerDisplay = document.getElementById("sellerDisplay");
const minBadge = document.getElementById("minBadge");

function fmt(n) {
  return "Rs. " + Math.round(n).toLocaleString("en-PK");
}
function calc() {
  const amt = Math.max(0, parseFloat(amtInput.value) || 0);
  const del = Math.max(0, parseFloat(deliveryInput.value) || 0);
  const base = amt + del;
  const rawFee = base * 0.05;
  const fee = Math.max(100, rawFee);
  const isMin = rawFee < 100 && base > 0;
  amtDisplay.textContent = fmt(amt);
  deliveryDisplay.textContent = fmt(del);
  productBreakdown.textContent = fmt(amt);
  deliveryBreakdown.textContent = fmt(del);
  feeDisplay.textContent = fmt(fee);
  totalDisplay.textContent = fmt(amt + del + fee);
  sellerDisplay.textContent = fmt(amt + del);
  minBadge.style.display = isMin ? "block" : "none";
  amtRange.value = Math.min(amt, 200000);
}
amtInput.addEventListener("input", calc);
deliveryInput.addEventListener("input", calc);
amtRange.addEventListener("input", () => {
  amtInput.value = amtRange.value;
  calc();
});
calc();
