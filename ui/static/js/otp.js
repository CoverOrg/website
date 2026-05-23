document.querySelectorAll(".otp-digit").forEach((input, i, inputs) => {
  input.addEventListener("input", () => {
    const val = input.value.replace(/\D/g, "");
    input.value = val.slice(-1);
    if (val && inputs[i + 1]) inputs[i + 1].focus();
  });
  input.addEventListener("keydown", (e) => {
    if (e.key === "Backspace" && !input.value && inputs[i - 1]) {
      inputs[i - 1].focus();
    }
  });
  input.addEventListener("paste", (e) => {
    e.preventDefault();
    const digits = e.clipboardData
      .getData("text")
      .replace(/\D/g, "")
      .slice(0, 6);
    digits.split("").forEach((d, j) => {
      if (inputs[i + j]) inputs[i + j].value = d;
    });
    const next = inputs[i + digits.length];
    if (next) next.focus();
    else inputs[5].focus();
  });
});
