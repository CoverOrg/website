const productInput = document.getElementById("amountInput");
const deliveryInput = document.getElementById("deliveryInput");
const rangeInput = document.getElementById("amountRange");

function calculate() {
  const productAmount = Math.max(0, parseInt(productInput.value) || 0);
  const deliveryAmount = Math.max(0, parseInt(deliveryInput.value) || 0);

  const totalBase = productAmount + deliveryAmount;
  const fee = Math.max(100, totalBase * 0.05);
  const seller = totalBase;
  const buyerPays = totalBase + fee;

  document.getElementById("amountDisplay").textContent =
    "₨. " + productAmount.toLocaleString("en-IN");
  document.getElementById("deliveryDisplay").textContent =
    "₨. " + deliveryAmount.toLocaleString("en-IN");
  document.getElementById("productBreakdown").textContent =
    "₨. " + productAmount.toLocaleString("en-IN");
  document.getElementById("deliveryBreakdown").textContent =
    "₨. " + deliveryAmount.toLocaleString("en-IN");
  document.getElementById("feeDisplay").textContent =
    "₨. " + fee.toLocaleString("en-IN");
  document.getElementById("sellerDisplay").textContent =
    "₨. " + seller.toLocaleString("en-IN");
  document.getElementById("totalDisplay").textContent =
    "₨. " + buyerPays.toLocaleString("en-IN");

  document.getElementById("minBadge").style.display =
    totalBase <= 2000 ? "flex" : "none";

  productInput.value = productAmount;
  deliveryInput.value = deliveryAmount;
  rangeInput.value = productAmount;
}

productInput.addEventListener("input", calculate);
deliveryInput.addEventListener("input", calculate);
rangeInput.addEventListener("input", (e) => {
  productInput.value = e.target.value;
  calculate();
});

// Initialize calculator on load
calculate();
